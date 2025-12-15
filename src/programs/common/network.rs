//! Network-related patterns (IP addresses, HTTP, connections).

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// IPv4 address pattern.
pub fn ipv4_rule() -> Rule {
    Rule::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// IPv6 address pattern (simplified).
pub fn ipv6_rule() -> Rule {
    Rule::new(r"\b([a-fA-F0-9:]+:+[a-fA-F0-9:]+)\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// Both IPv4 and IPv6 rules.
pub fn ip_rules() -> Vec<Rule> {
    vec![ipv4_rule(), ipv6_rule()]
}

/// MAC address patterns.
pub fn mac_address_rule() -> Rule {
    Rule::new(r"\b([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// HTTP status codes (4xx/5xx as errors, 2xx/3xx as success).
pub fn http_status_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b[45]\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\b[23]\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]
}

/// HTTP methods (GET, POST, PUT, DELETE, etc.).
pub fn http_method_rule() -> Rule {
    Rule::new(r"\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\b")
        .unwrap()
        .semantic(SemanticColor::Key)
        .build()
}

/// Network connection states for netstat/ss.
pub fn connection_state_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\bESTABLISHED\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bLISTEN(ING)?\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(TIME_WAIT|TIME-WAIT)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(CLOSE_WAIT|CLOSE-WAIT|CLOSING)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(SYN_SENT|SYN-SENT|SYN_RECV|SYN-RECV)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\b(FIN_WAIT|FIN-WAIT)\d?\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bCLOSED\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

/// Port state rules for nmap.
pub fn port_state_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\bopen\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bclosed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bfiltered\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bopen\|filtered\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_rules_compile() {
        let rules = ip_rules();
        assert_eq!(rules.len(), 2);
    }
}
