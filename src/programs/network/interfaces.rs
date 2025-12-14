//! Network interface configuration tools.
//!
//! Provides Program implementations for ifconfig, ip, iwconfig, and arp.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// IFCONFIG
// =============================================================================

fn ifconfig_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Interface names
    rules.push(
        Rule::new(r"^[\w\d]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Interface flags
    rules.push(
        Rule::new(r"<[A-Z,]+>")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // Netmask and broadcast
    rules.extend([
        Rule::new(r"\bnetmask\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bbroadcast\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // MAC/ether addresses
    rules.push(common::mac_address_rule());
    rules.push(
        Rule::new(r"\bether\s+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // MTU
    rules.push(
        Rule::new(r"\bmtu\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // RX/TX statistics
    rules.extend([
        Rule::new(r"\bRX\s+packets\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bTX\s+packets\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bRX\s+errors\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bTX\s+errors\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bdropped\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\boverruns\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Status keywords
    rules.extend([
        Rule::new(r"\bUP\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bRUNNING\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bLOOPBACK\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bMULTICAST\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bBROADCAST\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Bytes transferred
    rules.push(common::size_rule());

    rules.push(common::number_rule());
    rules
}

pub fn ifconfig_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ifconfig",
            "ifconfig",
            "Network interface configuration output",
            "network",
            ifconfig_rules(),
        )
        .with_detect_patterns(vec!["ifconfig"]),
    )
}

// =============================================================================
// IP (iproute2)
// =============================================================================

fn ip_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Interface numbers and names
    rules.push(
        Rule::new(r"^\d+:\s+[\w\d@]+:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Interface flags
    rules.push(
        Rule::new(r"<[A-Z,_]+>")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // IP addresses with CIDR
    rules.extend(common::ip_rules());
    rules.push(
        Rule::new(r"/\d{1,3}\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Link types
    rules.push(
        Rule::new(r"\blink/(ether|loopback|sit|gre|ipip|vti)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // MAC addresses
    rules.push(common::mac_address_rule());

    // Scope
    rules.push(
        Rule::new(r"\bscope\s+(global|link|host)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // State
    rules.extend([
        Rule::new(r"\bstate\s+UP\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bstate\s+DOWN\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bstate\s+UNKNOWN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Route table info
    rules.extend([
        Rule::new(r"\bdefault\s+via\s+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bdev\s+[\w\d]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bproto\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bmetric\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bsrc\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Neighbor/ARP info
    rules.extend([
        Rule::new(r"\blladdr\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bREACHABLE\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bSTALE\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bFAILED\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // MTU and qdisc
    rules.extend([
        Rule::new(r"\bmtu\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\bqdisc\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bqlen\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Dynamic/permanent
    rules.extend([
        Rule::new(r"\bdynamic\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bpermanent\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn ip_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ip",
            "ip",
            "Linux ip command output",
            "network",
            ip_rules(),
        )
        .with_detect_patterns(vec!["ip addr", "ip link", "ip route", "ip neigh", "ip -"]),
    )
}

// =============================================================================
// IWCONFIG (Wireless Interface Configuration)
// =============================================================================

fn iwconfig_rules() -> Vec<Rule> {
    let mut rules = vec![
        // Interface names
        Rule::new(r"^[\w\d]+\s+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Mode
        Rule::new(r"\bMode:(Managed|Master|Ad-Hoc|Monitor|Repeater|Secondary)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // ESSID
        Rule::new(r#"ESSID:"[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Access point / BSSID
        Rule::new(r"Access Point:\s+[A-Fa-f0-9:]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Frequency
        Rule::new(r"Frequency[=:]\d+(\.\d+)?\s*GHz")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // Channel
        Rule::new(r"Channel[=:]\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        // Bit rate
        Rule::new(r"Bit Rate[=:]\d+(\.\d+)?\s*(Mb/s|Gb/s)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // TX power
        Rule::new(r"Tx-Power[=:]\d+\s*dBm")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ];

    // Signal level / quality
    rules.extend([
        Rule::new(r"Signal level[=:]-?\d+\s*dBm")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"Link Quality[=:]\d+/\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"Noise level[=:]-?\d+\s*dBm")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Encryption
    rules.extend([
        Rule::new(r"Encryption key:(on|off)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bWPA\d?\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bWEP\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Power management
    rules.push(
        Rule::new(r"Power Management:(on|off)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Not associated
    rules.push(
        Rule::new(r"Not-Associated")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    // IEEE standard
    rules.push(
        Rule::new(r"IEEE\s+802\.11[abgn/ac]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn iwconfig_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.iwconfig",
            "iwconfig",
            "Wireless interface configuration output",
            "network",
            iwconfig_rules(),
        )
        .with_detect_patterns(vec!["iwconfig", "iw dev", "iwlist"]),
    )
}

// =============================================================================
// ARP (Address Resolution Protocol)
// =============================================================================

fn arp_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Header row
    rules.push(
        Rule::new(r"^Address\s+HWtype\s+HWaddress\s+Flags Mask\s+Iface")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // MAC addresses
    rules.push(common::mac_address_rule());

    // Hardware types
    rules.push(
        Rule::new(r"\bether\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Interface names
    rules.push(common::device_name_rule());

    // Flags
    rules.extend([
        Rule::new(r"\bC\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bM\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bP\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Incomplete entries
    rules.push(
        Rule::new(r"\(incomplete\)")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );

    // Modern arp -a output format
    rules.push(
        Rule::new(r"\?\s+\([^)]+\)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // "at" keyword in arp output
    rules.push(
        Rule::new(r"\bat\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // "on" keyword for interface
    rules.push(
        Rule::new(r"\bon\s+[\w\d]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn arp_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.arp",
            "arp",
            "ARP table output",
            "network",
            arp_rules(),
        )
        .with_detect_patterns(vec!["arp"]),
    )
}
