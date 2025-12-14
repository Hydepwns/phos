//! Socket and connection monitoring tools.
//!
//! Provides Program implementations for netstat, ss, and sockstat.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// NETSTAT
// =============================================================================

fn netstat_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Connection states
    rules.extend(common::connection_state_rules());

    // Protocol types
    rules.push(
        Rule::new(r"^(tcp|tcp6|udp|udp6|unix|raw)\s")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // IP addresses and ports
    rules.extend(common::ip_rules());
    rules.push(
        Rule::new(r":\d{1,5}\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Local/Foreign address columns
    rules.push(
        Rule::new(r"\b(0\.0\.0\.0|\*|::):\*?\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Header row
    rules.push(
        Rule::new(r"\b(Proto|Recv-Q|Send-Q|Local Address|Foreign Address|State|PID/Program name)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // Process info
    rules.push(
        Rule::new(r"\d+/[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Interface stats headers
    rules.push(
        Rule::new(r"\b(Iface|MTU|RX-OK|RX-ERR|RX-DRP|RX-OVR|TX-OK|TX-ERR|TX-DRP|TX-OVR|Flg)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // Interface names
    rules.push(common::device_name_rule());

    rules.push(common::number_rule());
    rules
}

pub fn netstat_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.netstat",
            "netstat",
            "Network statistics output",
            "network",
            netstat_rules(),
        )
        .with_detect_patterns(vec!["netstat"]),
    )
}

// =============================================================================
// SS (Socket Statistics)
// =============================================================================

fn ss_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Connection states
    rules.extend(common::connection_state_rules());

    // Protocol types
    rules.push(
        Rule::new(r"^(tcp|udp|raw|u_str|u_dgr|u_seq|nl|p_raw|p_dgr)\s")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Header row
    rules.push(
        Rule::new(r"\b(Netid|State|Recv-Q|Send-Q|Local Address:Port|Peer Address:Port|Process)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // IP addresses and ports
    rules.extend(common::ip_rules());

    // Socket options/info
    rules.extend([
Rule::new(r#"users:\(\("[^"]+",pid=\d+,fd=\d+\)\)"#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bcubic\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bwscale:\d+,\d+\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\brto:\d+\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\brtt:\d+(\.\d+)?/\d+(\.\d+)?\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\bcwnd:\d+\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Wildcard addresses
    rules.push(
        Rule::new(r"\b(\*|0\.0\.0\.0|::|\[::\]):\d+\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Timer info
    rules.push(
        Rule::new(r"\btimer:\([^)]+\)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Memory info
    rules.push(
        Rule::new(r"\bskmem:\([^)]+\)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn ss_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ss",
            "ss",
            "Socket statistics output",
            "network",
            ss_rules(),
        )
        .with_detect_patterns(vec!["ss "]),
    )
}

// =============================================================================
// SOCKSTAT (BSD Socket Statistics)
// =============================================================================

fn sockstat_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Header row
    rules.push(
        Rule::new(r"^(USER|COMMAND|PID|FD|PROTO|LOCAL ADDRESS|FOREIGN ADDRESS)\s")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
    );

    // Protocol types
    rules.push(
        Rule::new(r"\b(tcp4|tcp6|tcp46|udp4|udp6|udp46|icm4|icm6|raw4|raw6)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Connection states
    rules.extend(common::connection_state_rules());

    // IP addresses
    rules.extend(common::ip_rules());

    // Process info
    rules.extend([
        Rule::new(r"^\w+\s+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b\d+\s+\d+\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Wildcard addresses
    rules.push(
        Rule::new(r"\b\*:\d+\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Unix domain sockets
    rules.push(
        Rule::new(r"/[^\s]+\.sock(et)?")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn sockstat_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.sockstat",
            "sockstat",
            "BSD socket statistics output",
            "network",
            sockstat_rules(),
        )
        .with_detect_patterns(vec!["sockstat"]),
    )
}
