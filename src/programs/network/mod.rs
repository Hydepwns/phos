//! Network tool programs.
//!
//! Provides Program implementations for ping, curl, dig, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all Network programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(ping_program()));
    registry.register(Arc::new(curl_program()));
    registry.register(Arc::new(dig_program()));
    registry.register(Arc::new(nginx_program()));
    registry.register(Arc::new(caddy_program()));
    registry.register(Arc::new(apache_program()));
    registry.register(Arc::new(haproxy_program()));
    registry.register(Arc::new(traefik_program()));
}

// =============================================================================
// PING
// =============================================================================

fn ping_rules() -> Vec<Rule> {
    let mut rules = common::ip_rules();

    // Hostnames
    rules.push(
        Rule::new(r"PING\s+([\w\.\-]+)\s+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Response times (good < 50ms, moderate 50-150ms, slow > 150ms)
    rules.extend([
        Rule::new(r"time[=<]\s*([0-4]?\d(\.\d+)?)\s*ms")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"time[=<]\s*(([5-9]\d|1[0-4]\d)(\.\d+)?)\s*ms")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"time[=<]\s*(\d{3,}(\.\d+)?)\s*ms")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Packet loss
    rules.extend([
        Rule::new(r"0%\s+packet loss")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"[1-9]\d*%\s+packet loss")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    // Statistics
    rules.extend([
        Rule::new(r"(min|avg|max|mdev)\s*=\s*[\d\.]+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"icmp_seq[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"ttl[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\d+\s+bytes")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Timeout/unreachable
    rules.push(
        Rule::new(r"\b(Request timeout|Destination Host Unreachable|Network is unreachable)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

fn ping_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.ping",
        "ping",
        "Ping command output",
        "network",
        ping_rules(),
    )
    .with_detect_patterns(vec!["ping", "ping6", "fping"])
}

// =============================================================================
// CURL
// =============================================================================

fn curl_rules() -> Vec<Rule> {
    let mut rules = vec![
        // HTTP methods
        Rule::new(r"\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|TRACE)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // HTTP status codes - success (2xx)
        Rule::new(r"\bHTTP/[\d\.]+\s+2\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"< HTTP/[\d\.]+\s+2\d{2}")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // HTTP status codes - redirect (3xx)
        Rule::new(r"\bHTTP/[\d\.]+\s+3\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"< HTTP/[\d\.]+\s+3\d{2}")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // HTTP status codes - client error (4xx)
        Rule::new(r"\bHTTP/[\d\.]+\s+4\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"< HTTP/[\d\.]+\s+4\d{2}")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // HTTP status codes - server error (5xx)
        Rule::new(r"\bHTTP/[\d\.]+\s+5\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"< HTTP/[\d\.]+\s+5\d{2}")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Headers
        Rule::new(r"^>\s+[\w\-]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^<\s+[\w\-]+:")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"Content-Type:\s*[\w/\+\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // URLs
        Rule::new(r"https?://[\w\.\-/\?\&\=\%\#]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ];

    // IP addresses
    rules.push(common::ipv4_rule());

    // Timing and size
    rules.extend([
        Rule::new(r"time_[\w]+:\s*[\d\.]+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        common::size_rule(),
        common::percentage_rule(),
        Rule::new(r"\d+(\.\d+)?\s*(GB|MB|KB)/s")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Connection info and SSL
    rules.extend([
        Rule::new(r"\*\s+(Trying|Connected|Closing|Connection)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(SSL|TLS)\s+[\w\.\s]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b(certificate|cert)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

fn curl_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.curl",
        "curl",
        "Curl HTTP client output",
        "network",
        curl_rules(),
    )
    .with_detect_patterns(vec!["curl", "wget", "httpie", "http"])
}

// =============================================================================
// DIG
// =============================================================================

fn dig_rules() -> Vec<Rule> {
    let mut rules = vec![
        // Section headers
        Rule::new(r";;\s+(QUESTION|ANSWER|AUTHORITY|ADDITIONAL)\s+SECTION:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Status
        Rule::new(r"status:\s*NOERROR")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"status:\s*NXDOMAIN")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"status:\s*SERVFAIL")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"status:\s*REFUSED")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Record types
        Rule::new(r"\b(A|AAAA|CNAME|MX|NS|PTR|SOA|TXT|SRV|CAA|DNSKEY|DS|RRSIG)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ];

    // IP addresses
    rules.extend(common::ip_rules());

    // Domain names, TTL, query info
    rules.extend([
        Rule::new(r"\b[\w\-]+\.[\w\.\-]+\.\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b\d+\s+IN\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r";;\s+Query time:\s+\d+\s+msec")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r";;\s+SERVER:\s+[\d\.]+#\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r";;\s+WHEN:.*$")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r";;\s+MSG SIZE.*:\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"flags:\s+[\w\s]+;")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r";;\s+OPT PSEUDOSECTION:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

fn dig_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.dig",
        "dig",
        "DNS dig command output",
        "network",
        dig_rules(),
    )
    .with_detect_patterns(vec!["dig", "nslookup", "host", "drill"])
}

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

fn nginx_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.nginx",
        "nginx",
        "Nginx access and error logs",
        "network",
        nginx_rules(),
    )
    .with_detect_patterns(vec!["nginx", "openresty"])
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

fn caddy_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.caddy",
        "caddy",
        "Caddy web server logs",
        "network",
        caddy_rules(),
    )
    .with_detect_patterns(vec!["caddy"])
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

fn apache_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.apache",
        "apache",
        "Apache/httpd web server logs",
        "network",
        apache_rules(),
    )
    .with_detect_patterns(vec!["apache", "httpd", "apache2"])
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

fn haproxy_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.haproxy",
        "haproxy",
        "HAProxy load balancer logs",
        "network",
        haproxy_rules(),
    )
    .with_detect_patterns(vec!["haproxy"])
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

fn traefik_program() -> SimpleProgram {
    SimpleProgram::new(
        "network.traefik",
        "traefik",
        "Traefik reverse proxy logs",
        "network",
        traefik_rules(),
    )
    .with_detect_patterns(vec!["traefik"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 8);
    }

    #[test]
    fn test_ping_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("ping google.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "ping");
    }

    #[test]
    fn test_curl_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("curl -v https://api.example.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "curl");
    }

    #[test]
    fn test_dig_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("dig example.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "dig");
    }
}
