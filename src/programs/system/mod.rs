//! System log programs.
//!
//! Provides Program implementations for systemd, journalctl, syslog, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all System programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(systemd_program()));
    registry.register(Arc::new(syslog_program()));
    registry.register(Arc::new(fail2ban_program()));
    registry.register(Arc::new(dmesg_program()));
    registry.register(Arc::new(cron_program()));
    registry.register(Arc::new(auditd_program()));
    registry.register(Arc::new(iptables_program()));
}

// =============================================================================
// SYSTEMD / JOURNALCTL
// =============================================================================

fn systemd_rules() -> Vec<Rule> {
    let mut rules = common::syslog_priority_rules();

    // Systemd unit states
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

    // Service names (*.service, *.socket, etc.)
    rules.push(
        Rule::new(r"\b[\w\-]+\.(service|socket|timer|mount|target|slice|scope)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Hostname and PIDs
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

    // Timestamps
    rules.push(common::syslog_timestamp_rule());
    rules.push(common::iso_timestamp_rule());

    // Size and exit codes
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

    // Kernel messages
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

fn systemd_program() -> SimpleProgram {
    SimpleProgram::new(
        "system.systemd",
        "systemd",
        "Systemd journal and unit logs",
        "system",
        systemd_rules(),
    )
    .with_detect_patterns(vec!["journalctl", "systemctl", "systemd"])
}

// =============================================================================
// SYSLOG
// =============================================================================

fn syslog_rules() -> Vec<Rule> {
    // Facilities first
    let mut rules = vec![Rule::new(
        r"\b(kern|user|mail|daemon|auth|syslog|lpr|news|uucp|cron|authpriv|ftp|local[0-7])\b",
    )
    .unwrap()
    .semantic(SemanticColor::Label)
    .build()];

    // Priority levels
    rules.extend(common::syslog_priority_rules());

    // Timestamp and hostname
    rules.push(common::syslog_timestamp_rule());
    rules.push(
        Rule::new(r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\s+(\S+)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Process name and PID
    rules.push(
        Rule::new(r"\s([\w\-]+)\[\d+\]:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // IP addresses
    rules.push(common::ipv4_rule());

    // SSH/authentication
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

fn syslog_program() -> SimpleProgram {
    SimpleProgram::new(
        "system.syslog",
        "syslog",
        "Traditional syslog format",
        "system",
        syslog_rules(),
    )
    .with_detect_patterns(vec![
        "syslog",
        "/var/log/syslog",
        "/var/log/messages",
        "rsyslog",
    ])
}

// =============================================================================
// FAIL2BAN
// =============================================================================

fn fail2ban_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps (fail2ban format: 2023-02-17 23:44:17,037)
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2},\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Module names
    rules.push(
        Rule::new(r"\bfail2ban\.\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Log levels
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

    // Jail names in brackets
    rules.push(
        Rule::new(r"\[[\w\-]+\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Actions (Ban, Unban, Found)
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

    // IP addresses
    rules.push(common::ipv4_rule());

    // PIDs in brackets
    rules.push(
        Rule::new(r"\[\d+\]:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Ban count/retry info
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

fn fail2ban_program() -> SimpleProgram {
    SimpleProgram::new(
        "system.fail2ban",
        "fail2ban",
        "Fail2ban intrusion prevention logs",
        "system",
        fail2ban_rules(),
    )
    .with_detect_patterns(vec!["fail2ban", "fail2ban-client"])
}

// =============================================================================
// DMESG (Kernel Ring Buffer)
// =============================================================================

fn dmesg_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Kernel timestamp [    0.000000] or [ 1234.567890]
    rules.push(
        Rule::new(r"\[\s*\d+\.\d+\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Kernel log levels
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

    // Kernel subsystems
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

    // Device names
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

    // Memory addresses and hex values
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

    // Hardware events
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

fn dmesg_program() -> SimpleProgram {
    SimpleProgram::new(
        "system.dmesg",
        "dmesg",
        "Kernel ring buffer messages",
        "system",
        dmesg_rules(),
    )
    .with_detect_patterns(vec!["dmesg", "/var/log/kern", "/var/log/dmesg"])
}

// =============================================================================
// CRON
// =============================================================================

fn cron_rules() -> Vec<Rule> {
    let mut rules = vec![
        // Syslog timestamp for cron
        common::syslog_timestamp_rule(),
        // CRON identifier
        Rule::new(r"\bCRON\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // crond identifier
        Rule::new(r"\bcrond\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ];

    // User context
    rules.push(
        Rule::new(r"\([^\)]+\)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Command execution
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

    // Job completion status
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

    // Session management
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

    // Anacron
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

fn cron_program() -> SimpleProgram {
    SimpleProgram::new(
        "system.cron",
        "cron",
        "Cron scheduled task logs",
        "system",
        cron_rules(),
    )
    .with_detect_patterns(vec!["cron", "crond", "anacron", "/var/log/cron"])
}

// =============================================================================
// AUDITD (Linux Audit Daemon)
// =============================================================================

fn auditd_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Audit message type
    rules.push(
        Rule::new(r"type=\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Timestamp and audit ID
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

    // Common audit fields
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

    // Executable and command
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

    // Result status
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

    // Syscall info
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

    // SELinux context
    rules.push(
        Rule::new(r"\bsubj=[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    );

    // Key for audit rules
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

fn auditd_program() -> SimpleProgram {
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
    ])
}

// =============================================================================
// IPTABLES / NFTABLES (Firewall)
// =============================================================================

fn iptables_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Kernel prefix for firewall logs
    rules.push(common::syslog_timestamp_rule());

    // iptables/nftables prefixes
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

    // Action (DROP, ACCEPT, REJECT, etc.)
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

    // Chain names
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

    // Network interfaces
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

    // IP addresses
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

    // Protocol and ports
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

    // TCP flags
    rules.push(
        Rule::new(r"\b(SYN|ACK|FIN|RST|PSH|URG)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // MAC addresses
    rules.push(
        Rule::new(r"\bMAC=[a-fA-F0-9:]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Packet details
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

fn iptables_program() -> SimpleProgram {
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
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 7);
    }

    #[test]
    fn test_systemd_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("journalctl -f");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "systemd");
    }

    #[test]
    fn test_syslog_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("tail -f /var/log/syslog");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "syslog");
    }

    #[test]
    fn test_dmesg_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("dmesg -w");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "dmesg");
    }

    #[test]
    fn test_cron_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("tail -f /var/log/cron");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "cron");
    }

    #[test]
    fn test_auditd_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("ausearch -m USER_LOGIN");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "auditd");
    }

    #[test]
    fn test_iptables_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("iptables -L -n");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "iptables");
    }
}
