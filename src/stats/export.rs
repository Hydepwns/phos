//! Export formats and JSON structures for statistics.

use clap::ValueEnum;
use serde::Serialize;

/// Export format for statistics output.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum StatsExportFormat {
    /// Human-readable summary (default)
    #[default]
    Human,
    /// JSON format for scripting and monitoring
    Json,
    /// Prometheus metrics format
    Prometheus,
}

/// JSON representation of statistics.
#[derive(Debug, Clone, Serialize)]
pub struct StatsJson {
    /// phos version
    pub version: String,
    /// Program name (if specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// Processing statistics
    pub processing: ProcessingJson,
    /// Time range of log entries
    pub time_range: TimeRangeJson,
    /// Log level counts
    pub log_levels: LogLevelsJson,
    /// Error rate as percentage
    pub error_rate: f64,
    /// Top error messages
    pub top_errors: Vec<ErrorJson>,
    /// Ethereum-specific metrics
    pub ethereum: EthereumJson,
}

/// Processing statistics for JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessingJson {
    pub total_lines: usize,
    pub matched_lines: usize,
    pub skipped_lines: usize,
    pub match_percentage: f64,
}

/// Time range for JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct TimeRangeJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
}

/// Log level counts for JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct LogLevelsJson {
    pub error: usize,
    pub warn: usize,
    pub info: usize,
    pub debug: usize,
    pub trace: usize,
}

/// Error message entry for JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct ErrorJson {
    pub message: String,
    pub count: usize,
}

/// Ethereum-specific metrics for JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct EthereumJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_peer_count: Option<usize>,
}
