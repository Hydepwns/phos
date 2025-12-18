//! Apache/httpd web server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::programs::common;
use crate::rule::Rule;

fn apache_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // IP addresses (access log)
    rules.push(common::ipv4_rule());

    // HTTP methods
    rules.push(common::http_method_rule());

    // HTTP status codes
    rules.extend(common::http_status_rules());

    // Apache access log timestamp format
    rules.push(
        Rule::new(r"\[\d{2}/\w{3}/\d{4}:\d{2}:\d{2}:\d{2}\s+[+\-]\d{4}\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Error log timestamp and levels
    rules.extend([
        Rule::new(r"\[\w{3}\s+\w{3}\s+\d{2}\s+\d{2}:\d{2}:\d{2}\.\d+\s+\d{4}\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\[[\w:]+emerg\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[[\w:]+alert\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[[\w:]+crit\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[[\w:]+error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[[\w:]+warn\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[[\w:]+notice\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[[\w:]+info\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[[\w:]+debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Module identifiers
    rules.push(
        Rule::new(r"\[(core|mpm_\w+|ssl|rewrite|proxy|auth\w*|headers):")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // PID and client info
    rules.extend([
        Rule::new(r"\[pid\s+\d+(:tid\s+\d+)?\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\[client\s+[\d\.]+:\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Request line
    rules.push(
        Rule::new(r#""[A-Z]+\s+[^\s"]+\s+HTTP/[\d\.]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Common messages
    rules.extend([
        Rule::new(r"\bAH\d{5}:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bresuming normal operations\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bcaught SIGTERM\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

pub fn apache_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "network.apache",
            "apache",
            "Apache/httpd web server logs",
            Category::Network,
            apache_rules(),
        )
        .with_detect_patterns(vec!["apache", "httpd", "apache2"]),
    )
}
