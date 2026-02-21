use crate::memory::schema;
use crate::sessions::{Session, VerbosityLevel};
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::path::Path;

pub struct MemoryStore {
    conn: std::sync::Mutex<Connection>,
}

impl std::fmt::Debug for MemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryStore").field("conn", &"...").finish()
    }
}

impl MemoryStore {
    fn load_sqlite_vec_extension(conn: &Connection) -> Result<()> {
        unsafe {
            conn.load_extension_enable()?;

            // The sqlite-vec crate declares sqlite3_vec_init() with 0 arguments,
            // but the C function expects 3. We cast it to the correct signature.
            type Sqlite3VecInit = unsafe extern "C" fn(
                *mut rusqlite::ffi::sqlite3,
                *mut *mut std::os::raw::c_char,
                *const rusqlite::ffi::sqlite3_api_routines,
            ) -> std::os::raw::c_int;

            let init_fn: Sqlite3VecInit =
                std::mem::transmute(sqlite_vec::sqlite3_vec_init as *const ());

            // Load the extension - ignore return value as sqlite-vec doesn't return errors on init
            let _ = init_fn(conn.handle(), std::ptr::null_mut(), std::ptr::null());

            conn.load_extension_disable()?;
        }
        Ok(())
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::setup_conn(&conn)?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::setup_conn(&conn)?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    fn setup_conn(conn: &Connection) -> Result<()> {
        // SAFETY: This unsafe block is required to load the sqlite-vec extension.
        // The sqlite-vec crate provides a C-compatible function (sqlite3_vec_init)
        // that must be called with the correct FFI signature. The transmute is safe
        // because we cast from a known function pointer type to the correct FFI signature.
        // The extension is loaded from a trusted source (sqlite-vec crate) and
        // we immediately disable further extension loading after initialization.
        Self::load_sqlite_vec_extension(conn)?;

        schema::ensure_schema(conn)?;

        // Performance Tuning for Vector Search
        // WAL mode allows concurrent readers and better write performance
        conn.pragma_update(None, "journal_mode", "WAL")?;
        // Normal synchronous is safe enough for WAL and much faster
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        // Store temp tables in memory
        conn.pragma_update(None, "temp_store", "MEMORY")?;
        // Use memory mapping for faster read access (32GB limit)
        conn.pragma_update(None, "mmap_size", "30000000000")?;
        // Increase page cache size (e.g. -64000 = 64MB)
        conn.pragma_update(None, "cache_size", "-64000")?;

        Ok(())
    }

    pub fn ensure_vector_index(&self, dimensions: usize) -> Result<()> {
        schema::ensure_vector_table(&self.conn.lock().unwrap(), dimensions)
    }

    pub fn save_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }

    pub fn read_meta(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM meta WHERE key = ?1")?;
        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn build_fts_query(&self, raw: &str) -> Option<String> {
        let tokens: Vec<String> = raw
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|s| !s.is_empty())
            .map(|s| format!("\"{}\"", s.replace('"', "")))
            .collect();

        if tokens.is_empty() {
            None
        } else {
            Some(tokens.join(" AND "))
        }
    }

    pub fn delete_chunks_by_path(&self, path: &str, source: &str, model: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM chunks_vec WHERE id IN (SELECT id FROM chunks WHERE path = ?1 AND source = ?2 AND model = ?3)",
            [path, source, model],
        )?;
        conn.execute(
            "DELETE FROM chunks_fts WHERE path = ?1 AND source = ?2 AND model = ?3",
            [path, source, model],
        )?;
        conn.execute(
            "DELETE FROM chunks WHERE path = ?1 AND source = ?2",
            [path, source],
        )?;
        Ok(())
    }

    pub fn insert_chunk(
        &self,
        id: &str,
        path: &str,
        source: &str,
        start_line: i32,
        end_line: i32,
        hash: &str,
        model: &str,
        text: &str,
        embedding: &[f32],
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        let embedding_json = serde_json::to_string(&embedding).unwrap_or_default();
        let conn = self.conn.lock().unwrap();

        // 1. Insert/Update chunks table
        conn.execute(
            "INSERT INTO chunks (id, path, source, start_line, end_line, hash, model, text, embedding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
               hash=excluded.hash,
               model=excluded.model,
               text=excluded.text,
               embedding=excluded.embedding,
               updated_at=excluded.updated_at",
            rusqlite::params![id, path, source, start_line, end_line, hash, model, text, embedding_json, now],
        )?;

        // 2. Insert into FTS table
        conn.execute(
            "INSERT INTO chunks_fts (text, id, path, source, model, start_line, end_line)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![text, id, path, source, model, start_line, end_line],
        )?;

        // 3. Insert into Vector table
        let mut blob = Vec::with_capacity(embedding.len() * 4);
        for &f in embedding {
            blob.extend_from_slice(&f.to_le_bytes());
        }
        conn.execute(
            "INSERT INTO chunks_vec (id, embedding) VALUES (?1, ?2)
             ON CONFLICT(id) DO UPDATE SET embedding=excluded.embedding",
            rusqlite::params![id, blob],
        )?;

        Ok(())
    }

    pub fn bm25_to_score(&self, rank: f64) -> f64 {
        let normalized = if rank.is_finite() {
            rank.max(0.0)
        } else {
            999.0
        };
        1.0 / (1.0 + normalized)
    }

    pub fn get_chunks_for_vector_search(&self, model: &str) -> Result<Vec<ChunkData>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, source, model, start_line, end_line, text, embedding FROM chunks WHERE model = ?1"
        )?;

        let rows = stmt.query_map([model], |row| {
            Ok(ChunkData {
                id: row.get(0)?,
                path: row.get(1)?,
                source: row.get(2)?,
                model: row.get(3)?,
                start_line: row.get(4)?,
                end_line: row.get(5)?,
                text: row.get(6)?,
                embedding_json: row.get(7)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn search_vector(&self, query_vec: &[f32], limit: usize) -> Result<Vec<SearchResult>> {
        let mut blob = Vec::with_capacity(query_vec.len() * 4);
        for &f in query_vec {
            blob.extend_from_slice(&f.to_le_bytes());
        }

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT c.id, c.path, c.source, c.model, c.start_line, c.end_line, c.text, v.distance
             FROM chunks_vec v
             JOIN chunks c ON c.id = v.id
             WHERE v.embedding MATCH ?1 AND k = ?2
             ORDER BY distance ASC",
        )?;

        // Note: sqlite-vec uses distance, lower is better. We convert it to a score.
        let rows = stmt.query_map(rusqlite::params![blob, limit as i64], |row| {
            let distance: f64 = row.get(7)?;
            Ok(SearchResult {
                id: row.get(0)?,
                path: row.get(1)?,
                source: row.get(2)?,
                model: row.get(3)?,
                start_line: row.get(4)?,
                end_line: row.get(5)?,
                text: row.get(6)?,
                score: 1.0 / (1.0 + distance), // Distance to score
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn search_fts(
        &self,
        query_str: &str,
        model: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let fts_query = match self.build_fts_query(query_str) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, source, model, start_line, end_line, text, bm25(chunks_fts) as rank
             FROM chunks_fts 
             WHERE chunks_fts MATCH ?1 
               AND (model = ?2 OR model IS NULL)
             ORDER BY rank ASC 
             LIMIT ?3",
        )?;

        let rows = stmt.query_map(rusqlite::params![fts_query, model, limit as i64], |row| {
            let rank: f64 = row.get(7)?;
            Ok(SearchResult {
                id: row.get(0)?,
                path: row.get(1)?,
                source: row.get(2)?,
                model: row.get(3)?,
                start_line: row.get(4)?,
                end_line: row.get(5)?,
                text: row.get(6)?,
                score: self.bm25_to_score(rank),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn save_session(&self, session: &Session) -> Result<()> {
        let transcript_json =
            serde_json::to_string(&session.transcript).unwrap_or_else(|_| "[]".to_string());
        let metadata_json =
            serde_json::to_string(&session.metadata).unwrap_or_else(|_| "{}".to_string());
        let created_at = session.created_at.timestamp();
        let last_active = session.last_active.timestamp();
        let elevated = if session.elevated { 1 } else { 0 };

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sessions (
                id, label, model_override, verbosity, send_policy, elevated, 
                transcript, max_transcript, created_at, last_active, metadata
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                session.id,
                session.label,
                session.model_override,
                session.verbosity.as_str(),
                serde_json::to_string(&session.send_policy).unwrap_or_default(),
                elevated,
                transcript_json,
                session.max_transcript as i64,
                created_at,
                last_active,
                metadata_json
            ],
        )?;
        Ok(())
    }

    pub fn load_session(&self, id: &str) -> Result<Option<Session>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, label, model_override, verbosity, send_policy, elevated, 
                    transcript, max_transcript, created_at, last_active, metadata 
             FROM sessions WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            let verbosity_str: String = row.get(3)?;
            let send_policy_json: String = row.get(4)?;
            let transcript_json: String = row.get(6)?;
            let metadata_json: String = row.get(10)?;

            let created_at_ts: i64 = row.get(8)?;
            let last_active_ts: i64 = row.get(9)?;

            let session = Session {
                id: row.get(0)?,
                label: row.get(1)?,
                model_override: row.get(2)?,
                verbosity: VerbosityLevel::from_str(&verbosity_str),
                send_policy: serde_json::from_str(&send_policy_json).unwrap_or_default(),
                elevated: row.get::<_, i32>(5)? != 0,
                transcript: serde_json::from_str(&transcript_json).unwrap_or_default(),
                max_transcript: row.get::<_, i64>(7)? as usize,
                created_at: chrono::DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                last_active: chrono::DateTime::from_timestamp(last_active_ts, 0)
                    .unwrap_or_default(),
                metadata: serde_json::from_str(&metadata_json).unwrap_or_default(),
            };
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub fn list_sessions(&self) -> Result<Vec<Session>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, label, model_override, verbosity, send_policy, elevated, 
                    transcript, max_transcript, created_at, last_active, metadata 
             FROM sessions ORDER BY last_active DESC",
        )?;

        let session_rows = stmt.query_map([], |row| {
            let verbosity_str: String = row.get(3)?;
            let send_policy_json: String = row.get(4)?;
            let transcript_json: String = row.get(6)?;
            let metadata_json: String = row.get(10)?;

            let created_at_ts: i64 = row.get(8)?;
            let last_active_ts: i64 = row.get(9)?;

            Ok(Session {
                id: row.get(0)?,
                label: row.get(1)?,
                model_override: row.get(2)?,
                verbosity: VerbosityLevel::from_str(&verbosity_str),
                send_policy: serde_json::from_str(&send_policy_json).unwrap_or_default(),
                elevated: row.get::<_, i32>(5)? != 0,
                transcript: serde_json::from_str(&transcript_json).unwrap_or_default(),
                max_transcript: row.get::<_, i64>(7)? as usize,
                created_at: chrono::DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                last_active: chrono::DateTime::from_timestamp(last_active_ts, 0)
                    .unwrap_or_default(),
                metadata: serde_json::from_str(&metadata_json).unwrap_or_default(),
            })
        })?;

        let mut sessions = Vec::new();
        for s in session_rows {
            sessions.push(s?);
        }
        Ok(sessions)
    }

    pub fn delete_session(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM sessions WHERE id = ?1", [id])?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub path: String,
    pub source: String,
    pub model: String,
    pub start_line: i32,
    pub end_line: i32,
    pub text: String,
    pub score: f64,
}

#[derive(Debug)]
pub struct ChunkData {
    pub id: String,
    pub path: String,
    pub source: String,
    pub model: String,
    pub start_line: i32,
    pub end_line: i32,
    pub text: String,
    pub embedding_json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_init() {
        let store = MemoryStore::open_in_memory().unwrap();
        store.save_meta("version", "1").unwrap();
        assert_eq!(store.read_meta("version").unwrap(), Some("1".to_string()));
    }
}
