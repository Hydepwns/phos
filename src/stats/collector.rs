//! Stats collector that wraps around a colorizer.

use std::io::{self, Write};

use super::core::Stats;
use super::export::{StatsExportFormat, StatsJson};
use super::patterns::STATS_PATTERNS;

/// Stats collector that wraps around a colorizer.
///
/// Uses the global `STATS_PATTERNS` for regex matching, avoiding
/// repeated compilation of patterns for each collector instance.
pub struct StatsCollector {
    stats: Stats,
}

impl StatsCollector {
    /// Create a new stats collector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stats: Stats::new(),
        }
    }

    /// Process a line and return whether it had matches (for external colorizer to use).
    pub fn process_line(&mut self, line: &str, had_match: bool) {
        self.stats.process_line(line, &STATS_PATTERNS, had_match);
    }

    /// Record that a line was skipped by a skip rule.
    pub fn record_skipped(&mut self) {
        self.stats.skipped_lines += 1;
    }

    /// Get the collected stats.
    #[must_use]
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

    /// Get the current error count (for alerting).
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.stats.log_levels.error
    }

    /// Get the last observed peer count (for alerting).
    #[must_use]
    pub fn peer_count(&self) -> Option<usize> {
        self.stats.last_peer_count
    }

    /// Get the last observed slot (for alerting).
    #[must_use]
    pub fn slot(&self) -> Option<u64> {
        self.stats.last_slot
    }

    /// Export statistics as JSON.
    #[must_use]
    pub fn to_json(&self, program: Option<&str>) -> StatsJson {
        self.stats.to_json(program)
    }

    /// Export statistics as Prometheus metrics format.
    #[must_use]
    pub fn to_prometheus(&self, program: Option<&str>) -> String {
        self.stats.to_prometheus(program)
    }

    /// Format statistics as a compact single-line string for interval output.
    #[must_use]
    pub fn to_compact(&self) -> String {
        self.stats.to_compact()
    }

    /// Export statistics in the specified format.
    #[must_use]
    pub fn export(&self, format: StatsExportFormat, program: Option<&str>) -> String {
        match format {
            StatsExportFormat::Human => String::new(),
            StatsExportFormat::Json => {
                serde_json::to_string_pretty(&self.to_json(program)).unwrap_or_default()
            }
            StatsExportFormat::Prometheus => self.to_prometheus(program),
        }
    }

    /// Write statistics to the given writer in the specified format.
    pub fn write_export<W: Write>(
        &self,
        writer: &mut W,
        format: StatsExportFormat,
        program: Option<&str>,
    ) -> io::Result<()> {
        match format {
            StatsExportFormat::Human => {
                self.stats.print_summary();
                Ok(())
            }
            StatsExportFormat::Json => {
                let json = serde_json::to_string_pretty(&self.to_json(program))
                    .map_err(io::Error::other)?;
                writeln!(writer, "{json}")
            }
            StatsExportFormat::Prometheus => {
                write!(writer, "{}", self.to_prometheus(program))
            }
        }
    }
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}
