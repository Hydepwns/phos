//! Network tool programs.
//!
//! Provides Program implementations for network tools like ping, curl, dig, etc.

mod diagnostics;
mod interfaces;
mod servers;
mod sockets;
mod tools;

use super::common;
use crate::program::ProgramRegistry;

/// Register all Network programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    // Basic network tools
    registry.register(tools::ping_program());
    registry.register(tools::curl_program());
    registry.register(tools::dig_program());
    registry.register(tools::traceroute_program());
    registry.register(tools::nmap_program());

    // Web servers and proxies
    registry.register(servers::nginx_program());
    registry.register(servers::caddy_program());
    registry.register(servers::apache_program());
    registry.register(servers::haproxy_program());
    registry.register(servers::traefik_program());

    // Socket monitoring
    registry.register(sockets::netstat_program());
    registry.register(sockets::ss_program());
    registry.register(sockets::sockstat_program());

    // Interface configuration
    registry.register(interfaces::ifconfig_program());
    registry.register(interfaces::ip_program());
    registry.register(interfaces::iwconfig_program());
    registry.register(interfaces::arp_program());

    // Network diagnostics
    registry.register(diagnostics::mtr_program());
    registry.register(diagnostics::tcpdump_program());
    registry.register(diagnostics::whois_program());
    registry.register(diagnostics::ntpdate_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 21);
    }

    #[test]
    fn test_ping_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("ping google.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "ping");
    }

    #[test]
    fn test_curl_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("curl -v https://api.example.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "curl");
    }

    #[test]
    fn test_dig_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("dig example.com");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "dig");
    }
}
