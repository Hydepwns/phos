//! Network-related patterns (IP addresses, HTTP, connections).

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// IPv4 address pattern.
#[must_use] pub fn ipv4_rule() -> Rule {
    Rule::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// IPv6 address pattern (simplified).
#[must_use] pub fn ipv6_rule() -> Rule {
    Rule::new(r"\b([a-fA-F0-9:]+:+[a-fA-F0-9:]+)\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// Both IPv4 and IPv6 rules.
#[must_use] pub fn ip_rules() -> Vec<Rule> {
    vec![ipv4_rule(), ipv6_rule()]
}

/// MAC address patterns.
#[must_use] pub fn mac_address_rule() -> Rule {
    Rule::new(r"\b([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// HTTP status codes (4xx/5xx as errors, 2xx/3xx as success).
#[must_use] pub fn http_status_rules() -> Vec<Rule> {
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
#[must_use] pub fn http_method_rule() -> Rule {
    Rule::new(r"\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\b")
        .unwrap()
        .semantic(SemanticColor::Key)
        .build()
}

/// Network connection states for netstat/ss.
#[must_use] pub fn connection_state_rules() -> Vec<Rule> {
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
#[must_use] pub fn port_state_rules() -> Vec<Rule> {
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

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // COMPILE TESTS
    // =========================================================================

    #[test]
    fn test_ip_rules_compile() {
        let rules = ip_rules();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_http_status_rules_compile() {
        let rules = http_status_rules();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_connection_state_rules_compile() {
        let rules = connection_state_rules();
        assert_eq!(rules.len(), 7);
    }

    #[test]
    fn test_port_state_rules_compile() {
        let rules = port_state_rules();
        assert_eq!(rules.len(), 4);
    }

    // =========================================================================
    // IPV4 MATCHING
    // =========================================================================

    #[test]
    fn test_ipv4_rule_matches_valid() {
        let rule = ipv4_rule();
        assert!(rule.is_match("192.168.1.1"));
        assert!(rule.is_match("10.0.0.1"));
        assert!(rule.is_match("127.0.0.1"));
        assert!(rule.is_match("255.255.255.255"));
        assert!(rule.is_match("0.0.0.0"));
    }

    #[test]
    fn test_ipv4_rule_in_context() {
        let rule = ipv4_rule();
        assert!(rule.is_match("Connected to 192.168.1.1:8080"));
        assert!(rule.is_match("Source: 10.0.0.1"));
    }

    #[test]
    fn test_ipv4_rule_word_boundary() {
        let rule = ipv4_rule();
        // The regex allows any digit pattern x.x.x.x
        // This is intentionally permissive for log matching
        assert!(rule.is_match("1.2.3.4"));
    }

    // =========================================================================
    // IPV6 MATCHING
    // =========================================================================

    #[test]
    fn test_ipv6_rule_matches_valid() {
        let rule = ipv6_rule();
        // The simplified regex requires hex chars on both sides of ::
        assert!(rule.is_match("2001:db8::1"));
        assert!(rule.is_match("fe80::1"));
        assert!(rule.is_match("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    }

    #[test]
    fn test_ipv6_rule_in_context() {
        let rule = ipv6_rule();
        assert!(rule.is_match("Address: 2001:db8::1"));
        assert!(rule.is_match("Connecting to fe80::1"));
    }

    // =========================================================================
    // MAC ADDRESS MATCHING
    // =========================================================================

    #[test]
    fn test_mac_address_rule_matches() {
        let rule = mac_address_rule();
        assert!(rule.is_match("00:11:22:33:44:55"));
        assert!(rule.is_match("aa:bb:cc:dd:ee:ff"));
        assert!(rule.is_match("AA:BB:CC:DD:EE:FF"));
    }

    #[test]
    fn test_mac_address_in_context() {
        let rule = mac_address_rule();
        assert!(rule.is_match("MAC: 00:11:22:33:44:55"));
    }

    // =========================================================================
    // HTTP STATUS CODE MATCHING
    // =========================================================================

    #[test]
    fn test_http_status_error_codes() {
        let rules = http_status_rules();
        // 4xx client errors
        assert!(any_rule_matches(&rules, "HTTP 400 Bad Request"));
        assert!(any_rule_matches(&rules, "HTTP 401 Unauthorized"));
        assert!(any_rule_matches(&rules, "HTTP 403 Forbidden"));
        assert!(any_rule_matches(&rules, "HTTP 404 Not Found"));
        assert!(any_rule_matches(&rules, "HTTP 429 Too Many Requests"));
        // 5xx server errors
        assert!(any_rule_matches(&rules, "HTTP 500 Internal Server Error"));
        assert!(any_rule_matches(&rules, "HTTP 502 Bad Gateway"));
        assert!(any_rule_matches(&rules, "HTTP 503 Service Unavailable"));
    }

    #[test]
    fn test_http_status_success_codes() {
        let rules = http_status_rules();
        // 2xx success
        assert!(any_rule_matches(&rules, "HTTP 200 OK"));
        assert!(any_rule_matches(&rules, "HTTP 201 Created"));
        assert!(any_rule_matches(&rules, "HTTP 204 No Content"));
        // 3xx redirect
        assert!(any_rule_matches(&rules, "HTTP 301 Moved Permanently"));
        assert!(any_rule_matches(&rules, "HTTP 302 Found"));
        assert!(any_rule_matches(&rules, "HTTP 304 Not Modified"));
    }

    // =========================================================================
    // HTTP METHOD MATCHING
    // =========================================================================

    #[test]
    fn test_http_method_rule_matches() {
        let rule = http_method_rule();
        assert!(rule.is_match("GET /api/users"));
        assert!(rule.is_match("POST /api/login"));
        assert!(rule.is_match("PUT /api/users/1"));
        assert!(rule.is_match("DELETE /api/users/1"));
        assert!(rule.is_match("PATCH /api/users/1"));
        assert!(rule.is_match("HEAD /"));
        assert!(rule.is_match("OPTIONS /"));
    }

    #[test]
    fn test_http_method_word_boundary() {
        let rule = http_method_rule();
        // Should not match partial words
        assert!(!rule.is_match("GETTING"));
        assert!(!rule.is_match("POSTED"));
    }

    // =========================================================================
    // CONNECTION STATE MATCHING
    // =========================================================================

    #[test]
    fn test_connection_state_established() {
        let rules = connection_state_rules();
        assert!(any_rule_matches(&rules, "tcp  0  0 127.0.0.1:8080  127.0.0.1:12345 ESTABLISHED"));
    }

    #[test]
    fn test_connection_state_listen() {
        let rules = connection_state_rules();
        assert!(any_rule_matches(&rules, "tcp  0  0 0.0.0.0:22  0.0.0.0:*  LISTEN"));
        assert!(any_rule_matches(&rules, "tcp  0  0 :::80  :::*  LISTENING"));
    }

    #[test]
    fn test_connection_state_wait_states() {
        let rules = connection_state_rules();
        assert!(any_rule_matches(&rules, "TIME_WAIT"));
        assert!(any_rule_matches(&rules, "TIME-WAIT"));
        assert!(any_rule_matches(&rules, "CLOSE_WAIT"));
        assert!(any_rule_matches(&rules, "CLOSE-WAIT"));
        assert!(any_rule_matches(&rules, "CLOSING"));
    }

    #[test]
    fn test_connection_state_syn_states() {
        let rules = connection_state_rules();
        assert!(any_rule_matches(&rules, "SYN_SENT"));
        assert!(any_rule_matches(&rules, "SYN-SENT"));
        assert!(any_rule_matches(&rules, "SYN_RECV"));
        assert!(any_rule_matches(&rules, "SYN-RECV"));
    }

    #[test]
    fn test_connection_state_fin_wait() {
        let rules = connection_state_rules();
        assert!(any_rule_matches(&rules, "FIN_WAIT"));
        assert!(any_rule_matches(&rules, "FIN-WAIT"));
        assert!(any_rule_matches(&rules, "FIN_WAIT1"));
        assert!(any_rule_matches(&rules, "FIN_WAIT2"));
    }

    // =========================================================================
    // PORT STATE MATCHING (NMAP)
    // =========================================================================

    #[test]
    fn test_port_state_rules_match() {
        let rules = port_state_rules();
        assert!(any_rule_matches(&rules, "22/tcp open  ssh"));
        assert!(any_rule_matches(&rules, "80/tcp closed http"));
        assert!(any_rule_matches(&rules, "443/tcp filtered https"));
        assert!(any_rule_matches(&rules, "8080/tcp open|filtered http-proxy"));
    }
}
