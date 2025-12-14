//! Network diagnostic tools.
//!
//! Provides Program implementations for mtr, tcpdump, whois, and ntpdate.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// MTR (My Traceroute)
// =============================================================================

fn mtr_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Header line
    rules.push(
        Rule::new(r"^Start:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Column headers
    rules.push(
        Rule::new(r"^\s*(HOST|Loss%|Snt|Last|Avg|Best|Wrst|StDev)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // Hop numbers
    rules.push(
        Rule::new(r"^\s*\d{1,2}\.\|?--")
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

    // Packet loss (0% is good)
    rules.push(
        Rule::new(r"\b0\.0%\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    // Packet loss (non-zero is bad)
    rules.push(
        Rule::new(r"\b[1-9]\d*\.\d%\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // 100% loss
    rules.push(
        Rule::new(r"\b100\.0%\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // Response times - good (< 50ms)
    rules.push(
        Rule::new(r"\b([0-4]?\d\.\d)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    // Response times - moderate (50-150ms)
    rules.push(
        Rule::new(r"\b([5-9]\d|1[0-4]\d)\.\d\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );

    // Response times - slow (> 150ms)
    rules.push(
        Rule::new(r"\b\d{3,}\.\d\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    );

    // Question marks for unresponsive hops
    rules.push(
        Rule::new(r"\?\?\?")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // AS numbers
    rules.push(
        Rule::new(r"\bAS\d+\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn mtr_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.mtr",
            "mtr",
            "My traceroute output",
            "network",
            mtr_rules(),
        )
        .with_detect_patterns(vec!["mtr"]),
    )
}

// =============================================================================
// TCPDUMP
// =============================================================================

fn tcpdump_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamp
    rules.push(
        Rule::new(r"^\d{2}:\d{2}:\d{2}\.\d+")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Protocol types
    rules.push(
        Rule::new(r"\b(IP|IP6|ARP|RARP|TCP|UDP|ICMP|IGMP|GRE|ESP|AH)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // IP addresses with ports
    rules.extend(common::ip_rules());
    rules.push(
        Rule::new(r"\.\d{1,5}\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Direction arrows
    rules.push(
        Rule::new(r"\s+>\s+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // TCP flags
    rules.push(
        Rule::new(r"\bFlags\s+\[[^\]]+\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Individual TCP flags
    rules.extend([
        Rule::new(r"\[S\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[S\.\]")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\[\.\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\[P\.\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[F\.\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[R\.\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\[R\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    // Sequence/Ack numbers
    rules.extend([
        Rule::new(r"\bseq\s+\d+(:\d+)?")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\back\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Window size
    rules.push(
        Rule::new(r"\bwin\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Length
    rules.push(
        Rule::new(r"\blength\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // TTL
    rules.push(
        Rule::new(r"\bttl\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // ARP specific
    rules.extend([
        Rule::new(r"\bRequest\s+who-has\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bReply\s+\S+\s+is-at\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // ICMP specific
    rules.extend([
        Rule::new(r"\bICMP echo request\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bICMP echo reply\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bICMP .* unreachable\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Packet capture info
    rules.extend([
        Rule::new(r"\d+\s+packets?\s+captured")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d+\s+packets?\s+received")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d+\s+packets?\s+dropped")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // MAC addresses
    rules.push(common::mac_address_rule());

    rules.push(common::number_rule());
    rules
}

pub fn tcpdump_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.tcpdump",
            "tcpdump",
            "Packet capture output",
            "network",
            tcpdump_rules(),
        )
        .with_detect_patterns(vec!["tcpdump", "tshark", "wireshark"]),
    )
}

// =============================================================================
// WHOIS
// =============================================================================

fn whois_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Field labels
    rules.push(
        Rule::new(r"^[A-Za-z][A-Za-z\s\-/]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Important fields
    rules.extend([
        Rule::new(r"^(Domain Name|Registrar|Registry Domain ID|Registrant|Admin|Tech|Name Server):")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^(Creation Date|Updated Date|Registry Expiry Date|Expiration Date):")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Domain names
    rules.push(
        Rule::new(r"\b[\w\-]+\.(com|net|org|io|dev|edu|gov|co\.\w+|uk|de|fr|eu)\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // CIDR blocks
    rules.push(
        Rule::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}/\d{1,2}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Dates
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}Z?)?")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Email addresses
    rules.push(
        Rule::new(r"\b[\w\.\-]+@[\w\.\-]+\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
    );

    // URLs
    rules.push(
        Rule::new(r"https?://[\w\.\-/]+")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
    );

    // Status values
    rules.extend([
        Rule::new(r"\b(clientTransferProhibited|clientDeleteProhibited|clientUpdateProhibited)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(ok|active)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(redemptionPeriod|pendingDelete)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Organization/Company names
    rules.push(
        Rule::new(r"\b(LLC|Inc\.|Corp\.|Ltd\.?|GmbH|S\.A\.|Co\.)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // AS numbers
    rules.push(
        Rule::new(r"\bAS\d+\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Notice/disclaimer sections
    rules.push(
        Rule::new(r"^%.*$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn whois_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.whois",
            "whois",
            "WHOIS domain lookup output",
            "network",
            whois_rules(),
        )
        .with_detect_patterns(vec!["whois"]),
    )
}

// =============================================================================
// NTPDATE / NTP
// =============================================================================

fn ntpdate_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamp
    rules.push(common::syslog_timestamp_rule());

    // Server info
    rules.push(
        Rule::new(r"\bserver\s+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // Stratum
    rules.push(
        Rule::new(r"\bstratum\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Offset values
    rules.extend([
        Rule::new(r"\boffset\s+[\-\+]?\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\boffset\s+[\-\+]?0\.0+\d*\s+sec")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\boffset\s+[\-\+]?[1-9]\d*\.\d+\s+sec")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Delay
    rules.push(
        Rule::new(r"\bdelay\s+[\-\+]?\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Status messages
    rules.extend([
        Rule::new(r"\badjust time server\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bstep time server\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bno server suitable\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bthe NTP socket is in use\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Reach values (reachability register)
    rules.push(
        Rule::new(r"\breach\s+[0-7]+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Jitter
    rules.push(
        Rule::new(r"\bjitter\s+\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Reference ID
    rules.push(
        Rule::new(r"\brefid\s+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Poll interval
    rules.push(
        Rule::new(r"\bpoll\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // ntpq/ntpdc specific
    rules.extend([
        Rule::new(r"^\*[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^\+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^-[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^x[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn ntpdate_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ntpdate",
            "ntpdate",
            "NTP time synchronization output",
            "network",
            ntpdate_rules(),
        )
        .with_detect_patterns(vec!["ntpdate", "ntpd", "ntpq", "chronyd", "chronyc"]),
    )
}
