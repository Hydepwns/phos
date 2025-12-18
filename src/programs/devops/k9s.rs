//! K9s Kubernetes CLI colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn k9s_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Structured log levels (level=error, level=warn, etc.)
    rules.extend(common::structured_log_level_rules());

    // Kubernetes resource names
    rules.extend([
        Rule::new(
            r"\b(pod|deployment|service|configmap|secret|namespace|node|ingress|pvc)s?/[\w\-]+",
        )
        .unwrap()
        .semantic(SemanticColor::Key)
        .build(),
        Rule::new(r"\bnamespace=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Key=value patterns (common in structured logs)
    rules.push(common::key_value_rule());

    // Timestamps
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn k9s_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.k9s",
            "k9s",
            "K9s Kubernetes CLI logs",
            Category::DevOps,
            k9s_rules(),
        )
        .with_detect_patterns(vec!["k9s"]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    #[test]
    fn test_k9s_rules_compile() {
        let rules = k9s_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_k9s_program_info() {
        let program = k9s_program();
        assert_eq!(program.info().id, "devops.k9s");
        assert_eq!(program.info().name, "k9s");
    }

    #[test]
    fn test_k9s_log_levels() {
        let rules = k9s_rules();
        assert!(any_rule_matches(
            &rules,
            "level=error msg=\"failed to connect\""
        ));
        assert!(any_rule_matches(
            &rules,
            "level=warn msg=\"pod restarting\""
        ));
        assert!(any_rule_matches(&rules, "level=info msg=\"ready\""));
    }

    #[test]
    fn test_k9s_kubernetes_resources() {
        let rules = k9s_rules();
        assert!(any_rule_matches(&rules, "pod/nginx-abc123"));
        assert!(any_rule_matches(&rules, "deployment/myapp"));
        assert!(any_rule_matches(&rules, "namespace=kube-system"));
    }

    #[test]
    fn test_k9s_timestamps() {
        let rules = k9s_rules();
        assert!(any_rule_matches(&rules, "2024-01-15T10:30:45.123Z"));
    }
}
