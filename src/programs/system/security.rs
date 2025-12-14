//! Security-related system programs.
//!
//! Provides Program implementations for fail2ban, auditd, and iptables/nftables.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// FAIL2BAN
// =============================================================================

fn fail2ban_rules() -> Vec<Rule> {
    let mut rules = vec![
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2},\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\bfail2ban\.\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ];

    rules.extend([
        Rule::new(r"\bCRITICAL\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bWARNING\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bNOTICE\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\[[\w\-]+\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.extend([
        Rule::new(r"\bBan\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bUnban\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bFound\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bAlready banned\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bRestore\s+Ban\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(
        Rule::new(r"\[\d+\]:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.extend([
        Rule::new(r"Ban\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"Fail2Ban\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn fail2ban_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.fail2ban",
            "fail2ban",
            "Fail2ban intrusion prevention logs",
            "system",
            fail2ban_rules(),
        )
        .with_detect_patterns(vec!["fail2ban", "fail2ban-client"]),
    )
}

// =============================================================================
// AUDITD (Linux Audit Daemon)
// =============================================================================

fn auditd_rules() -> Vec<Rule> {
    let mut rules = vec![
        Rule::new(r"type=\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    ];

    rules.extend([
        Rule::new(r"msg=audit\(\d+\.\d+:\d+\)")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\baudit\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bpid=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\buid=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bgid=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bauid=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bses=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bexe=[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bcomm=[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bres=success\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bres=failed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bsyscall=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\barch=[a-f0-9]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\bsubj=[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    );
    rules.push(
        Rule::new(r"\bkey=[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    rules.push(common::ipv4_rule());
    rules.push(common::key_value_rule());
    rules.push(common::number_rule());
    rules
}

pub fn auditd_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.auditd",
            "auditd",
            "Linux audit daemon logs",
            "system",
            auditd_rules(),
        )
        .with_detect_patterns(vec![
            "auditd",
            "auditctl",
            "ausearch",
            "aureport",
            "/var/log/audit",
        ]),
    )
}

// =============================================================================
// IPTABLES / NFTABLES (Firewall)
// =============================================================================

fn iptables_rules() -> Vec<Rule> {
    let mut rules = vec![common::syslog_timestamp_rule()];

    rules.extend([
        Rule::new(r"\biptables\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\bnft\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\bnftables\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bDROP\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bREJECT\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bACCEPT\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bLOG\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b(INPUT|OUTPUT|FORWARD|PREROUTING|POSTROUTING)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bchain=\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bIN=[\w\*]*")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bOUT=[\w\*]*")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bPHYSIN=\w+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bPHYSOUT=\w+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bSRC=\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bDST=\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bPROTO=(TCP|UDP|ICMP|GRE|ESP|AH)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bSPT=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bDPT=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\b(SYN|ACK|FIN|RST|PSH|URG)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );
    rules.push(
        Rule::new(r"\bMAC=[a-fA-F0-9:]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.extend([
        Rule::new(r"\bLEN=\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\bTTL=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bID=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bWINDOW=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn iptables_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.iptables",
            "iptables",
            "iptables/nftables firewall logs",
            "system",
            iptables_rules(),
        )
        .with_detect_patterns(vec![
            "iptables",
            "ip6tables",
            "nft",
            "nftables",
            "firewalld",
            "ufw",
        ]),
    )
}
