//! Ansible playbook colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn ansible_rules() -> Vec<Rule> {
    vec![
        // PLAY and TASK headers
        Rule::new(r"^PLAY\s+\[[^\]]*\]\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^TASK\s+\[[^\]]*\]\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        Rule::new(r"^PLAY RECAP\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Task status
        Rule::new(r"^ok:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^changed:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^failed:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^skipping:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^unreachable:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^fatal:\s*\[[^\]]*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        // Recap summary counts
        Rule::new(r"\bok=\d+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bchanged=\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bunreachable=\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bfailed=\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bskipped=\d+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\brescued=\d+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bignored=\d+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Host IPs/names in recap
        common::ipv4_rule(),
        common::number_rule(),
    ]
}

#[must_use] pub fn ansible_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.ansible",
            "ansible",
            "Ansible playbook output",
            Category::DevOps,
            ansible_rules(),
        )
        .with_detect_patterns(vec!["ansible", "ansible-playbook"]),
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
    fn test_ansible_rules_compile() {
        let rules = ansible_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_ansible_program_info() {
        let program = ansible_program();
        assert_eq!(program.info().id, "devops.ansible");
        assert_eq!(program.info().name, "ansible");
    }

    // =========================================================================
    // PLAY AND TASK HEADERS
    // =========================================================================

    #[test]
    fn test_play_header() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "PLAY [Install nginx] ****"));
        assert!(any_rule_matches(
            &rules,
            "PLAY [Configure webservers] *******************************"
        ));
    }

    #[test]
    fn test_task_header() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "TASK [Install packages] ****"));
        assert!(any_rule_matches(
            &rules,
            "TASK [Copy configuration] ***********************"
        ));
    }

    #[test]
    fn test_play_recap_header() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "PLAY RECAP *****"));
        assert!(any_rule_matches(
            &rules,
            "PLAY RECAP *******************************************"
        ));
    }

    // =========================================================================
    // TASK STATUS MATCHING
    // =========================================================================

    #[test]
    fn test_task_ok() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "ok: [webserver1]"));
        assert!(any_rule_matches(&rules, "ok: [192.168.1.10]"));
    }

    #[test]
    fn test_task_changed() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "changed: [webserver1]"));
        assert!(any_rule_matches(&rules, "changed: [db-server]"));
    }

    #[test]
    fn test_task_failed() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "failed: [webserver1]"));
        assert!(any_rule_matches(&rules, "failed: [app-server]"));
    }

    #[test]
    fn test_task_skipping() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "skipping: [webserver1]"));
        assert!(any_rule_matches(&rules, "skipping: [all]"));
    }

    #[test]
    fn test_task_unreachable() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "unreachable: [webserver1]"));
    }

    #[test]
    fn test_task_fatal() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "fatal: [webserver1]"));
        assert!(any_rule_matches(&rules, "fatal: [db-master]"));
    }

    // =========================================================================
    // RECAP SUMMARY MATCHING
    // =========================================================================

    #[test]
    fn test_recap_ok() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "ok=5"));
        assert!(any_rule_matches(&rules, "ok=10"));
    }

    #[test]
    fn test_recap_changed() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "changed=2"));
        assert!(any_rule_matches(&rules, "changed=0"));
    }

    #[test]
    fn test_recap_unreachable() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "unreachable=0"));
        assert!(any_rule_matches(&rules, "unreachable=1"));
    }

    #[test]
    fn test_recap_failed() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "failed=0"));
        assert!(any_rule_matches(&rules, "failed=3"));
    }

    #[test]
    fn test_recap_skipped() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "skipped=2"));
    }

    #[test]
    fn test_recap_rescued() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "rescued=1"));
    }

    #[test]
    fn test_recap_ignored() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "ignored=0"));
    }

    // =========================================================================
    // IP ADDRESS MATCHING
    // =========================================================================

    #[test]
    fn test_ip_addresses() {
        let rules = ansible_rules();
        assert!(any_rule_matches(&rules, "192.168.1.100"));
        assert!(any_rule_matches(&rules, "10.0.0.5"));
    }

    // =========================================================================
    // REAL OUTPUT EXAMPLES
    // =========================================================================

    #[test]
    fn test_ansible_playbook_output() {
        let rules = ansible_rules();
        assert!(any_rule_matches(
            &rules,
            "PLAY [Deploy application] ****************************************"
        ));
        assert!(any_rule_matches(
            &rules,
            "TASK [Gathering Facts] *******************************************"
        ));
        assert!(any_rule_matches(&rules, "ok: [app-server-1]"));
        assert!(any_rule_matches(&rules, "changed: [app-server-1]"));
    }

    #[test]
    fn test_ansible_recap_line() {
        let rules = ansible_rules();
        let line = "webserver1  : ok=5  changed=2  unreachable=0  failed=0  skipped=1  rescued=0  ignored=0";
        assert!(any_rule_matches(&rules, line));
    }
}
