use rusqlite::{Connection, Result};

pub fn ensure_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meta (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            path TEXT PRIMARY KEY,
            source TEXT NOT NULL DEFAULT 'memory',
            hash TEXT NOT NULL,
            mtime INTEGER NOT NULL,
            size INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunks (
            id TEXT PRIMARY KEY,
            path TEXT NOT NULL,
            source TEXT NOT NULL DEFAULT 'memory',
            start_line INTEGER NOT NULL,
            end_line INTEGER NOT NULL,
            hash TEXT NOT NULL,
            model TEXT NOT NULL,
            text TEXT NOT NULL,
            embedding TEXT NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS embedding_cache (
            provider TEXT NOT NULL,
            model TEXT NOT NULL,
            provider_key TEXT NOT NULL,
            hash TEXT NOT NULL,
            embedding TEXT NOT NULL,
            dims INTEGER,
            updated_at INTEGER NOT NULL,
            PRIMARY KEY (provider, model, provider_key, hash)
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_embedding_cache_updated_at ON embedding_cache(updated_at)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chunks_path ON chunks(path)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chunks_source ON chunks(source)",
        [],
    )?;

    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
            text,
            id UNINDEXED,
            path UNINDEXED,
            source UNINDEXED,
            model UNINDEXED,
            start_line UNINDEXED,
            end_line UNINDEXED
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            label TEXT,
            model_override TEXT,
            verbosity TEXT NOT NULL,
            send_policy TEXT NOT NULL,
            elevated INTEGER NOT NULL DEFAULT 0,
            transcript TEXT NOT NULL,
            max_transcript INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            last_active INTEGER NOT NULL,
            metadata TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn ensure_vector_table(conn: &Connection, dimensions: usize) -> Result<()> {
    conn.execute(
        &format!(
            "CREATE VIRTUAL TABLE IF NOT EXISTS chunks_vec USING vec0(
                id TEXT PRIMARY KEY,
                embedding FLOAT[{}]
            )",
            dimensions
        ),
        [],
    )?;
    Ok(())
}
