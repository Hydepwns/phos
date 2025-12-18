//! Server lifecycle patterns (startup, ready, shutdown).

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Server ready patterns - matches "ready for connections", "Ready to accept connections",
/// "Waiting for connections", "listening", etc.
#[must_use]
pub fn server_ready_rule() -> Rule {
    Rule::new(r"(?i)\b(ready (for|to accept) connections|waiting for connections|listening on)\b")
        .unwrap()
        .semantic(SemanticColor::Success)
        .bold()
        .build()
}

/// Server initialized patterns - matches "initialized", "Server initialized", "started".
#[must_use]
pub fn server_initialized_rule() -> Rule {
    Rule::new(r"(?i)\b(server\s+)?(initialized|started)\b")
        .unwrap()
        .semantic(SemanticColor::Success)
        .build()
}

/// Server shutdown patterns - matches "shutdown", "shutting down", "stopping".
#[must_use]
pub fn server_shutdown_rule() -> Rule {
    Rule::new(r"(?i)\b(shut\s*down|shutting\s+down|stopping)\b")
        .unwrap()
        .semantic(SemanticColor::Warn)
        .build()
}

/// Shutdown complete pattern - for "Shutdown complete", "Shutdown completed".
#[must_use]
pub fn server_shutdown_complete_rule() -> Rule {
    Rule::new(r"(?i)\bshutdown\s+complet(e|ed)\b")
        .unwrap()
        .semantic(SemanticColor::Info)
        .build()
}

/// Common server lifecycle rules.
/// Returns rules for ready, initialized, shutdown complete, and shutdown states.
#[must_use]
pub fn server_lifecycle_rules() -> Vec<Rule> {
    vec![
        server_ready_rule(),
        server_initialized_rule(),
        server_shutdown_complete_rule(),
        server_shutdown_rule(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_rules_compile() {
        let rules = server_lifecycle_rules();
        assert_eq!(rules.len(), 4);
    }

    #[test]
    fn test_ready_rule_matches() {
        let rule = server_ready_rule();
        assert!(rule.regex.is_match("ready for connections"));
        assert!(rule.regex.is_match("Ready to accept connections"));
        assert!(rule.regex.is_match("Waiting for connections"));
        assert!(rule.regex.is_match("listening on port 5432"));
    }

    #[test]
    fn test_shutdown_rule_matches() {
        let rule = server_shutdown_rule();
        assert!(rule.regex.is_match("shutting down"));
        assert!(rule.regex.is_match("Shutdown"));
        assert!(rule.regex.is_match("stopping"));
    }
}
