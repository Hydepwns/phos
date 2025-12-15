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
