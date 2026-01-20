//! Core statistics types and processing.

use std::collections::HashMap;
use std::io::{self, Write};

use regex::Regex;

use super::export::{
    ErrorJson, EthereumJson, LogLevelsJson, ProcessingJson, StatsJson, TimeRangeJson,
};
use super::helpers::{
    append_metric, append_metric_value, extract_numeric, format_time_hms, percentage,
    truncate_message, MetricType,
};
use super::patterns::StatsPatterns;

/// Type alias for log level pattern matching with associated incrementer.
type LevelPattern<'a> = (&'a Regex, fn(&mut LogLevelCounts));

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
    #[must_use]
    pub fn total(&self) -> usize {
        self.error + self.warn + self.info + self.debug + self.trace
    }

    /// Returns an iterator over (name, count) pairs for non-zero levels.
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
    /// Last observed peer count (for alerting)
    pub last_peer_count: Option<usize>,
    /// Last observed slot number (for alerting)
    pub last_slot: Option<u64>,
}

impl Stats {
    /// Create a new stats collector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_errors: 10,
            ..Default::default()
        }
    }

    /// Process a line and collect statistics.
    pub fn process_line(&mut self, line: &str, patterns: &StatsPatterns, had_match: bool) {
        self.total_lines += 1;

        if had_match {
            self.matched_lines += 1;
        }

        self.detect_log_level(line, patterns);
        self.extract_timestamp(line, patterns);
        self.extract_peer_count(line, patterns);
        self.extract_slot(line, patterns);
    }

    /// Extract peer count from a line.
    fn extract_peer_count(&mut self, line: &str, patterns: &StatsPatterns) {
        if let Some(count) = extract_numeric::<usize>(line, &patterns.peer_count) {
            self.last_peer_count = Some(count);
        }
    }

    /// Extract slot number from a line.
    fn extract_slot(&mut self, line: &str, patterns: &StatsPatterns) {
        if let Some(slot) = extract_numeric::<u64>(line, &patterns.slot) {
            self.last_slot = Some(slot);
        }
    }

    /// Detect and count log level from a line.
    fn detect_log_level(&mut self, line: &str, patterns: &StatsPatterns) {
        let level_patterns: &[LevelPattern] = &[
            (&patterns.error, |c| c.error += 1),
            (&patterns.warn, |c| c.warn += 1),
            (&patterns.info, |c| c.info += 1),
            (&patterns.debug, |c| c.debug += 1),
            (&patterns.trace, |c| c.trace += 1),
        ];

        if let Some((idx, _)) = level_patterns
            .iter()
            .enumerate()
            .find(|(_, (pattern, _))| pattern.is_match(line))
        {
            level_patterns[idx].1(&mut self.log_levels);
            if idx == 0 {
                self.extract_error_message(line, patterns);
            }
        }
    }

    /// Extract error message content for tracking top errors.
    fn extract_error_message(&mut self, line: &str, patterns: &StatsPatterns) {
        let msg = patterns
            .error_message
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
            .filter(|m| !m.is_empty());

        if let Some(msg) = msg {
            if self.top_errors.len() < self.max_errors * 2 {
                *self.top_errors.entry(msg.to_string()).or_insert(0) += 1;
            }
        }
    }

    /// Extract timestamp from line.
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

    /// Get top errors sorted by frequency (descending).
    fn sorted_errors(&self) -> Vec<(&String, &usize)> {
        let mut errors: Vec<_> = self.top_errors.iter().collect();
        errors.sort_by(|a, b| b.1.cmp(a.1));
        errors
    }

    /// Print statistics summary to stderr.
    pub fn print_summary(&self) {
        let mut stderr = io::stderr().lock();

        macro_rules! print_stat {
            ($($arg:tt)*) => { let _ = writeln!(stderr, $($arg)*); };
            () => { let _ = writeln!(stderr); };
        }

        print_stat!();
        print_stat!("=== Log Statistics ===");
        print_stat!();

        print_stat!("Lines processed: {}", self.total_lines);
        print_stat!(
            "Lines with color: {} ({:.1}%)",
            self.matched_lines,
            percentage(self.matched_lines, self.total_lines)
        );
        if self.skipped_lines > 0 {
            print_stat!(
                "Lines skipped:    {} ({:.1}%)",
                self.skipped_lines,
                percentage(self.skipped_lines, self.total_lines)
            );
        }
        print_stat!();

        if let (Some(first), Some(last)) = (&self.first_timestamp, &self.last_timestamp) {
            print_stat!("Time range:");
            print_stat!("  First: {first}");
            print_stat!("  Last:  {last}");
            print_stat!();
        }

        let level_total = self.log_levels.total();
        if level_total > 0 {
            print_stat!("Log levels:");
            for (name, count) in self.log_levels.iter_nonzero() {
                print_stat!(
                    "  {name:5} {:>6} ({:>5.1}%)",
                    count,
                    percentage(count, level_total)
                );
            }
            print_stat!();
        }

        if !self.top_errors.is_empty() {
            print_stat!("Top errors:");
            for (msg, count) in self.sorted_errors().iter().take(self.max_errors) {
                let truncated = truncate_message(msg, 60);
                print_stat!("  {count:>4}x {truncated}");
            }
            print_stat!();
        }

        if self.log_levels.error > 0 && self.total_lines > 0 {
            print_stat!(
                "Error rate: {:.2}% ({} errors in {} lines)",
                percentage(self.log_levels.error, self.total_lines),
                self.log_levels.error,
                self.total_lines
            );
        }
    }

    /// Export statistics as JSON.
    pub fn to_json(&self, program: Option<&str>) -> StatsJson {
        StatsJson {
            version: env!("CARGO_PKG_VERSION").to_string(),
            program: program.map(String::from),
            processing: ProcessingJson {
                total_lines: self.total_lines,
                matched_lines: self.matched_lines,
                skipped_lines: self.skipped_lines,
                match_percentage: percentage(self.matched_lines, self.total_lines),
            },
            time_range: TimeRangeJson {
                first: self.first_timestamp.clone(),
                last: self.last_timestamp.clone(),
            },
            log_levels: LogLevelsJson {
                error: self.log_levels.error,
                warn: self.log_levels.warn,
                info: self.log_levels.info,
                debug: self.log_levels.debug,
                trace: self.log_levels.trace,
            },
            error_rate: percentage(self.log_levels.error, self.total_lines),
            top_errors: self
                .sorted_errors()
                .into_iter()
                .take(self.max_errors)
                .map(|(msg, count)| ErrorJson {
                    message: msg.clone(),
                    count: *count,
                })
                .collect(),
            ethereum: EthereumJson {
                last_slot: self.last_slot,
                last_peer_count: self.last_peer_count,
            },
        }
    }

    /// Export statistics as Prometheus metrics format.
    #[must_use]
    pub fn to_prometheus(&self, program: Option<&str>) -> String {
        let program_label = program.unwrap_or("unknown");
        let base_labels = format!("program=\"{program_label}\"");
        let mut output = String::new();

        append_metric(
            &mut output,
            "phos_lines_processed_total",
            "Total lines processed",
            MetricType::Counter,
            &base_labels,
            self.total_lines,
        );
        append_metric(
            &mut output,
            "phos_lines_matched_total",
            "Lines that matched colorization rules",
            MetricType::Counter,
            &base_labels,
            self.matched_lines,
        );

        output.push_str("# HELP phos_log_level_total Log entries by level\n");
        output.push_str("# TYPE phos_log_level_total counter\n");
        for (level, count) in [
            ("error", self.log_levels.error),
            ("warn", self.log_levels.warn),
            ("info", self.log_levels.info),
            ("debug", self.log_levels.debug),
            ("trace", self.log_levels.trace),
        ] {
            let labels = format!("{base_labels},level=\"{level}\"");
            append_metric_value(&mut output, "phos_log_level_total", &labels, count);
        }

        append_metric(
            &mut output,
            "phos_error_rate",
            "Error rate as percentage",
            MetricType::Gauge,
            &base_labels,
            format!("{:.2}", percentage(self.log_levels.error, self.total_lines)),
        );

        if let Some(slot) = self.last_slot {
            append_metric(
                &mut output,
                "phos_ethereum_slot",
                "Last observed slot number",
                MetricType::Gauge,
                &base_labels,
                slot,
            );
        }

        if let Some(peers) = self.last_peer_count {
            append_metric(
                &mut output,
                "phos_ethereum_peers",
                "Last observed peer count",
                MetricType::Gauge,
                &base_labels,
                peers,
            );
        }

        output
    }

    /// Format statistics as a compact single-line string.
    #[must_use]
    pub fn to_compact(&self) -> String {
        use std::fmt::Write;
        let mut output = String::with_capacity(80);
        let _ = write!(
            output,
            "[{}] lines={} err={} warn={} info={}",
            format_time_hms(),
            self.total_lines,
            self.log_levels.error,
            self.log_levels.warn,
            self.log_levels.info
        );
        if let Some(peers) = self.last_peer_count {
            let _ = write!(output, " peers={peers}");
        }
        if let Some(slot) = self.last_slot {
            let _ = write!(output, " slot={slot}");
        }
        output
    }

    /// Merge statistics from another Stats instance into this one.
    pub fn merge(&mut self, other: &Stats) {
        self.total_lines += other.total_lines;
        self.matched_lines += other.matched_lines;
        self.skipped_lines += other.skipped_lines;

        self.log_levels.error += other.log_levels.error;
        self.log_levels.warn += other.log_levels.warn;
        self.log_levels.info += other.log_levels.info;
        self.log_levels.debug += other.log_levels.debug;
        self.log_levels.trace += other.log_levels.trace;

        if self.first_timestamp.is_none() {
            self.first_timestamp.clone_from(&other.first_timestamp);
        }

        if other.last_timestamp.is_some() {
            self.last_timestamp.clone_from(&other.last_timestamp);
        }

        for (msg, count) in &other.top_errors {
            *self.top_errors.entry(msg.clone()).or_insert(0) += count;
        }
    }
}
