//! Helm package manager colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn helm_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Release status
    rules.extend([
        Rule::new(r"\bSTATUS:\s*deployed\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(failed|superseded)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(pending|pending-install|pending-upgrade|pending-rollback)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bSTATUS:\s*uninstalled\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Chart info
    rules.extend([
        Rule::new(r"\bCHART:\s*[\w\-]+[\d\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bNAME:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bNAMESPACE:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bREVISION:\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Hooks and resources
    rules.extend([
        Rule::new(
            r"\b(pre-install|post-install|pre-upgrade|post-upgrade|pre-delete|post-delete)\b",
        )
        .unwrap()
        .semantic(SemanticColor::Key)
        .build(),
        Rule::new(r"created|configured|unchanged")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn helm_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.helm",
            "helm",
            "Helm package manager output",
            Category::DevOps,
            helm_rules(),
        )
        .with_detect_patterns(vec!["helm"]),
    )
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
    fn test_helm_rules_compile() {
        let rules = helm_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_helm_program_info() {
        let program = helm_program();
        assert_eq!(program.info().id, "devops.helm");
        assert_eq!(program.info().name, "helm");
    }

    // =========================================================================
    // RELEASE STATUS MATCHING
    // =========================================================================

    #[test]
    fn test_status_deployed() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "STATUS: deployed"));
    }

    #[test]
    fn test_status_failed() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "STATUS: failed"));
        assert!(any_rule_matches(&rules, "STATUS: superseded"));
    }

    #[test]
    fn test_status_pending() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "STATUS: pending"));
        assert!(any_rule_matches(&rules, "STATUS: pending-install"));
        assert!(any_rule_matches(&rules, "STATUS: pending-upgrade"));
        assert!(any_rule_matches(&rules, "STATUS: pending-rollback"));
    }

    #[test]
    fn test_status_uninstalled() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "STATUS: uninstalled"));
    }

    // =========================================================================
    // CHART INFO MATCHING
    // =========================================================================

    #[test]
    fn test_chart_info() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "CHART: nginx-1.2.3"));
        assert!(any_rule_matches(&rules, "CHART: prometheus-15.0.1"));
    }

    #[test]
    fn test_name_info() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "NAME: my-release"));
        assert!(any_rule_matches(&rules, "NAME: nginx-ingress"));
    }

    #[test]
    fn test_namespace_info() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "NAMESPACE: default"));
        assert!(any_rule_matches(&rules, "NAMESPACE: kube-system"));
    }

    #[test]
    fn test_revision_info() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "REVISION: 1"));
        assert!(any_rule_matches(&rules, "REVISION: 42"));
    }

    // =========================================================================
    // HOOKS MATCHING
    // =========================================================================

    #[test]
    fn test_hooks() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "pre-install"));
        assert!(any_rule_matches(&rules, "post-install"));
        assert!(any_rule_matches(&rules, "pre-upgrade"));
        assert!(any_rule_matches(&rules, "post-upgrade"));
        assert!(any_rule_matches(&rules, "pre-delete"));
        assert!(any_rule_matches(&rules, "post-delete"));
    }

    // =========================================================================
    // RESOURCE STATUS MATCHING
    // =========================================================================

    #[test]
    fn test_resource_status() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "deployment.apps/nginx created"));
        assert!(any_rule_matches(&rules, "service/nginx configured"));
        assert!(any_rule_matches(&rules, "configmap/nginx unchanged"));
    }

    // =========================================================================
    // LOG LEVELS
    // =========================================================================

    #[test]
    fn test_log_levels() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "ERROR: release not found"));
        assert!(any_rule_matches(&rules, "WARNING: chart deprecated"));
    }

    // =========================================================================
    // REAL OUTPUT EXAMPLES
    // =========================================================================

    #[test]
    fn test_helm_install_output() {
        let rules = helm_rules();
        assert!(any_rule_matches(&rules, "NAME: my-nginx"));
        assert!(any_rule_matches(&rules, "NAMESPACE: default"));
        assert!(any_rule_matches(&rules, "STATUS: deployed"));
        assert!(any_rule_matches(&rules, "REVISION: 1"));
    }

    #[test]
    fn test_helm_list_output() {
        let rules = helm_rules();
        let line = "my-release  default  1  2024-01-15 10:30:45  deployed  nginx-1.0.0";
        assert!(any_rule_matches(&rules, line));
    }
}
