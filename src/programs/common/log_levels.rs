//! Log level patterns for various formats.

use once_cell::sync::Lazy;
use regex::Regex;

use crate::colors::SemanticColor;
use crate::rule::Rule;

// ---------------------------------------------------------------------------
// Shared Error Detection Pattern
// ---------------------------------------------------------------------------

/// Raw pattern string for error-level log messages.
/// Matches: ERROR, ERR, CRIT, CRITICAL, FATAL, PANIC (case-insensitive).
pub const ERROR_LEVEL_PATTERN_STR: &str = r"(?i)\b(ERROR|ERR|CRIT|CRITICAL|FATAL|PANIC)\b";

/// Compiled regex for detecting error-level log messages.
/// Used by StatsCollector and AlertConditionEvaluator.
pub static ERROR_LEVEL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(ERROR_LEVEL_PATTERN_STR).expect("ERROR_LEVEL_PATTERN regex is valid")
});

/// Standard log level rules (ERROR, WARN, INFO, DEBUG, TRACE).
/// Handles common case variations.
pub fn log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(ERROR|error|Error|ERR|err|Err)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(WARN|warn|Warn|WARNING|warning|Warning)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\b(INFO|info|Info)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(DEBUG|debug|Debug)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\b(TRACE|trace|Trace)\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Systemd/syslog priority levels (emerg, alert, crit, etc.).
pub fn syslog_priority_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(emerg|emergency)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\balert\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(crit|critical)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(err|error)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\b(warn|warning)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bnotice\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\binfo\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bdebug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

/// Structured log level patterns (level=error, level=info).
/// Common in Prometheus, Grafana, and Go applications.
pub fn structured_log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\blevel=(error|fatal|panic)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\blevel=warn(ing)?\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\blevel=info\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\blevel=debug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\blevel=trace\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Bracketed log levels (`[ERROR]`, `[WARN]`, `[INFO]`).
/// Common in databases and Java applications.
pub fn bracketed_log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\[(ERROR|FATAL|PANIC)\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[(WARN|WARNING|Warning)\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\[(INFO|INFO\s*|Note|NOTICE)\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[(DEBUG|DEBUG\d*)\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\[(TRACE|TRACE\d*)\]")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Database-style log levels (LOG, NOTICE, FATAL, PANIC).
/// Common in PostgreSQL, MySQL, and other databases.
pub fn database_log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(PANIC|FATAL):")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERROR:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bWARNING:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(NOTICE|LOG|INFO):")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\d?:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

/// JSON structured log levels ("level": "error", etc.).
/// Common in Caddy, Traefik, and modern JSON-logging applications.
pub fn json_log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r#""level"\s*:\s*"(error|fatal)""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r#""level"\s*:\s*"warn(ing)?""#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#""level"\s*:\s*"info""#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#""level"\s*:\s*"debug""#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r#""level"\s*:\s*"trace""#)
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Syslog-style bracketed log levels (`[emerg]`, `[alert]`, `[crit]`, etc.).
/// Common in nginx and other traditional Unix services.
pub fn syslog_bracketed_log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\[emerg\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[alert\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[crit\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[warn\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[notice\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[info\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_rules_compile() {
        let rules = log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_structured_log_level_rules_compile() {
        let rules = structured_log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_bracketed_log_level_rules_compile() {
        let rules = bracketed_log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_database_log_level_rules_compile() {
        let rules = database_log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_json_log_level_rules_compile() {
        let rules = json_log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_syslog_bracketed_log_level_rules_compile() {
        let rules = syslog_bracketed_log_level_rules();
        assert_eq!(rules.len(), 8);
    }
}
