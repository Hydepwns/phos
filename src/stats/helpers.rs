//! Helper functions for statistics calculations and formatting.

use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;

/// Calculate percentage, returning 0.0 if total is zero to avoid division by zero.
#[must_use]
pub fn percentage(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Truncate a message to a maximum length, adding "..." if truncated.
/// Returns a Cow to avoid allocation when no truncation is needed.
#[must_use]
pub fn truncate_message(msg: &str, max_len: usize) -> Cow<'_, str> {
    if msg.len() > max_len {
        format!("{}...", &msg[..max_len.saturating_sub(3)]).into()
    } else {
        msg.into()
    }
}

/// Extract a numeric value from a regex capture group.
pub fn extract_numeric<T: std::str::FromStr>(line: &str, pattern: &Regex) -> Option<T> {
    pattern
        .captures(line)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

/// Format current time as HH:MM:SS (UTC).
#[must_use]
pub fn format_time_hms() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() % 86400)
        .unwrap_or(0);

    format!(
        "{:02}:{:02}:{:02}",
        secs / 3600,
        (secs % 3600) / 60,
        secs % 60
    )
}

/// Prometheus metric types.
pub enum MetricType {
    Counter,
    Gauge,
}

impl MetricType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
        }
    }
}

/// Append a Prometheus metric to the output string.
pub fn append_metric(
    output: &mut String,
    name: &str,
    help: &str,
    metric_type: MetricType,
    labels: &str,
    value: impl std::fmt::Display,
) {
    output.push_str(&format!("# HELP {name} {help}\n"));
    output.push_str(&format!("# TYPE {name} {}\n", metric_type.as_str()));
    output.push_str(&format!("{name}{{{labels}}} {value}\n"));
}

/// Append a labeled metric value (without HELP/TYPE headers).
pub fn append_metric_value(
    output: &mut String,
    name: &str,
    labels: &str,
    value: impl std::fmt::Display,
) {
    output.push_str(&format!("{name}{{{labels}}} {value}\n"));
}
