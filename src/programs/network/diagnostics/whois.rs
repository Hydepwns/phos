//! WHOIS domain lookup colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::programs::common;
use crate::rule::Rule;

fn whois_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Field labels
    rules.push(
        Rule::new(r"^[A-Za-z][A-Za-z\s\-/]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Important fields
    rules.extend([
        Rule::new(
            r"^(Domain Name|Registrar|Registry Domain ID|Registrant|Admin|Tech|Name Server):",
        )
        .unwrap()
        .semantic(SemanticColor::Key)
        .bold()
        .build(),
        Rule::new(r"^(Creation Date|Updated Date|Registry Expiry Date|Expiration Date):")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Domain names
    rules.push(
        Rule::new(r"\b[\w\-]+\.(com|net|org|io|dev|edu|gov|co\.\w+|uk|de|fr|eu)\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // CIDR blocks
    rules.push(
        Rule::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}/\d{1,2}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Dates
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}Z?)?")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Email addresses
    rules.push(
        Rule::new(r"\b[\w\.\-]+@[\w\.\-]+\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
    );

    // URLs
    rules.push(
        Rule::new(r"https?://[\w\.\-/]+")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
    );

    // Status values
    rules.extend([
        Rule::new(r"\b(clientTransferProhibited|clientDeleteProhibited|clientUpdateProhibited)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(ok|active)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(redemptionPeriod|pendingDelete)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Organization/Company names
    rules.push(
        Rule::new(r"\b(LLC|Inc\.|Corp\.|Ltd\.?|GmbH|S\.A\.|Co\.)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // AS numbers
    rules.push(
        Rule::new(r"\bAS\d+\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Notice/disclaimer sections
    rules.push(
        Rule::new(r"^%.*$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn whois_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "network.whois",
            "whois",
            "WHOIS domain lookup output",
            Category::Network,
            whois_rules(),
        )
        .with_detect_patterns(vec!["whois"]),
    )
}
