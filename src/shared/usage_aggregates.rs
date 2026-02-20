//! Usage aggregates for tracking resource consumption

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Usage metrics for a single resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageMetrics {
    /// Total count/amount used
    #[serde(default)]
    pub total: u64,

    /// Count in current period
    #[serde(default)]
    pub current: u64,

    /// Peak usage
    #[serde(default)]
    pub peak: u64,

    /// Average usage
    #[serde(default)]
    pub average: f64,

    /// Last updated timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl UsageMetrics {
    /// Record new usage
    pub fn record(&mut self, amount: u64) {
        self.total += amount;
        self.current += amount;

        if amount > self.peak {
            self.peak = amount;
        }

        // Simple running average
        if self.total > 0 {
            self.average = self.total as f64 / self.current.max(1) as f64;
        }

        self.last_updated = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Reset current period
    pub fn reset_period(&mut self) {
        self.current = 0;
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Usage aggregates collection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageAggregates {
    /// Token usage by model/provider
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub tokens: HashMap<String, TokenUsage>,

    /// API call counts
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub api_calls: HashMap<String, UsageMetrics>,

    /// Request counts
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub requests: HashMap<String, UsageMetrics>,

    /// Duration metrics (in milliseconds)
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub duration_ms: HashMap<String, UsageMetrics>,

    /// Custom metrics
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub custom: HashMap<String, UsageMetrics>,

    /// Session start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_start: Option<String>,

    /// Last activity time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<String>,
}

/// Token usage breakdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    /// Input/prompt tokens
    #[serde(default)]
    pub input: u64,

    /// Output/completion tokens
    #[serde(default)]
    pub output: u64,

    /// Total tokens
    #[serde(default)]
    pub total: u64,

    /// Estimated cost (in USD)
    #[serde(default)]
    pub estimated_cost: f64,
}

impl TokenUsage {
    /// Record token usage
    pub fn record(&mut self, input: u64, output: u64, cost_per_1k: Option<f64>) {
        self.input += input;
        self.output += output;
        self.total += input + output;

        if let Some(rate) = cost_per_1k {
            let input_cost = (input as f64 / 1000.0) * rate;
            let output_cost = (output as f64 / 1000.0) * rate * 2.0; // Output usually costs more
            self.estimated_cost += input_cost + output_cost;
        }
    }

    /// Get total tokens
    pub fn total_tokens(&self) -> u64 {
        self.total
    }
}

impl UsageAggregates {
    /// Create new usage aggregates
    pub fn new() -> Self {
        Self {
            session_start: Some(chrono::Utc::now().to_rfc3339()),
            ..Default::default()
        }
    }

    /// Record token usage
    pub fn record_tokens(&mut self, key: impl Into<String>, input: u64, output: u64) {
        let key = key.into();
        self.tokens
            .entry(key)
            .or_default()
            .record(input, output, None);
        self.update_activity();
    }

    /// Record token usage with cost
    pub fn record_tokens_with_cost(
        &mut self,
        key: impl Into<String>,
        input: u64,
        output: u64,
        cost_per_1k: f64,
    ) {
        let key = key.into();
        self.tokens
            .entry(key)
            .or_default()
            .record(input, output, Some(cost_per_1k));
        self.update_activity();
    }

    /// Record API call
    pub fn record_api_call(&mut self, key: impl Into<String>) {
        let key = key.into();
        self.api_calls.entry(key).or_default().record(1);
        self.update_activity();
    }

    /// Record request
    pub fn record_request(&mut self, key: impl Into<String>) {
        let key = key.into();
        self.requests.entry(key).or_default().record(1);
        self.update_activity();
    }

    /// Record duration
    pub fn record_duration(&mut self, key: impl Into<String>, duration_ms: u64) {
        let key = key.into();
        self.duration_ms.entry(key).or_default().record(duration_ms);
        self.update_activity();
    }

    /// Record custom metric
    pub fn record_custom(&mut self, key: impl Into<String>, amount: u64) {
        let key = key.into();
        self.custom.entry(key).or_default().record(amount);
        self.update_activity();
    }

    /// Get total tokens across all models
    pub fn total_tokens(&self) -> u64 {
        self.tokens.values().map(|t| t.total).sum()
    }

    /// Get total estimated cost
    pub fn total_estimated_cost(&self) -> f64 {
        self.tokens.values().map(|t| t.estimated_cost).sum()
    }

    /// Get total API calls
    pub fn total_api_calls(&self) -> u64 {
        self.api_calls.values().map(|m| m.total).sum()
    }

    /// Get total requests
    pub fn total_requests(&self) -> u64 {
        self.requests.values().map(|m| m.total).sum()
    }

    /// Reset period metrics
    pub fn reset_period(&mut self) {
        for metrics in self.api_calls.values_mut() {
            metrics.reset_period();
        }
        for metrics in self.requests.values_mut() {
            metrics.reset_period();
        }
        for metrics in self.duration_ms.values_mut() {
            metrics.reset_period();
        }
        for metrics in self.custom.values_mut() {
            metrics.reset_period();
        }
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Merge another aggregates into this one
    pub fn merge(&mut self, other: &UsageAggregates) {
        for (key, usage) in &other.tokens {
            let entry = self.tokens.entry(key.clone()).or_default();
            entry.input += usage.input;
            entry.output += usage.output;
            entry.total += usage.total;
            entry.estimated_cost += usage.estimated_cost;
        }

        for (key, metrics) in &other.api_calls {
            let entry = self.api_calls.entry(key.clone()).or_default();
            entry.total += metrics.total;
            entry.peak = entry.peak.max(metrics.peak);
        }

        for (key, metrics) in &other.requests {
            let entry = self.requests.entry(key.clone()).or_default();
            entry.total += metrics.total;
            entry.peak = entry.peak.max(metrics.peak);
        }

        self.update_activity();
    }

    fn update_activity(&mut self) {
        self.last_activity = Some(chrono::Utc::now().to_rfc3339());
    }
}

/// Usage tracker with windowing
#[derive(Debug)]
pub struct UsageTracker {
    current: UsageAggregates,
    history: Vec<UsageAggregates>,
    max_history: usize,
}

impl UsageTracker {
    /// Create new tracker with history size
    pub fn new(max_history: usize) -> Self {
        Self {
            current: UsageAggregates::new(),
            history: Vec::new(),
            max_history,
        }
    }

    /// Get current aggregates
    pub fn current(&self) -> &UsageAggregates {
        &self.current
    }

    /// Get mutable current aggregates
    pub fn current_mut(&mut self) -> &mut UsageAggregates {
        &mut self.current
    }

    /// Rotate to new period
    pub fn rotate(&mut self) {
        let new_period = UsageAggregates::new();
        let old_period = std::mem::replace(&mut self.current, new_period);

        self.history.push(old_period);

        // Trim history
        while self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get total across all history
    pub fn total(&self) -> UsageAggregates {
        let mut total = self.current.clone();
        for period in &self.history {
            total.merge(period);
        }
        total
    }

    /// Get history
    pub fn history(&self) -> &[UsageAggregates] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for UsageTracker {
    fn default() -> Self {
        Self::new(24) // Keep 24 periods by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_usage() {
        let mut usage = TokenUsage::default();
        usage.record(100, 50, Some(0.01));

        assert_eq!(usage.input, 100);
        assert_eq!(usage.output, 50);
        assert_eq!(usage.total, 150);
        assert!(usage.estimated_cost > 0.0);
    }

    #[test]
    fn test_usage_metrics() {
        let mut metrics = UsageMetrics::default();
        metrics.record(10);
        metrics.record(20);
        metrics.record(5);

        assert_eq!(metrics.total, 35);
        assert_eq!(metrics.current, 35);
        assert_eq!(metrics.peak, 20);
        assert!(metrics.last_updated.is_some());
    }

    #[test]
    fn test_usage_aggregates() {
        let mut aggregates = UsageAggregates::new();

        aggregates.record_tokens("gpt-4", 100, 50);
        aggregates.record_api_call("openai");
        aggregates.record_request("chat");
        aggregates.record_duration("inference", 500);
        aggregates.record_custom("custom_metric", 42);

        assert_eq!(aggregates.total_tokens(), 150);
        assert_eq!(aggregates.total_api_calls(), 1);
        assert_eq!(aggregates.total_requests(), 1);
        assert!(aggregates.session_start.is_some());
        assert!(aggregates.last_activity.is_some());
    }

    #[test]
    fn test_usage_tracker() {
        let mut tracker = UsageTracker::new(3);

        tracker.current_mut().record_tokens("model", 100, 50);
        assert_eq!(tracker.current().total_tokens(), 150);

        tracker.rotate();
        tracker.current_mut().record_tokens("model", 50, 25);

        assert_eq!(tracker.history().len(), 1);
        assert_eq!(tracker.total().total_tokens(), 225);
    }

    #[test]
    fn test_merge() {
        let mut agg1 = UsageAggregates::new();
        agg1.record_tokens("model", 100, 50);

        let mut agg2 = UsageAggregates::new();
        agg2.record_tokens("model", 50, 25);
        agg2.record_tokens("other", 10, 5);

        agg1.merge(&agg2);

        assert_eq!(agg1.total_tokens(), 240);
        assert_eq!(agg1.tokens.len(), 2);
    }

    #[test]
    fn test_reset_period() {
        let mut metrics = UsageMetrics::default();
        metrics.record(100);
        assert_eq!(metrics.current, 100);

        metrics.reset_period();
        assert_eq!(metrics.current, 0);
        assert_eq!(metrics.total, 100); // Total preserved
    }
}
