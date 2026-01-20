//! Compiled regex patterns for statistics extraction.

use regex::Regex;
use std::sync::LazyLock;

use crate::programs::common::log_levels::ERROR_LEVEL_PATTERN;

/// Compiled regex patterns for extracting statistics from log lines.
///
/// All patterns are compiled once at construction and reused for each line.
/// Log level patterns are case-insensitive and match common variations.
pub struct StatsPatterns {
    /// Matches ERROR, ERR, CRIT, CRITICAL, FATAL, PANIC (case-insensitive)
    pub error: Regex,
    /// Matches WARN, WARNING (case-insensitive)
    pub warn: Regex,
    /// Matches INFO, NOTICE (case-insensitive)
    pub info: Regex,
    /// Matches DEBUG (case-insensitive)
    pub debug: Regex,
    /// Matches TRACE (case-insensitive)
    pub trace: Regex,
    /// ISO 8601 timestamps: 2024-01-15T10:30:45 or 2024-01-15 10:30:45
    pub timestamp_iso: Regex,
    /// Syslog timestamps: Jan 15 10:30:45
    pub timestamp_syslog: Regex,
    /// Extracts error message content after "error:", "failed:", etc.
    pub error_message: Regex,
    /// Extracts peer count from log lines (peer=N, peers=N, Peers N)
    pub peer_count: Regex,
    /// Extracts slot number from log lines (slot=N, slot: N)
    pub slot: Regex,
}

/// Global instance of stats patterns, compiled once at first use.
/// Uses `ERROR_LEVEL_PATTERN` from `common::log_levels` to avoid duplication.
pub static STATS_PATTERNS: LazyLock<StatsPatterns> = LazyLock::new(|| StatsPatterns {
    error: ERROR_LEVEL_PATTERN.clone(),
    warn: Regex::new(r"(?i)\b(WARN|WARNING)\b").unwrap(),
    info: Regex::new(r"(?i)\b(INFO|NOTICE)\b").unwrap(),
    debug: Regex::new(r"(?i)\bDEBUG\b").unwrap(),
    trace: Regex::new(r"(?i)\bTRACE\b").unwrap(),
    timestamp_iso: Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}").unwrap(),
    timestamp_syslog: Regex::new(r"[A-Z][a-z]{2}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}").unwrap(),
    error_message: Regex::new(r#"(?i)(?:error|err|failed|failure)[:\s]+["']?([^"'\n]{1,100})"#)
        .unwrap(),
    peer_count: Regex::new(r"(?i)\bpeers?[=:\s]+(\d+)").unwrap(),
    slot: Regex::new(r"(?i)\bslot[=:\s]+(\d+)").unwrap(),
});
