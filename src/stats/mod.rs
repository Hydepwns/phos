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

#![allow(clippy::format_push_string)]

mod collector;
mod core;
mod export;
mod helpers;
mod patterns;

// Re-export public types
pub use collector::StatsCollector;
pub use core::{LogLevelCounts, Stats};
pub use export::{
    ErrorJson, EthereumJson, LogLevelsJson, ProcessingJson, StatsExportFormat, StatsJson,
    TimeRangeJson,
};
pub use patterns::{StatsPatterns, STATS_PATTERNS};

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
        assert_eq!(
            stats.first_timestamp,
            Some("2024-01-15T10:30:45".to_string())
        );
        assert_eq!(
            stats.last_timestamp,
            Some("2024-01-15T10:30:50".to_string())
        );
    }

    #[test]
    fn test_syslog_timestamp() {
        let mut collector = StatsCollector::new();

        collector.process_line("Jan 15 10:30:45 hostname service: INFO message", true);

        let stats = collector.stats();
        assert!(stats.first_timestamp.is_some());
    }
}
