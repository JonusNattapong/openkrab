use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

const DAY_MS: i64 = 24 * 60 * 60 * 1000;
const DATED_MEMORY_PATH_RE: &str = r"(?:^|/)memory/(\d{4})-(\d{2})-(\d{2})\.md$";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDecayConfig {
    pub enabled: bool,
    pub half_life_days: f64,
}

impl Default for TemporalDecayConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            half_life_days: 30.0,
        }
    }
}

pub fn to_decay_lambda(half_life_days: f64) -> f64 {
    if !half_life_days.is_finite() || half_life_days <= 0.0 {
        return 0.0;
    }
    std::f64::consts::LN_2 / half_life_days
}

pub fn calculate_temporal_decay_multiplier(age_in_days: f64, half_life_days: f64) -> f64 {
    let lambda = to_decay_lambda(half_life_days);
    let clamped_age = age_in_days.max(0.0);

    if lambda <= 0.0 || !clamped_age.is_finite() {
        return 1.0;
    }

    (-lambda * clamped_age).exp()
}

pub fn apply_temporal_decay_to_score(score: f64, age_in_days: f64, half_life_days: f64) -> f64 {
    score * calculate_temporal_decay_multiplier(age_in_days, half_life_days)
}

fn parse_memory_date_from_path(file_path: &str) -> Option<DateTime<Utc>> {
    let normalized = file_path.replace("\\", "/");
    let re = regex::Regex::new(DATED_MEMORY_PATH_RE).ok()?;

    let caps = re.captures(&normalized)?;
    let year: i32 = caps[1].parse().ok()?;
    let month: u32 = caps[2].parse().ok()?;
    let day: u32 = caps[3].parse().ok()?;

    chrono::NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| DateTime::from_naive_utc_and_offset(d, Utc))
}

fn is_evergreen_memory_path(file_path: &str) -> bool {
    let normalized = file_path.replace("\\", "/");
    if normalized == "MEMORY.md" || normalized == "memory.md" {
        return true;
    }
    if !normalized.starts_with("memory/") {
        return false;
    }

    let re = regex::Regex::new(DATED_MEMORY_PATH_RE).unwrap();
    !re.is_match(&normalized)
}

fn extract_timestamp(
    file_path: &str,
    source: Option<&str>,
    workspace_dir: Option<&Path>,
) -> Option<DateTime<Utc>> {
    if let Some(from_path) = parse_memory_date_from_path(file_path) {
        return Some(from_path);
    }

    if source == Some("memory") && is_evergreen_memory_path(file_path) {
        return None;
    }

    if let Some(ws_dir) = workspace_dir {
        let absolute_path = if Path::new(file_path).is_absolute() {
            PathBuf::from(file_path)
        } else {
            ws_dir.join(file_path)
        };

        if let Ok(metadata) = std::fs::metadata(&absolute_path) {
            if let Ok(modified) = metadata.modified() {
                return Some(DateTime::from(modified));
            }
        }
    }

    None
}

fn age_in_days_from_timestamp(timestamp: DateTime<Utc>, now_ms: i64) -> f64 {
    let now = DateTime::from_timestamp_millis(now_ms).unwrap_or_else(Utc::now);
    let age_ms = (now - timestamp).num_milliseconds().max(0);
    age_ms as f64 / DAY_MS as f64
}

use std::path::PathBuf;

pub fn apply_temporal_decay_to_results<T: Clone + TemporalDecayItem>(
    results: Vec<T>,
    config: Option<TemporalDecayConfig>,
    workspace_dir: Option<&Path>,
    now_ms: Option<i64>,
) -> Vec<T> {
    let cfg = config.unwrap_or_default();
    if !cfg.enabled {
        return results;
    }

    let now = now_ms.unwrap_or_else(|| Utc::now().timestamp_millis());

    results
        .into_iter()
        .map(|entry| {
            let timestamp = extract_timestamp(entry.path(), entry.source(), workspace_dir);

            if let Some(ts) = timestamp {
                let decayed_score = apply_temporal_decay_to_score(
                    entry.score(),
                    age_in_days_from_timestamp(ts, now),
                    cfg.half_life_days,
                );
                entry.with_score(decayed_score)
            } else {
                entry
            }
        })
        .collect()
}

pub trait TemporalDecayItem: Clone {
    fn path(&self) -> &str;
    fn source(&self) -> Option<&str>;
    fn score(&self) -> f64;
    fn with_score(&self, score: f64) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_decay_lambda() {
        let lambda = to_decay_lambda(30.0);
        assert!(lambda > 0.0);
        assert!((lambda - std::f64::consts::LN_2 / 30.0).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_temporal_decay_multiplier() {
        let mult = calculate_temporal_decay_multiplier(30.0, 30.0);
        assert!((mult - 0.5).abs() < 0.01);

        let mult_zero = calculate_temporal_decay_multiplier(0.0, 30.0);
        assert!((mult_zero - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_temporal_decay_to_score() {
        let decayed = apply_temporal_decay_to_score(1.0, 30.0, 30.0);
        assert!((decayed - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_memory_date_from_path() {
        let date = parse_memory_date_from_path("memory/2024-01-15.md");
        assert!(date.is_some());

        let no_date = parse_memory_date_from_path("memory/topics.md");
        assert!(no_date.is_none());
    }

    #[test]
    fn test_is_evergreen_memory_path() {
        assert!(is_evergreen_memory_path("MEMORY.md"));
        assert!(is_evergreen_memory_path("memory/topics.md"));
        assert!(!is_evergreen_memory_path("memory/2024-01-15.md"));
    }
}
