//! Log statistics collection and reporting.
//!
//! This module provides statistics collection for log streams processed by phos.
//! Statistics are collected alongside colorization and displayed after processing.
//!
//! ## Collected Statistics
//!
//! - **Line counts**: Total lines processed and lines that matched colorization rules
//! - **Log levels**: Distribution of ERROR, WARN, INFO, DEBUG, TRACE levels
//! - **Time range**: First and last timestamps found (ISO 8601 and syslog formats)
//! - **Top errors**: Most frequent error messages (up to 10)
//! - **Error rate**: Percentage of lines containing errors
//!
//! ## Usage
//!
//! ```rust,ignore
//! use phos::StatsCollector;
//!
//! let mut collector = StatsCollector::new();
//! collector.process_line("2024-01-15T10:30:45 ERROR Connection failed", true);
//! collector.print_summary();  // outputs to stderr
//! ```
//!
//! Statistics are written to stderr to keep stdout clean for pipe chains.

use std::collections::HashMap;
use std::io::{self, Write};

use regex::Regex;

// ---------------------------------------------------------------------------
// Helper Functions
// ---------------------------------------------------------------------------

/// Calculate percentage, returning 0.0 if total is zero to avoid division by zero.
fn percentage(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Truncate a message to a maximum length, adding "..." if truncated.
/// Used for displaying error messages in a fixed-width column.
fn truncate_message(msg: &str, max_len: usize) -> String {
    if msg.len() > max_len {
        format!("{}...", &msg[..max_len.saturating_sub(3)])
    } else {
        msg.to_string()
    }
}

/// Statistics collected from log processing.
#[derive(Debug, Default)]
pub struct Stats {
    /// Total lines processed
    pub total_lines: usize,
    /// Lines that matched at least one rule
    pub matched_lines: usize,
    /// Lines skipped by skip rules
    pub skipped_lines: usize,
    /// Log level counts
    pub log_levels: LogLevelCounts,
    /// First timestamp seen (if any)
    pub first_timestamp: Option<String>,
    /// Last timestamp seen (if any)
    pub last_timestamp: Option<String>,
    /// Top error messages (message -> count)
    pub top_errors: HashMap<String, usize>,
    /// Maximum errors to track
    max_errors: usize,
}

/// Counts of log levels found in the log stream.
///
/// Only one level is counted per line, in priority order:
/// ERROR > WARN > INFO > DEBUG > TRACE
#[derive(Debug, Default, Clone)]
pub struct LogLevelCounts {
    pub error: usize,
    pub warn: usize,
    pub info: usize,
    pub debug: usize,
    pub trace: usize,
}

impl LogLevelCounts {
    /// Total of all log levels.
    pub fn total(&self) -> usize {
        self.error + self.warn + self.info + self.debug + self.trace
    }

    /// Returns an iterator over (name, count) pairs for non-zero levels.
    /// Used for functional-style printing of level statistics.
    pub fn iter_nonzero(&self) -> impl Iterator<Item = (&'static str, usize)> {
        [
            ("ERROR", self.error),
            ("WARN", self.warn),
            ("INFO", self.info),
            ("DEBUG", self.debug),
            ("TRACE", self.trace),
        ]
        .into_iter()
        .filter(|(_, count)| *count > 0)
    }
}

/// Compiled regex patterns for extracting statistics from log lines.
///
/// All patterns are compiled once at construction and reused for each line.
/// Log level patterns are case-insensitive and match common variations.
pub struct StatsPatterns {
    /// Matches ERROR, ERR, CRIT, CRITICAL, FATAL, PANIC (case-insensitive)
    error: Regex,
    /// Matches WARN, WARNING (case-insensitive)
    warn: Regex,
    /// Matches INFO, NOTICE (case-insensitive)
    info: Regex,
    /// Matches DEBUG (case-insensitive)
    debug: Regex,
    /// Matches TRACE (case-insensitive)
    trace: Regex,
    /// ISO 8601 timestamps: 2024-01-15T10:30:45 or 2024-01-15 10:30:45
    timestamp_iso: Regex,
    /// Syslog timestamps: Jan 15 10:30:45
    timestamp_syslog: Regex,
    /// Extracts error message content after "error:", "failed:", etc.
    error_message: Regex,
}

impl Default for StatsPatterns {
    fn default() -> Self {
        Self {
            error: Regex::new(r"(?i)\b(ERROR|ERR|CRIT|CRITICAL|FATAL|PANIC)\b").unwrap(),
            warn: Regex::new(r"(?i)\b(WARN|WARNING)\b").unwrap(),
            info: Regex::new(r"(?i)\b(INFO|NOTICE)\b").unwrap(),
            debug: Regex::new(r"(?i)\bDEBUG\b").unwrap(),
            trace: Regex::new(r"(?i)\bTRACE\b").unwrap(),
            timestamp_iso: Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}").unwrap(),
            timestamp_syslog: Regex::new(r"[A-Z][a-z]{2}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}").unwrap(),
            error_message: Regex::new(r#"(?i)(?:error|err|failed|failure)[:\s]+["']?([^"'\n]{1,100})"#).unwrap(),
        }
    }
}

impl Stats {
    /// Create a new stats collector.
    pub fn new() -> Self {
        Self {
            max_errors: 10,
            ..Default::default()
        }
    }

    /// Process a line and collect statistics.
    ///
    /// Extracts log level, timestamps, and error messages from the line.
    /// The `had_match` parameter indicates whether the colorizer found any matches.
    pub fn process_line(&mut self, line: &str, patterns: &StatsPatterns, had_match: bool) {
        self.total_lines += 1;

        if had_match {
            self.matched_lines += 1;
        }

        // Detect log level (priority order: error > warn > info > debug > trace)
        // Only one level counted per line to avoid double-counting
        self.detect_log_level(line, patterns);

        // Extract timestamp using ISO 8601 first, falling back to syslog format
        self.extract_timestamp(line, patterns);
    }

    /// Detect and count log level from a line.
    /// Checks in priority order and extracts error messages for ERROR lines.
    fn detect_log_level(&mut self, line: &str, patterns: &StatsPatterns) {
        if patterns.error.is_match(line) {
            self.log_levels.error += 1;
            self.extract_error_message(line, patterns);
        } else if patterns.warn.is_match(line) {
            self.log_levels.warn += 1;
        } else if patterns.info.is_match(line) {
            self.log_levels.info += 1;
        } else if patterns.debug.is_match(line) {
            self.log_levels.debug += 1;
        } else if patterns.trace.is_match(line) {
            self.log_levels.trace += 1;
        }
    }

    /// Extract error message content for tracking top errors.
    /// Collects up to 2x max_errors to ensure we have enough after deduplication.
    fn extract_error_message(&mut self, line: &str, patterns: &StatsPatterns) {
        let msg = patterns
            .error_message
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|m| !m.is_empty());

        if let Some(msg) = msg {
            // Collect 2x max to ensure top N after sorting by frequency
            if self.top_errors.len() < self.max_errors * 2 {
                *self.top_errors.entry(msg).or_insert(0) += 1;
            }
        }
    }

    /// Extract timestamp from line, trying ISO 8601 first then syslog format.
    fn extract_timestamp(&mut self, line: &str, patterns: &StatsPatterns) {
        let timestamp = patterns
            .timestamp_iso
            .find(line)
            .or_else(|| patterns.timestamp_syslog.find(line))
            .map(|m| m.as_str().to_string());

        if let Some(ts) = timestamp {
            if self.first_timestamp.is_none() {
                self.first_timestamp = Some(ts.clone());
            }
            self.last_timestamp = Some(ts);
        }
    }

    /// Print statistics summary to stderr.
    ///
    /// Output goes to stderr to keep stdout clean for piped log data.
    pub fn print_summary(&self) {
        let mut stderr = io::stderr().lock();

        writeln!(stderr).ok();
        writeln!(stderr, "=== Log Statistics ===").ok();
        writeln!(stderr).ok();

        // Basic counts
        writeln!(stderr, "Lines processed: {}", self.total_lines).ok();
        writeln!(
            stderr,
            "Lines with color: {} ({:.1}%)",
            self.matched_lines,
            percentage(self.matched_lines, self.total_lines)
        )
        .ok();
        if self.skipped_lines > 0 {
            writeln!(
                stderr,
                "Lines skipped:    {} ({:.1}%)",
                self.skipped_lines,
                percentage(self.skipped_lines, self.total_lines)
            )
            .ok();
        }
        writeln!(stderr).ok();

        // Time range (if timestamps were found)
        if let (Some(first), Some(last)) = (&self.first_timestamp, &self.last_timestamp) {
            writeln!(stderr, "Time range:").ok();
            writeln!(stderr, "  First: {first}").ok();
            writeln!(stderr, "  Last:  {last}").ok();
            writeln!(stderr).ok();
        }

        // Log levels using iterator
        let level_total = self.log_levels.total();
        if level_total > 0 {
            writeln!(stderr, "Log levels:").ok();
            self.log_levels.iter_nonzero().for_each(|(name, count)| {
                writeln!(
                    stderr,
                    "  {name:5} {:>6} ({:>5.1}%)",
                    count,
                    percentage(count, level_total)
                )
                .ok();
            });
            writeln!(stderr).ok();
        }

        // Top errors sorted by frequency
        if !self.top_errors.is_empty() {
            writeln!(stderr, "Top errors:").ok();

            let mut errors: Vec<_> = self.top_errors.iter().collect();
            errors.sort_by(|a, b| b.1.cmp(a.1));

            errors
                .iter()
                .take(self.max_errors)
                .for_each(|(msg, count)| {
                    let truncated = truncate_message(msg, 60);
                    writeln!(stderr, "  {count:>4}x {truncated}").ok();
                });
            writeln!(stderr).ok();
        }

        // Overall error rate
        if self.log_levels.error > 0 && self.total_lines > 0 {
            writeln!(
                stderr,
                "Error rate: {:.2}% ({} errors in {} lines)",
                percentage(self.log_levels.error, self.total_lines),
                self.log_levels.error,
                self.total_lines
            )
            .ok();
        }
    }

    /// Merge statistics from another Stats instance into this one.
    ///
    /// Used when combining stats from multiple threads (e.g., stdout + stderr).
    pub fn merge(&mut self, other: &Stats) {
        self.total_lines += other.total_lines;
        self.matched_lines += other.matched_lines;
        self.skipped_lines += other.skipped_lines;

        // Merge log level counts
        self.log_levels.error += other.log_levels.error;
        self.log_levels.warn += other.log_levels.warn;
        self.log_levels.info += other.log_levels.info;
        self.log_levels.debug += other.log_levels.debug;
        self.log_levels.trace += other.log_levels.trace;

        // Keep earliest first_timestamp
        if self.first_timestamp.is_none() {
            self.first_timestamp.clone_from(&other.first_timestamp);
        }

        // Use latest last_timestamp
        if other.last_timestamp.is_some() {
            self.last_timestamp.clone_from(&other.last_timestamp);
        }

        // Merge error message counts
        other.top_errors.iter().for_each(|(msg, count)| {
            *self.top_errors.entry(msg.clone()).or_insert(0) += count;
        });
    }
}

/// Stats collector that wraps around a colorizer.
pub struct StatsCollector {
    stats: Stats,
    patterns: StatsPatterns,
}

impl StatsCollector {
    /// Create a new stats collector.
    pub fn new() -> Self {
        Self {
            stats: Stats::new(),
            patterns: StatsPatterns::default(),
        }
    }

    /// Process a line and return whether it had matches (for external colorizer to use).
    pub fn process_line(&mut self, line: &str, had_match: bool) {
        self.stats.process_line(line, &self.patterns, had_match);
    }

    /// Record that a line was skipped by a skip rule.
    pub fn record_skipped(&mut self) {
        self.stats.skipped_lines += 1;
    }

    /// Get the collected stats.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Get mutable access to the collected stats.
    pub fn stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }

    /// Print the summary.
    pub fn print_summary(&self) {
        self.stats.print_summary();
    }
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_collection() {
        let mut collector = StatsCollector::new();

        collector.process_line("2024-01-15T10:30:45 INFO Starting service", true);
        collector.process_line("2024-01-15T10:30:46 ERROR Connection failed", true);
        collector.process_line("2024-01-15T10:30:47 WARN Low memory", true);
        collector.process_line("2024-01-15T10:30:48 DEBUG Processing request", false);

        let stats = collector.stats();
        assert_eq!(stats.total_lines, 4);
        assert_eq!(stats.matched_lines, 3);
        assert_eq!(stats.log_levels.error, 1);
        assert_eq!(stats.log_levels.warn, 1);
        assert_eq!(stats.log_levels.info, 1);
        assert_eq!(stats.log_levels.debug, 1);
    }

    #[test]
    fn test_timestamp_extraction() {
        let mut collector = StatsCollector::new();

        collector.process_line("2024-01-15T10:30:45 INFO First", true);
        collector.process_line("2024-01-15T10:30:50 INFO Last", true);

        let stats = collector.stats();
        assert_eq!(stats.first_timestamp, Some("2024-01-15T10:30:45".to_string()));
        assert_eq!(stats.last_timestamp, Some("2024-01-15T10:30:50".to_string()));
    }

    #[test]
    fn test_syslog_timestamp() {
        let mut collector = StatsCollector::new();

        collector.process_line("Jan 15 10:30:45 hostname service: INFO message", true);

        let stats = collector.stats();
        assert!(stats.first_timestamp.is_some());
    }
}
