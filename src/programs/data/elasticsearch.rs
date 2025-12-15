//! Elasticsearch cluster colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn elasticsearch_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamp in brackets (Log4j format)
    rules.push(common::log4j_timestamp_rule());

    // Log levels
    rules.extend([
        Rule::new(r"\[FATAL\s*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[ERROR\s*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[WARN\s*\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[INFO\s*\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[DEBUG\s*\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\[TRACE\s*\]")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]);

    // Component/logger names in brackets
    rules.push(
        Rule::new(r"\[[\w\.]+\s*\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Node info
    rules.extend([
        Rule::new(r"\[node-\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bnode\s*\{[^}]+\}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Cluster health
    rules.extend([
        Rule::new(r"\bstatus\s*\[green\]")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bstatus\s*\[yellow\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bstatus\s*\[red\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
    ]);

    // Index operations
    rules.extend([
        Rule::new(r"\bindex\s*\[[^\]]+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bshard\s*\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(creating|closing|deleting)\s+index\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Cluster events
    rules.extend([
        Rule::new(r"\bcluster state updated\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bmaster node changed\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bnew master\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
    ]);

    // Server lifecycle
    rules.extend(common::server_lifecycle_rules());

    // GC logs
    rules.extend([
        Rule::new(r"\[gc\]\[\w+\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"GC overhead")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

pub fn elasticsearch_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "data.elasticsearch",
            "elasticsearch",
            "Elasticsearch cluster logs",
            Category::Data,
            elasticsearch_rules(),
        )
        .with_detect_patterns(vec!["elasticsearch", "elastic"]),
    )
}
