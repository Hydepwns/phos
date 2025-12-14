//! System programs.
//!
//! Provides Program implementations for system utilities like systemd, ps, ls, etc.

mod core;
mod files;
mod hardware;
mod packages;
mod processes;
mod security;

use super::common;
use crate::program::ProgramRegistry;

/// Register all System programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    // Core system logging
    registry.register(core::systemd_program());
    registry.register(core::syslog_program());
    registry.register(core::dmesg_program());
    registry.register(core::cron_program());

    // Security
    registry.register(security::fail2ban_program());
    registry.register(security::auditd_program());
    registry.register(security::iptables_program());

    // Filesystem
    registry.register(files::ls_program());
    registry.register(files::df_program());
    registry.register(files::du_program());
    registry.register(files::stat_program());
    registry.register(files::mount_program());

    // Process monitoring
    registry.register(processes::ps_program());
    registry.register(processes::free_program());
    registry.register(processes::top_program());
    registry.register(processes::uptime_program());
    registry.register(processes::lsof_program());

    // Hardware and system stats
    registry.register(hardware::lsmod_program());
    registry.register(hardware::lspci_program());
    registry.register(hardware::vmstat_program());
    registry.register(hardware::iostat_program());
    registry.register(hardware::env_program());
    registry.register(hardware::blkid_program());
    registry.register(hardware::fdisk_program());
    registry.register(hardware::lsblk_program());

    // Package management
    registry.register(packages::dnf_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 26);
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
