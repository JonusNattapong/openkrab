//! diagnostics — Lightweight observability module.
//! Ported from `openclaw/extensions/diagnostics-otel/` (Phase 13).
//!
//! Provides span tracing, metric recording, and structured event logging
//! without requiring the full OpenTelemetry SDK.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ─── Span ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_ms: u64,
    pub end_ms: Option<u64>,
    pub attributes: HashMap<String, String>,
    pub events: Vec<SpanEvent>,
    pub status: SpanStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanStatus {
    Unset,
    Ok,
    Error(String),
}

impl Span {
    pub fn new(name: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            span_id: new_id(),
            parent_span_id: None,
            name: name.into(),
            start_ms: now_ms(),
            end_ms: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            status: SpanStatus::Unset,
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_span_id = Some(parent_id.into());
        self
    }

    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    pub fn add_event(&mut self, name: impl Into<String>, attrs: HashMap<String, String>) {
        self.events.push(SpanEvent {
            name: name.into(),
            attributes: attrs,
            timestamp_ms: now_ms(),
        });
    }

    pub fn finish(&mut self) {
        self.end_ms = Some(now_ms());
    }

    pub fn finish_ok(&mut self) {
        self.status = SpanStatus::Ok;
        self.finish();
    }

    pub fn finish_err(&mut self, err: impl Into<String>) {
        self.status = SpanStatus::Error(err.into());
        self.finish();
    }

    pub fn duration_ms(&self) -> Option<u64> {
        self.end_ms.map(|e| e.saturating_sub(self.start_ms))
    }

    pub fn is_finished(&self) -> bool { self.end_ms.is_some() }
    pub fn is_error(&self) -> bool { matches!(self.status, SpanStatus::Error(_)) }
}

// ─── Metrics ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(i64),
    Gauge(f64),
    Histogram(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: MetricValue,
    pub labels: HashMap<String, String>,
    pub timestamp_ms: u64,
}

impl Metric {
    pub fn counter(name: impl Into<String>, value: i64) -> Self {
        Self { name: name.into(), value: MetricValue::Counter(value), labels: HashMap::new(), timestamp_ms: now_ms() }
    }

    pub fn gauge(name: impl Into<String>, value: f64) -> Self {
        Self { name: name.into(), value: MetricValue::Gauge(value), labels: HashMap::new(), timestamp_ms: now_ms() }
    }

    pub fn with_label(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.labels.insert(k.into(), v.into());
        self
    }
}

// ─── Tracer ───────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct Tracer {
    spans: Arc<Mutex<Vec<Span>>>,
    metrics: Arc<Mutex<Vec<Metric>>>,
}

impl Tracer {
    pub fn new() -> Self { Self::default() }

    pub fn start_span(&self, name: impl Into<String>) -> Span {
        Span::new(name, new_id())
    }

    pub fn record_span(&self, span: Span) {
        self.spans.lock().unwrap().push(span);
    }

    pub fn record_metric(&self, metric: Metric) {
        self.metrics.lock().unwrap().push(metric);
    }

    pub fn inc_counter(&self, name: &str, delta: i64) {
        self.record_metric(Metric::counter(name, delta));
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        self.record_metric(Metric::gauge(name, value));
    }

    pub fn spans(&self) -> Vec<Span> {
        self.spans.lock().unwrap().clone()
    }

    pub fn metrics(&self) -> Vec<Metric> {
        self.metrics.lock().unwrap().clone()
    }

    pub fn error_spans(&self) -> Vec<Span> {
        self.spans.lock().unwrap().iter().filter(|s| s.is_error()).cloned().collect()
    }

    pub fn clear(&self) {
        self.spans.lock().unwrap().clear();
        self.metrics.lock().unwrap().clear();
    }
}

// ─── Timed scope helper ───────────────────────────────────────────────────────

pub struct TimedScope {
    pub name: String,
    start: Instant,
}

impl TimedScope {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), start: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn new_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();
    format!("{:016x}", t as u64 ^ (now_ms() << 32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_lifecycle() {
        let mut span = Span::new("test.op", "trace-1");
        span.set_attr("component", "gateway");
        span.finish_ok();
        assert!(span.is_finished());
        assert!(!span.is_error());
        assert_eq!(span.status, SpanStatus::Ok);
    }

    #[test]
    fn span_error() {
        let mut span = Span::new("failing.op", "trace-2");
        span.finish_err("timeout");
        assert!(span.is_error());
        assert!(matches!(span.status, SpanStatus::Error(_)));
    }

    #[test]
    fn tracer_record_and_query() {
        let tracer = Tracer::new();
        let mut s = tracer.start_span("op1");
        s.finish_ok();
        tracer.record_span(s);
        let mut s2 = tracer.start_span("op2");
        s2.finish_err("fail");
        tracer.record_span(s2);
        assert_eq!(tracer.spans().len(), 2);
        assert_eq!(tracer.error_spans().len(), 1);
    }

    #[test]
    fn metrics_counter_gauge() {
        let tracer = Tracer::new();
        tracer.inc_counter("requests.total", 1);
        tracer.set_gauge("memory.usage_mb", 128.5);
        let m = tracer.metrics();
        assert_eq!(m.len(), 2);
        assert!(matches!(m[0].value, MetricValue::Counter(1)));
    }

    #[test]
    fn timed_scope() {
        let scope = TimedScope::new("compute");
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert!(scope.elapsed_ms() >= 1);
    }
}
