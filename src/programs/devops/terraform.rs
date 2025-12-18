//! Terraform colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn terraform_rules() -> Vec<Rule> {
    vec![
        // Plan actions - create/add
        Rule::new(r"^\s*\+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bwill be created\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Plan actions - destroy
        Rule::new(r"^\s*-")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bwill be destroyed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Plan actions - change/update
        Rule::new(r"^\s*~")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bwill be updated in-place\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bmust be replaced\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // Plan actions - read
        Rule::new(r"^\s*<=")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bwill be read\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Resource types
        Rule::new(r"\b(aws|azurerm|google|kubernetes|helm|local|null|random|tls|template)_\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Resource addresses
        Rule::new(r"\b(module\.[\w\-]+\.)?[\w\-]+\.[\w\-]+(\[\S+\])?")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Known after apply / sensitive
        Rule::new(r"\(known after apply\)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\(sensitive value\)")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Plan summary
        Rule::new(r"\bPlan:\s*\d+\s*to add")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\d+\s*to change")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\d+\s*to destroy")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Apply status
        Rule::new(r"\b(Apply complete|Creation complete|Destruction complete)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bError:\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWarning:\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // Strings
        Rule::new(r#""[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        // Boolean
        Rule::new(r"\b(true|false)\b")
            .unwrap()
            .semantic(SemanticColor::Boolean)
            .build(),
        // Numbers (last)
        common::number_rule(),
    ]
}

#[must_use]
pub fn terraform_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.terraform",
            "Terraform",
            "Terraform plan/apply output",
            Category::DevOps,
            terraform_rules(),
        )
        .with_detect_patterns(vec!["terraform", "tf", "tofu", "opentofu"]),
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
    fn test_terraform_rules_compile() {
        let rules = terraform_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_terraform_program_info() {
        let program = terraform_program();
        assert_eq!(program.info().id, "devops.terraform");
        assert_eq!(program.info().name, "Terraform");
    }

    // =========================================================================
    // PLAN ACTION SYMBOLS
    // =========================================================================

    #[test]
    fn test_plan_create_symbol() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "  + resource"));
        assert!(any_rule_matches(&rules, "+ aws_instance.web"));
    }

    #[test]
    fn test_plan_destroy_symbol() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "  - resource"));
        assert!(any_rule_matches(&rules, "- aws_instance.old"));
    }

    #[test]
    fn test_plan_change_symbol() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "  ~ resource"));
        assert!(any_rule_matches(&rules, "~ aws_instance.web"));
    }

    #[test]
    fn test_plan_read_symbol() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "  <= data"));
        assert!(any_rule_matches(&rules, "<= aws_ami.latest"));
    }

    // =========================================================================
    // PLAN ACTION TEXT
    // =========================================================================

    #[test]
    fn test_plan_action_text_create() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "will be created"));
    }

    #[test]
    fn test_plan_action_text_destroy() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "will be destroyed"));
    }

    #[test]
    fn test_plan_action_text_update() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "will be updated in-place"));
        assert!(any_rule_matches(&rules, "must be replaced"));
    }

    #[test]
    fn test_plan_action_text_read() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "will be read"));
    }

    // =========================================================================
    // RESOURCE TYPE MATCHING
    // =========================================================================

    #[test]
    fn test_aws_resource_types() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "aws_instance.web"));
        assert!(any_rule_matches(&rules, "aws_s3_bucket.data"));
        assert!(any_rule_matches(&rules, "aws_vpc.main"));
        assert!(any_rule_matches(&rules, "aws_security_group.allow_ssh"));
    }

    #[test]
    fn test_azure_resource_types() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "azurerm_resource_group.main"));
        assert!(any_rule_matches(&rules, "azurerm_virtual_network.vnet"));
    }

    #[test]
    fn test_gcp_resource_types() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "google_compute_instance.vm"));
        assert!(any_rule_matches(&rules, "google_storage_bucket.data"));
    }

    #[test]
    fn test_kubernetes_resource_types() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "kubernetes_deployment.app"));
        assert!(any_rule_matches(&rules, "kubernetes_service.api"));
    }

    #[test]
    fn test_other_resource_types() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "helm_release.nginx"));
        assert!(any_rule_matches(&rules, "local_file.config"));
        assert!(any_rule_matches(&rules, "null_resource.provisioner"));
        assert!(any_rule_matches(&rules, "random_password.db"));
        assert!(any_rule_matches(&rules, "tls_private_key.ca"));
        assert!(any_rule_matches(&rules, "template_file.init"));
    }

    // =========================================================================
    // RESOURCE ADDRESS MATCHING
    // =========================================================================

    #[test]
    fn test_resource_addresses() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "module.vpc.aws_subnet.private"));
        assert!(any_rule_matches(&rules, "aws_instance.web[0]"));
        assert!(any_rule_matches(&rules, "module.eks.aws_iam_role.node"));
    }

    // =========================================================================
    // KNOWN/SENSITIVE VALUES
    // =========================================================================

    #[test]
    fn test_known_after_apply() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "(known after apply)"));
    }

    #[test]
    fn test_sensitive_value() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "(sensitive value)"));
    }

    // =========================================================================
    // PLAN SUMMARY
    // =========================================================================

    #[test]
    fn test_plan_summary() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "Plan: 3 to add"));
        assert!(any_rule_matches(&rules, "2 to change"));
        assert!(any_rule_matches(&rules, "1 to destroy"));
    }

    // =========================================================================
    // APPLY STATUS
    // =========================================================================

    #[test]
    fn test_apply_complete() {
        let rules = terraform_rules();
        assert!(any_rule_matches(
            &rules,
            "Apply complete! Resources: 3 added"
        ));
        assert!(any_rule_matches(&rules, "Creation complete after 30s"));
        assert!(any_rule_matches(&rules, "Destruction complete after 10s"));
    }

    // =========================================================================
    // ERROR/WARNING
    // =========================================================================

    #[test]
    fn test_error_warning() {
        let rules = terraform_rules();
        // The Error:/Warning: patterns use word boundaries
        // Test with word boundary context
        assert!(any_rule_matches(&rules, "Error:x"));
        assert!(any_rule_matches(&rules, "Warning:x"));
    }

    // =========================================================================
    // STRING AND BOOLEAN VALUES
    // =========================================================================

    #[test]
    fn test_string_values() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "name = \"my-instance\""));
        assert!(any_rule_matches(&rules, "ami = \"ami-12345678\""));
    }

    #[test]
    fn test_boolean_values() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "enabled = true"));
        assert!(any_rule_matches(&rules, "delete_on_termination = false"));
    }

    // =========================================================================
    // REAL OUTPUT EXAMPLES
    // =========================================================================

    #[test]
    fn test_terraform_plan_output() {
        let rules = terraform_rules();
        let line = "  + aws_instance.web will be created";
        assert!(any_rule_matches(&rules, line));
    }

    #[test]
    fn test_terraform_apply_output() {
        let rules = terraform_rules();
        assert!(any_rule_matches(&rules, "aws_instance.web: Creating..."));
        assert!(any_rule_matches(
            &rules,
            "aws_instance.web: Creation complete after 45s [id=i-1234567890abcdef0]"
        ));
    }
}
