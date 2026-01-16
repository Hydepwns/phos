//! Core system logging programs.
//!
//! Provides Program implementations for systemd, syslog, dmesg, and cron.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

// =============================================================================
// SYSTEMD / JOURNALCTL
// =============================================================================

fn systemd_rules() -> Vec<Rule> {
    let mut rules = common::syslog_priority_rules();

    rules.extend([
        Rule::new(r"\b(active|running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(inactive|dead|stopped)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\b(failed)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b(activating|deactivating|reloading)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\b[\w\-]+\.(service|socket|timer|mount|target|slice|scope)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    rules.extend([
        Rule::new(r"^[A-Za-z][A-Za-z0-9\-]*\s")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\[(\d+)\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"PID[=:\s]+\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"UID[=:\s]+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"GID[=:\s]+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    rules.push(common::syslog_timestamp_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::size_rule());

    rules.extend([
        Rule::new(r"exit[- ]?(code|status)[=:\s]+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"code=exited, status=\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\[\s*\d+\.\d+\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"kernel:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn systemd_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "system.systemd",
            "systemd",
            "Systemd journal and unit logs",
            Category::System,
            systemd_rules(),
        )
        .with_detect_patterns(vec!["journalctl", "systemctl", "systemd"]),
    )
}

// =============================================================================
// SYSLOG
// =============================================================================

fn syslog_rules() -> Vec<Rule> {
    let mut rules = vec![Rule::new(
        r"\b(kern|user|mail|daemon|auth|syslog|lpr|news|uucp|cron|authpriv|ftp|local[0-7])\b",
    )
    .unwrap()
    .semantic(SemanticColor::Label)
    .build()];

    rules.extend(common::syslog_priority_rules());
    rules.push(common::syslog_timestamp_rule());
    rules.push(
        Rule::new(r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\s+(\S+)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );
    rules.push(
        Rule::new(r"\s([\w\-]+)\[\d+\]:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );
    rules.push(common::ipv4_rule());

    rules.extend([
        Rule::new(r"\b(Accepted|accepted)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Failed|failed|Failure|failure|Invalid|invalid)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\b(session opened|session closed)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"for user (\w+)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"from (\S+)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"port \d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn syslog_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "system.syslog",
            "syslog",
            "Traditional syslog format",
            Category::System,
            syslog_rules(),
        )
        .with_detect_patterns(vec![
            "syslog",
            "/var/log/syslog",
            "/var/log/messages",
            "rsyslog",
        ]),
    )
}

// =============================================================================
// DMESG (Kernel Ring Buffer)
// =============================================================================

fn dmesg_rules() -> Vec<Rule> {
    let mut rules = vec![Rule::new(r"\[\s*\d+\.\d+\]")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()];

    rules.extend([
        Rule::new(r"\b(emerg|emergency|EMERG)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(alert|ALERT)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(crit|critical|CRIT)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(err|error|ERROR)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\b(warn|warning|WARNING)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(notice|NOTICE)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(info|INFO)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(debug|DEBUG)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b(usb|USB)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(scsi|SCSI)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(pci|PCI)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(acpi|ACPI)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(net|NET)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(cpu|CPU)\d*\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b(sd[a-z]+\d*|nvme\d+n\d+p?\d*)\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(eth\d+|enp\d+s\d+|wlan\d+|wlp\d+s\d+)\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b0x[a-fA-F0-9]+\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\b[a-fA-F0-9]{8,16}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b(attached|detached|connected|disconnected)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(initialized|registered)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(failed|failure|segfault|oops|panic)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    ]);

    rules.push(common::size_rule());
    rules.push(common::number_rule());
    rules
}

pub fn dmesg_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "system.dmesg",
            "dmesg",
            "Kernel ring buffer messages",
            Category::System,
            dmesg_rules(),
        )
        .with_detect_patterns(vec!["dmesg", "/var/log/kern", "/var/log/dmesg"]),
    )
}

// =============================================================================
// CRON
// =============================================================================

fn cron_rules() -> Vec<Rule> {
    let mut rules = vec![
        common::syslog_timestamp_rule(),
        Rule::new(r"\bCRON\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\bcrond\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ];

    rules.push(
        Rule::new(r"\([^\)]+\)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.extend([
        Rule::new(r"\bCMD\s+\(")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bEXEC\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\b(completed|finished|succeeded)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(failed|error)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\bpam_unix\(cron:session\)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bsession (opened|closed)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    rules.extend([
        Rule::new(r"\banacron\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bJob .+ started\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bNormal exit\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::number_rule());
    rules
}

pub fn cron_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "system.cron",
            "cron",
            "Cron scheduled task logs",
            Category::System,
            cron_rules(),
        )
        .with_detect_patterns(vec!["cron", "crond", "anacron", "/var/log/cron"]),
    )
}
