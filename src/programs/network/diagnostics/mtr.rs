//! My Traceroute (mtr) colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::programs::common;
use crate::rule::Rule;

fn mtr_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Header line
    rules.push(
        Rule::new(r"^Start:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Column headers
    rules.push(
        Rule::new(r"^\s*(HOST|Loss%|Snt|Last|Avg|Best|Wrst|StDev)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // Hop numbers
    rules.push(
        Rule::new(r"^\s*\d{1,2}\.\|?--")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // Hostnames
    rules.push(
        Rule::new(r"\b[\w\-\.]+\.(com|net|org|io|dev|edu|gov|mil|co\.\w+)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Packet loss (0% is good)
    rules.push(
        Rule::new(r"\b0\.0%\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    // Packet loss (non-zero is bad)
    rules.push(
        Rule::new(r"\b[1-9]\d*\.\d%\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // 100% loss
    rules.push(
        Rule::new(r"\b100\.0%\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // Response times - good (< 50ms)
    rules.push(
        Rule::new(r"\b([0-4]?\d\.\d)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    // Response times - moderate (50-150ms)
    rules.push(
        Rule::new(r"\b([5-9]\d|1[0-4]\d)\.\d\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );

    // Response times - slow (> 150ms)
    rules.push(
        Rule::new(r"\b\d{3,}\.\d\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    );

    // Question marks for unresponsive hops
    rules.push(
        Rule::new(r"\?\?\?")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // AS numbers
    rules.push(
        Rule::new(r"\bAS\d+\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn mtr_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "network.mtr",
            "mtr",
            "My traceroute output",
            Category::Network,
            mtr_rules(),
        )
        .with_detect_patterns(vec!["mtr"]),
    )
}
