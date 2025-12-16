//! `HAProxy` load balancer colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::programs::common;
use crate::rule::Rule;

fn haproxy_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Syslog timestamp
    rules.push(common::syslog_timestamp_rule());

    // Process identifier
    rules.push(
        Rule::new(r"\bhaproxy\[\d+\]:")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // HTTP status codes
    rules.extend(common::http_status_rules());

    // Frontend/backend names
    rules.extend([
        Rule::new(r"\b\w+/\w+/\w+\s+\d+/\d+/\d+/\d+/\d+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bfe:\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bbe:\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Termination state
    rules.extend([
        Rule::new(r"\b[CSPLRI][CDRSKQPLHI][-NIDVEC][-NICUR]\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b--\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Connection counts
    rules.push(
        Rule::new(r"\b\d+/\d+/\d+/\d+/\d+\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Server states
    rules.extend([
        Rule::new(r"\bUP\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bDOWN\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bNOLB\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bMAINT\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Health checks
    rules.extend([
        Rule::new(r"\bHealth check\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bL[47](OK|TOUT|CON|RSP)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Warnings/errors
    rules.extend([
        Rule::new(r"\[WARNING\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[ALERT\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bno server available\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

pub fn haproxy_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.haproxy",
            "haproxy",
            "HAProxy load balancer logs",
            Category::Network,
            haproxy_rules(),
        )
        .with_detect_patterns(vec!["haproxy"]),
    )
}
