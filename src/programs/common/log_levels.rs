//! Log level patterns for various formats.

use crate::colors::SemanticColor;
use crate::rule::Rule;

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
}
