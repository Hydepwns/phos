//! Alert condition types and parsing.

use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

/// Alert condition types that trigger webhook notifications.
#[derive(Debug, Clone)]
pub enum AlertCondition {
    /// Fire on first ERROR/FATAL/PANIC/CRIT detection.
    Error,

    /// Fire when error count exceeds threshold.
    ErrorThreshold { count: usize },

    /// Fire when peer count drops below threshold.
    PeerDrop { threshold: usize },

    /// Fire when no sync progress is detected (sync stall).
    SyncStall,

    /// Fire on custom pattern match.
    Pattern { regex: Regex },
}

impl AlertCondition {
    /// Returns a string identifier for this condition type.
    pub fn condition_type(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::ErrorThreshold { .. } => "error_threshold",
            Self::PeerDrop { .. } => "peer_drop",
            Self::SyncStall => "sync_stall",
            Self::Pattern { .. } => "pattern",
        }
    }
}

/// Error parsing an alert condition from a string.
#[derive(Debug, Error)]
pub enum ParseConditionError {
    #[error("unknown condition type: {0}")]
    UnknownType(String),

    #[error("missing value for condition: {0}")]
    MissingValue(String),

    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),

    #[error("invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
}

impl FromStr for AlertCondition {
    type Err = ParseConditionError;

    /// Parse condition from CLI string format.
    ///
    /// Supported formats:
    /// - `error` -> Fire on first ERROR
    /// - `error-threshold:10` -> Fire when error count >= 10
    /// - `peer-drop:5` -> Fire when peers drop below 5
    /// - `sync-stall` -> Fire on sync stall
    /// - `pattern:FATAL|OOM` -> Fire on regex match
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Simple conditions without parameters
        if s.eq_ignore_ascii_case("error") {
            return Ok(Self::Error);
        }
        if s.eq_ignore_ascii_case("sync-stall") {
            return Ok(Self::SyncStall);
        }

        // Conditions with parameters (format: type:value)
        if let Some((cond_type, value)) = s.split_once(':') {
            let cond_type = cond_type.trim().to_lowercase();
            let value = value.trim();

            match cond_type.as_str() {
                "error-threshold" => {
                    let count = value.parse::<usize>()?;
                    Ok(Self::ErrorThreshold { count })
                }
                "peer-drop" => {
                    let threshold = value.parse::<usize>()?;
                    Ok(Self::PeerDrop { threshold })
                }
                "pattern" => {
                    let regex = Regex::new(value)?;
                    Ok(Self::Pattern { regex })
                }
                _ => Err(ParseConditionError::UnknownType(cond_type)),
            }
        } else {
            Err(ParseConditionError::UnknownType(s.to_string()))
        }
    }
}

/// Alert severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertSeverity {
    Critical,
    Error,
    #[default]
    Warning,
    Info,
}

impl AlertSeverity {
    /// Discord embed color (decimal).
    pub fn discord_color(&self) -> u32 {
        match self {
            Self::Critical => 0xFF0000, // Red
            Self::Error => 0xFF5500,    // Orange-red
            Self::Warning => 0xFFAA00,  // Yellow-orange
            Self::Info => 0x55AAFF,     // Blue
        }
    }

    /// Display string for messages.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
        }
    }

    /// Short tag for compact display.
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Critical => "[!!!]",
            Self::Error => "[ERR]",
            Self::Warning => "[WRN]",
            Self::Info => "[INF]",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_condition() {
        let cond: AlertCondition = "error".parse().unwrap();
        assert!(matches!(cond, AlertCondition::Error));

        let cond: AlertCondition = "ERROR".parse().unwrap();
        assert!(matches!(cond, AlertCondition::Error));
    }

    #[test]
    fn test_parse_error_threshold() {
        let cond: AlertCondition = "error-threshold:10".parse().unwrap();
        match cond {
            AlertCondition::ErrorThreshold { count } => assert_eq!(count, 10),
            _ => panic!("expected ErrorThreshold"),
        }

        let cond: AlertCondition = "error-threshold:100".parse().unwrap();
        match cond {
            AlertCondition::ErrorThreshold { count } => assert_eq!(count, 100),
            _ => panic!("expected ErrorThreshold"),
        }
    }

    #[test]
    fn test_parse_peer_drop() {
        let cond: AlertCondition = "peer-drop:5".parse().unwrap();
        match cond {
            AlertCondition::PeerDrop { threshold } => assert_eq!(threshold, 5),
            _ => panic!("expected PeerDrop"),
        }
    }

    #[test]
    fn test_parse_sync_stall() {
        let cond: AlertCondition = "sync-stall".parse().unwrap();
        assert!(matches!(cond, AlertCondition::SyncStall));
    }

    #[test]
    fn test_parse_pattern() {
        let cond: AlertCondition = "pattern:FATAL|OOM".parse().unwrap();
        match cond {
            AlertCondition::Pattern { regex } => {
                assert!(regex.is_match("FATAL error"));
                assert!(regex.is_match("OOM killed"));
                assert!(!regex.is_match("normal log"));
            }
            _ => panic!("expected Pattern"),
        }
    }

    #[test]
    fn test_parse_invalid() {
        assert!("unknown".parse::<AlertCondition>().is_err());
        assert!("error-threshold:abc".parse::<AlertCondition>().is_err());
        assert!("pattern:[invalid".parse::<AlertCondition>().is_err());
    }

    #[test]
    fn test_condition_type() {
        assert_eq!(AlertCondition::Error.condition_type(), "error");
        assert_eq!(
            AlertCondition::ErrorThreshold { count: 10 }.condition_type(),
            "error_threshold"
        );
        assert_eq!(
            AlertCondition::PeerDrop { threshold: 5 }.condition_type(),
            "peer_drop"
        );
        assert_eq!(AlertCondition::SyncStall.condition_type(), "sync_stall");
    }

    #[test]
    fn test_severity_colors() {
        assert_eq!(AlertSeverity::Critical.discord_color(), 0xFF0000);
        assert_eq!(AlertSeverity::Error.discord_color(), 0xFF5500);
        assert_eq!(AlertSeverity::Warning.discord_color(), 0xFFAA00);
        assert_eq!(AlertSeverity::Info.discord_color(), 0x55AAFF);
    }
}
