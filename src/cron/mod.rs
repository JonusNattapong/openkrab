//! cron — Cron job scheduler.
//! Ported from `openkrab/src/cron/` (Phase 6).
//!
//! Provides a persistent cron-like scheduler for recurring agent tasks.
//! Jobs are stored in `memory/schedule.json` and evaluated against the
//! system clock on each tick.

use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Job definition ───────────────────────────────────────────────────────────

/// Frequency specification for a cron job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JobFrequency {
    /// Every N seconds.
    Interval { secs: u64 },
    /// Once at a specific UTC timestamp (one-shot).
    Once { at: DateTime<Utc> },
    /// Cron-expression-like daily schedule (HH:MM UTC).
    Daily { hour: u8, minute: u8 },
    /// Hourly at a specific minute.
    Hourly { minute: u8 },
}

/// A single scheduled job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    /// Unique identifier.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Natural-language or structured task description sent to the agent.
    pub task: String,
    /// Scheduling frequency.
    pub frequency: JobFrequency,
    /// Whether the job is active.
    pub enabled: bool,
    /// Last time the job was triggered (None = never).
    pub last_run: Option<DateTime<Utc>>,
    /// Created at.
    pub created_at: DateTime<Utc>,
}

impl CronJob {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        task: impl Into<String>,
        frequency: JobFrequency,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            task: task.into(),
            frequency,
            enabled: true,
            last_run: None,
            created_at: Utc::now(),
        }
    }

    /// Check if this job should fire now, given the current time.
    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        if !self.enabled {
            return false;
        }
        match &self.frequency {
            JobFrequency::Interval { secs } => match self.last_run {
                None => true,
                Some(last) => (now - last).num_seconds() >= *secs as i64,
            },
            JobFrequency::Once { at } => self.last_run.is_none() && now >= *at,
            JobFrequency::Daily { hour, minute } => {
                let same_slot = now.hour() == *hour as u32 && now.minute() == *minute as u32;
                let not_run_today = match self.last_run {
                    None => true,
                    Some(last) => now.date_naive() > last.date_naive(),
                };
                same_slot && not_run_today
            }
            JobFrequency::Hourly { minute } => {
                let same_slot = now.minute() == *minute as u32;
                let not_run_this_hour = match self.last_run {
                    None => true,
                    Some(last) => {
                        let hour_secs = 3600i64;
                        (now - last).num_seconds() >= hour_secs
                    }
                };
                same_slot && not_run_this_hour
            }
        }
    }
}

// ─── Cron store ───────────────────────────────────────────────────────────────

/// In-memory store of cron jobs backed by a JSON file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CronStore {
    pub jobs: HashMap<String, CronJob>,
}

impl CronStore {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    /// Load from a JSON file. Returns an empty store if the file doesn't exist.
    pub fn load(path: &std::path::Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = std::fs::read_to_string(path)?;
        let store: Self = serde_json::from_str(&content)?;
        Ok(store)
    }

    /// Persist to a JSON file.
    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, job: CronJob) {
        self.jobs.insert(job.id.clone(), job);
    }

    pub fn remove(&mut self, id: &str) -> Option<CronJob> {
        self.jobs.remove(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut CronJob> {
        self.jobs.get_mut(id)
    }

    /// Returns a sorted list of all jobs.
    pub fn list(&self) -> Vec<&CronJob> {
        let mut jobs: Vec<&CronJob> = self.jobs.values().collect();
        jobs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        jobs
    }

    /// Returns all jobs that are due at `now`.
    pub fn due_jobs(&self, now: DateTime<Utc>) -> Vec<&CronJob> {
        self.jobs.values().filter(|j| j.is_due(now)).collect()
    }
}

// ─── Cron service ─────────────────────────────────────────────────────────────

/// High-level cron service that can be ticked from an async runtime loop.
pub struct CronService {
    store: CronStore,
    store_path: std::path::PathBuf,
}

impl CronService {
    pub fn new(store_path: impl Into<std::path::PathBuf>) -> Result<Self> {
        let path = store_path.into();
        let store = CronStore::load(&path)?;
        Ok(Self {
            store,
            store_path: path,
        })
    }

    pub fn add_job(&mut self, job: CronJob) -> Result<()> {
        self.store.add(job);
        self.store.save(&self.store_path)
    }

    pub fn remove_job(&mut self, id: &str) -> Result<Option<CronJob>> {
        let removed = self.store.remove(id);
        self.store.save(&self.store_path)?;
        Ok(removed)
    }

    pub fn list_jobs(&self) -> Vec<&CronJob> {
        self.store.list()
    }

    /// Tick: collect all due jobs, mark them as run, persist, and return their tasks.
    pub fn tick(&mut self) -> Result<Vec<(String, String)>> {
        let now = Utc::now();
        let due_ids: Vec<String> = self
            .store
            .due_jobs(now)
            .iter()
            .map(|j| j.id.clone())
            .collect();

        let mut tasks = Vec::new();
        for id in &due_ids {
            if let Some(job) = self.store.get_mut(id) {
                tasks.push((job.id.clone(), job.task.clone()));
                job.last_run = Some(now);
                // Disable one-shot jobs after firing
                if matches!(job.frequency, JobFrequency::Once { .. }) {
                    job.enabled = false;
                }
            }
        }

        if !tasks.is_empty() {
            self.store.save(&self.store_path)?;
        }
        Ok(tasks)
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Parse a simple "every Xm", "every Xh", "every Xs" duration string.
pub fn parse_interval(s: &str) -> Option<JobFrequency> {
    let s = s.trim().to_lowercase();
    let s = s.strip_prefix("every ").unwrap_or(&s);
    if let Some(n) = s.strip_suffix('s').and_then(|n| n.parse::<u64>().ok()) {
        return Some(JobFrequency::Interval { secs: n });
    }
    if let Some(n) = s.strip_suffix('m').and_then(|n| n.parse::<u64>().ok()) {
        return Some(JobFrequency::Interval { secs: n * 60 });
    }
    if let Some(n) = s.strip_suffix('h').and_then(|n| n.parse::<u64>().ok()) {
        return Some(JobFrequency::Interval { secs: n * 3600 });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t(ts: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(ts, 0).unwrap()
    }

    #[test]
    fn interval_job_is_due_on_first_run() {
        let job = CronJob::new("1", "Test", "do thing", JobFrequency::Interval { secs: 60 });
        assert!(job.is_due(Utc::now()));
    }

    #[test]
    fn interval_job_not_due_before_interval() {
        let mut job = CronJob::new(
            "1",
            "Test",
            "do thing",
            JobFrequency::Interval { secs: 120 },
        );
        job.last_run = Some(Utc::now());
        assert!(!job.is_due(Utc::now()));
    }

    #[test]
    fn one_shot_fires_once() {
        let past = Utc::now() - chrono::Duration::seconds(10);
        let mut job = CronJob::new("1", "Test", "do thing", JobFrequency::Once { at: past });
        assert!(job.is_due(Utc::now()));
        job.last_run = Some(Utc::now());
        assert!(!job.is_due(Utc::now()));
    }

    #[test]
    fn parse_interval_strings() {
        assert_eq!(
            parse_interval("every 30s"),
            Some(JobFrequency::Interval { secs: 30 })
        );
        assert_eq!(
            parse_interval("every 5m"),
            Some(JobFrequency::Interval { secs: 300 })
        );
        assert_eq!(
            parse_interval("every 2h"),
            Some(JobFrequency::Interval { secs: 7200 })
        );
        assert_eq!(parse_interval("bad"), None);
    }

    #[test]
    fn disabled_job_not_due() {
        let mut job = CronJob::new("1", "Test", "do thing", JobFrequency::Interval { secs: 0 });
        job.enabled = false;
        assert!(!job.is_due(Utc::now()));
    }

    #[test]
    fn cron_store_add_and_list() {
        let mut store = CronStore::new();
        store.add(CronJob::new(
            "a",
            "A",
            "task a",
            JobFrequency::Interval { secs: 60 },
        ));
        store.add(CronJob::new(
            "b",
            "B",
            "task b",
            JobFrequency::Interval { secs: 120 },
        ));
        assert_eq!(store.jobs.len(), 2);
        store.remove("a");
        assert_eq!(store.jobs.len(), 1);
    }
}
