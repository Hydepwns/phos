//! Web servers and reverse proxies.
//!
//! Provides Program implementations for nginx, caddy, apache, haproxy, and traefik.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// NGINX
// =============================================================================

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

    // Error log levels
    rules.extend([
        Rule::new(r"\[emerg\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[alert\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[crit\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[warn\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[notice\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[info\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

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

pub fn nginx_program() -> Arc<SimpleProgram> {
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

// =============================================================================
// CADDY
// =============================================================================

fn caddy_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // JSON structured log keys
    rules.extend([
        Rule::new(r#""level"\s*:\s*"error""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r#""level"\s*:\s*"warn""#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#""level"\s*:\s*"info""#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#""level"\s*:\s*"debug""#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // HTTP methods and status
    rules.push(common::http_method_rule());
    rules.extend(common::http_status_rules());

    // JSON structured fields
    rules.extend([
        Rule::new(r#""status"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""duration"\s*:\s*[\d\.]+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""size"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""ts"\s*:\s*[\d\.]+"#)
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    ]);

    // Request/response info
    rules.extend([
        Rule::new(r#""request"\s*:\s*\{[^}]+\}"#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r#""uri"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""remote_ip"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Console format (non-JSON)
    rules.extend([
        Rule::new(r"\bserving initial configuration\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\btls\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bhttp\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn caddy_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.caddy",
            "caddy",
            "Caddy web server logs",
            Category::Network,
            caddy_rules(),
        )
        .with_detect_patterns(vec!["caddy"]),
    )
}

// =============================================================================
// APACHE / HTTPD
// =============================================================================

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

pub fn apache_program() -> Arc<SimpleProgram> {
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

// =============================================================================
// HAPROXY
// =============================================================================

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

// =============================================================================
// TRAEFIK
// =============================================================================

fn traefik_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // JSON structured fields
    rules.extend([
        Rule::new(r#""level"\s*:\s*"error""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r#""level"\s*:\s*"warn""#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#""level"\s*:\s*"info""#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#""level"\s*:\s*"debug""#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // HTTP status and methods
    rules.push(common::http_method_rule());
    rules.extend(common::http_status_rules());

    // Router/service/middleware names
    rules.extend([
        Rule::new(r#""RouterName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""ServiceName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""entryPointName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Access log fields
    rules.extend([
        Rule::new(r#""RequestPath"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""ClientHost"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""Duration"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Provider events
    rules.extend([
        Rule::new(r"\bConfiguration loaded\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bServer configuration reloaded\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bprovider\s+\w+\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // TLS/ACME
    rules.extend([
        Rule::new(r"\bACME\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bLetsEncrypt\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bcertificate\s+(obtained|renewed)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Health/readiness
    rules.extend([
        Rule::new(r"\b(healthy|unhealthy)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bservers\s+\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn traefik_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.traefik",
            "traefik",
            "Traefik reverse proxy logs",
            Category::Network,
            traefik_rules(),
        )
        .with_detect_patterns(vec!["traefik"]),
    )
}
