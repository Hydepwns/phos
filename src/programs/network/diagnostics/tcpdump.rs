//! Tcpdump packet capture colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::programs::common;
use crate::rule::Rule;

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
            Category::Network,
            tcpdump_rules(),
        )
        .with_detect_patterns(vec!["tcpdump", "tshark", "wireshark"]),
    )
}
