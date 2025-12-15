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

pub fn ansible_program() -> Arc<dyn Program> {
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
