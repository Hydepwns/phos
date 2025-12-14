//! Basic network diagnostic tools.
//!
//! Provides Program implementations for ping, curl, dig, traceroute, and nmap.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

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

pub fn ping_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ping",
            "ping",
            "Ping command output",
            "network",
            ping_rules(),
        )
        .with_detect_patterns(vec!["ping", "ping6", "fping"]),
    )
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

pub fn curl_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.curl",
            "curl",
            "Curl HTTP client output",
            "network",
            curl_rules(),
        )
        .with_detect_patterns(vec!["curl", "wget", "httpie"]),
    )
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

pub fn dig_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.dig",
            "dig",
            "DNS dig command output",
            "network",
            dig_rules(),
        )
        .with_detect_patterns(vec!["dig", "nslookup", "host", "drill"]),
    )
}

// =============================================================================
// TRACEROUTE
// =============================================================================

fn traceroute_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Header line
    rules.push(
        Rule::new(r"^traceroute to (\S+)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Hop numbers
    rules.push(
        Rule::new(r"^\s*\d{1,2}\s")
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

    // Response times - good (< 50ms)
    rules.push(
        Rule::new(r"\b([0-4]?\d(\.\d+)?)\s*ms\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    // Response times - moderate (50-150ms)
    rules.push(
        Rule::new(r"\b([5-9]\d|1[0-4]\d)(\.\d+)?\s*ms\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );

    // Response times - slow (> 150ms)
    rules.push(
        Rule::new(r"\b(\d{3,})(\.\d+)?\s*ms\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    );

    // Timeout/no response
    rules.push(
        Rule::new(r"\*")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // AS numbers
    rules.push(
        Rule::new(r"\[AS\d+\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn traceroute_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.traceroute",
            "traceroute",
            "Traceroute network path output",
            "network",
            traceroute_rules(),
        )
        .with_detect_patterns(vec!["traceroute", "traceroute6", "tracepath"]),
    )
}

// =============================================================================
// NMAP
// =============================================================================

fn nmap_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Scan header
    rules.push(
        Rule::new(r"^Starting Nmap")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
    );

    // Target info
    rules.push(
        Rule::new(r"^Nmap scan report for (\S+)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Host status
    rules.extend([
        Rule::new(r"Host is up")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"Host seems down")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    // Port states
    rules.extend(common::port_state_rules());

    // Service names
    rules.push(
        Rule::new(r"\b(http|https|ssh|ftp|smtp|dns|mysql|postgresql|redis|mongodb|telnet|pop3|imap)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Port numbers
    rules.push(
        Rule::new(r"\b\d{1,5}/(tcp|udp)\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // MAC addresses
    rules.push(common::mac_address_rule());

    // Latency
    rules.push(
        Rule::new(r"\(\d+(\.\d+)?s latency\)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Version detection
    rules.push(
        Rule::new(r"VERSION")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // OS detection
    rules.extend([
        Rule::new(r"OS details?:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"Running:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"Aggressive OS guesses:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Scan summary
    rules.extend([
        Rule::new(r"^Nmap done:")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\d+\s+hosts?\s+up")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"scanned in \d+(\.\d+)?\s+seconds?")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Not shown ports
    rules.push(
        Rule::new(r"Not shown: \d+ \w+ ports?")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn nmap_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.nmap",
            "nmap",
            "Nmap network scanner output",
            "network",
            nmap_rules(),
        )
        .with_detect_patterns(vec!["nmap"]),
    )
}
