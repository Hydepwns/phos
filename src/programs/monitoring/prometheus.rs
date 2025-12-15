//! Prometheus server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn prometheus_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Structured key=value log format
    // ts=2024-12-05T00:12:36.123Z level=info component=tsdb msg="compaction"
    rules.push(
        Rule::new(r"\bts=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.extend(common::structured_log_level_rules());

    // Component names
    rules.push(
        Rule::new(r"\bcomponent=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Message strings
    rules.push(
        Rule::new(r#"\bmsg="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // TSDB operations
    rules.extend([
        Rule::new(r"\bcompaction\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bcheckpoint\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bhead\s+GC\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bWAL\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Scraping
    rules.extend([
        Rule::new(r"\bScrape\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\btarget=[\w\-\.:]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bjob=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bscrape_pool=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Service discovery
    rules.extend([
        Rule::new(r"\bdiscovery\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bkubernetes_sd\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bfile_sd\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Alerting
    rules.extend([
        Rule::new(r"\balertmanager\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\brule_group=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\balert=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bServer is ready to receive web requests\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bLoading configuration file\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCompleted loading of configuration file\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Errors and warnings
    rules.extend([
        Rule::new(r#"\berr="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bfailed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // General key=value pattern (should be after specific ones)
    rules.push(common::key_value_rule());

    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

pub fn prometheus_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "monitoring.prometheus",
            "prometheus",
            "Prometheus server logs",
            Category::Monitoring,
            prometheus_rules(),
        )
        .with_detect_patterns(vec!["prometheus", "promtool"]),
    )
}
