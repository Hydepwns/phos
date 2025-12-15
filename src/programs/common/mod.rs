//! Common rule patterns shared across programs.
//!
//! Provides reusable rule builders for common patterns like log levels,
//! IP addresses, timestamps, and size metrics.

pub mod containers;
pub mod database;
pub mod development;
pub mod identifiers;
pub mod lifecycle;
pub mod log_levels;
pub mod metrics;
pub mod network;
pub mod time;

// Re-export everything for backward compatibility
pub use containers::*;
pub use database::*;
pub use development::*;
pub use identifiers::*;
pub use lifecycle::*;
pub use log_levels::*;
pub use metrics::*;
pub use network::*;
pub use time::*;
