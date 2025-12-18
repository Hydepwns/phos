//! Nginx web server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::programs::common;
use crate::rule::Rule;

fn nginx_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Access log format: IP - - [timestamp] "method path" status bytes
    rules.push(common::ipv4_rule());

    // HTTP methods
    rules.push(common::http_method_rule());

    // HTTP status codes
    rules.extend(common::http_status_rules());

    // Timestamps in nginx format
    rules.push(
        Rule::new(r"\[\d{2}/\w{3}/\d{4}:\d{2}:\d{2}:\d{2}\s+[+\-]\d{4}\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Error log levels (syslog-style)
    rules.extend(common::syslog_bracketed_log_level_rules());

    // Error log timestamp format
    rules.push(
        Rule::new(r"\d{4}/\d{2}/\d{2}\s+\d{2}:\d{2}:\d{2}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Worker process/connection info
    rules.extend([
        Rule::new(r"\*\d+\s+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\d+#\d+:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // URLs and paths
    rules.push(
        Rule::new(r#""[A-Z]+\s+[^\s"]+\s+HTTP/[\d\.]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Upstream and server names
    rules.extend([
        Rule::new(r"\bupstream\s+[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bserver\s+[\w\.\-]+:\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

pub fn nginx_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "network.nginx",
            "nginx",
            "Nginx access and error logs",
            Category::Network,
            nginx_rules(),
        )
        .with_detect_patterns(vec!["nginx", "openresty"]),
    )
}
