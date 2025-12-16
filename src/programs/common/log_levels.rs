//! Log level patterns for various formats.

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
/// Used by `StatsCollector` and `AlertConditionEvaluator`.
pub static ERROR_LEVEL_PATTERN: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(ERROR_LEVEL_PATTERN_STR).expect("ERROR_LEVEL_PATTERN regex is valid")
});

/// Standard log level rules (ERROR, WARN, INFO, DEBUG, TRACE).
/// Handles common case variations.
#[must_use] pub fn log_level_rules() -> Vec<Rule> {
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
#[must_use] pub fn syslog_priority_rules() -> Vec<Rule> {
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
#[must_use] pub fn structured_log_level_rules() -> Vec<Rule> {
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
#[must_use] pub fn bracketed_log_level_rules() -> Vec<Rule> {
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
/// Common in `PostgreSQL`, `MySQL`, and other databases.
#[must_use] pub fn database_log_level_rules() -> Vec<Rule> {
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
#[must_use] pub fn json_log_level_rules() -> Vec<Rule> {
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
#[must_use] pub fn syslog_bracketed_log_level_rules() -> Vec<Rule> {
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

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // COMPILE TESTS
    // =========================================================================

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

    // =========================================================================
    // STANDARD LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_log_level_rules_match_uppercase() {
        let rules = log_level_rules();
        assert!(any_rule_matches(&rules, "ERROR: something failed"));
        assert!(any_rule_matches(&rules, "WARN: warning message"));
        assert!(any_rule_matches(&rules, "INFO: informational"));
        assert!(any_rule_matches(&rules, "DEBUG: debug output"));
        assert!(any_rule_matches(&rules, "TRACE: trace level"));
    }

    #[test]
    fn test_log_level_rules_match_lowercase() {
        let rules = log_level_rules();
        assert!(any_rule_matches(&rules, "error: something failed"));
        assert!(any_rule_matches(&rules, "warn: warning message"));
        assert!(any_rule_matches(&rules, "info: informational"));
        assert!(any_rule_matches(&rules, "debug: debug output"));
        assert!(any_rule_matches(&rules, "trace: trace level"));
    }

    #[test]
    fn test_log_level_rules_match_short_forms() {
        let rules = log_level_rules();
        assert!(any_rule_matches(&rules, "ERR: error"));
        assert!(any_rule_matches(&rules, "err: error"));
    }

    #[test]
    fn test_log_level_rules_word_boundary() {
        let rules = log_level_rules();
        // Should not match partial words
        assert!(!any_rule_matches(&rules, "ERRORS"));
        assert!(!any_rule_matches(&rules, "INFOBOX"));
        assert!(!any_rule_matches(&rules, "DEBUGGER"));
    }

    // =========================================================================
    // SYSLOG PRIORITY MATCHING
    // =========================================================================

    #[test]
    fn test_syslog_priority_rules_match() {
        let rules = syslog_priority_rules();
        assert!(any_rule_matches(&rules, "emerg: system down"));
        assert!(any_rule_matches(&rules, "emergency: system down"));
        assert!(any_rule_matches(&rules, "alert: immediate action"));
        assert!(any_rule_matches(&rules, "crit: critical error"));
        assert!(any_rule_matches(&rules, "critical: critical error"));
        assert!(any_rule_matches(&rules, "err: error occurred"));
        assert!(any_rule_matches(&rules, "warn: warning"));
        assert!(any_rule_matches(&rules, "notice: notice message"));
        assert!(any_rule_matches(&rules, "info: informational"));
        assert!(any_rule_matches(&rules, "debug: debug message"));
    }

    // =========================================================================
    // STRUCTURED LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_structured_log_level_rules_match() {
        let rules = structured_log_level_rules();
        assert!(any_rule_matches(&rules, "level=error msg=\"failed\""));
        assert!(any_rule_matches(&rules, "level=fatal something crashed"));
        assert!(any_rule_matches(&rules, "level=panic unrecoverable"));
        assert!(any_rule_matches(&rules, "level=warn caution"));
        assert!(any_rule_matches(&rules, "level=warning be careful"));
        assert!(any_rule_matches(&rules, "level=info status"));
        assert!(any_rule_matches(&rules, "level=debug debugging"));
        assert!(any_rule_matches(&rules, "level=trace tracing"));
    }

    #[test]
    fn test_structured_log_level_word_boundary() {
        let rules = structured_log_level_rules();
        // Should not match without level= prefix
        assert!(!any_rule_matches(&rules, "error msg=\"failed\""));
        assert!(!any_rule_matches(&rules, "mylevel=error"));
    }

    // =========================================================================
    // BRACKETED LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_bracketed_log_level_rules_match() {
        let rules = bracketed_log_level_rules();
        assert!(any_rule_matches(&rules, "[ERROR] something failed"));
        assert!(any_rule_matches(&rules, "[FATAL] crash"));
        assert!(any_rule_matches(&rules, "[PANIC] unrecoverable"));
        assert!(any_rule_matches(&rules, "[WARN] warning"));
        assert!(any_rule_matches(&rules, "[WARNING] caution"));
        assert!(any_rule_matches(&rules, "[INFO] status"));
        assert!(any_rule_matches(&rules, "[NOTICE] notice"));
        assert!(any_rule_matches(&rules, "[DEBUG] debugging"));
        assert!(any_rule_matches(&rules, "[TRACE] tracing"));
    }

    #[test]
    fn test_bracketed_debug_with_number() {
        let rules = bracketed_log_level_rules();
        assert!(any_rule_matches(&rules, "[DEBUG1] detailed"));
        assert!(any_rule_matches(&rules, "[DEBUG2] more detailed"));
    }

    // =========================================================================
    // DATABASE LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_database_log_level_rules_match() {
        let rules = database_log_level_rules();
        assert!(any_rule_matches(&rules, "PANIC: database crashed"));
        assert!(any_rule_matches(&rules, "FATAL: connection failed"));
        assert!(any_rule_matches(&rules, "ERROR: query failed"));
        assert!(any_rule_matches(&rules, "WARNING: slow query"));
        assert!(any_rule_matches(&rules, "NOTICE: table created"));
        assert!(any_rule_matches(&rules, "LOG: checkpoint"));
        assert!(any_rule_matches(&rules, "INFO: autovacuum"));
        assert!(any_rule_matches(&rules, "DEBUG: query plan"));
    }

    #[test]
    fn test_database_log_requires_colon() {
        let rules = database_log_level_rules();
        // Should not match without colon
        assert!(!any_rule_matches(&rules, "PANIC something"));
        assert!(!any_rule_matches(&rules, "ERROR message"));
    }

    // =========================================================================
    // JSON LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_json_log_level_rules_match() {
        let rules = json_log_level_rules();
        assert!(any_rule_matches(&rules, r#"{"level": "error", "msg": "failed"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "fatal", "msg": "crash"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "warn", "msg": "caution"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "warning", "msg": "caution"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "info", "msg": "status"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "debug", "msg": "debug"}"#));
        assert!(any_rule_matches(&rules, r#"{"level": "trace", "msg": "trace"}"#));
    }

    #[test]
    fn test_json_log_level_with_spaces() {
        let rules = json_log_level_rules();
        // Should handle various spacing
        assert!(any_rule_matches(&rules, r#""level":"error""#));
        assert!(any_rule_matches(&rules, r#""level" : "error""#));
    }

    // =========================================================================
    // SYSLOG BRACKETED LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_syslog_bracketed_rules_match() {
        let rules = syslog_bracketed_log_level_rules();
        assert!(any_rule_matches(&rules, "[emerg] system down"));
        assert!(any_rule_matches(&rules, "[alert] immediate action"));
        assert!(any_rule_matches(&rules, "[crit] critical failure"));
        assert!(any_rule_matches(&rules, "[error] error occurred"));
        assert!(any_rule_matches(&rules, "[warn] warning"));
        assert!(any_rule_matches(&rules, "[notice] notice"));
        assert!(any_rule_matches(&rules, "[info] info"));
        assert!(any_rule_matches(&rules, "[debug] debug"));
    }

    // =========================================================================
    // ERROR LEVEL PATTERN (SHARED)
    // =========================================================================

    #[test]
    fn test_error_level_pattern_matches() {
        assert!(ERROR_LEVEL_PATTERN.is_match("ERROR: something failed"));
        assert!(ERROR_LEVEL_PATTERN.is_match("ERR: error"));
        assert!(ERROR_LEVEL_PATTERN.is_match("CRIT: critical"));
        assert!(ERROR_LEVEL_PATTERN.is_match("CRITICAL: very bad"));
        assert!(ERROR_LEVEL_PATTERN.is_match("FATAL: crash"));
        assert!(ERROR_LEVEL_PATTERN.is_match("PANIC: unrecoverable"));
    }

    #[test]
    fn test_error_level_pattern_case_insensitive() {
        assert!(ERROR_LEVEL_PATTERN.is_match("error: lowercase"));
        assert!(ERROR_LEVEL_PATTERN.is_match("Error: mixed"));
        assert!(ERROR_LEVEL_PATTERN.is_match("fatal: lowercase"));
    }

    #[test]
    fn test_error_level_pattern_no_false_positives() {
        // Should not match info/warn/debug
        assert!(!ERROR_LEVEL_PATTERN.is_match("INFO: info"));
        assert!(!ERROR_LEVEL_PATTERN.is_match("WARN: warning"));
        assert!(!ERROR_LEVEL_PATTERN.is_match("DEBUG: debug"));
    }
}
