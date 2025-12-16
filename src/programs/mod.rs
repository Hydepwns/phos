//! Built-in program implementations.
//!
//! Provides high-performance, compiled-in programs for various log formats.

pub mod ci;
pub mod common;
pub mod data;
pub mod dev;
pub mod devops;
pub mod ethereum;
pub mod messaging;
pub mod monitoring;
pub mod network;
pub mod system;

use crate::program::ProgramRegistry;

/// Register all built-in programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    ethereum::register_all(registry);
    devops::register_all(registry);
    system::register_all(registry);
    dev::register_all(registry);
    network::register_all(registry);
    data::register_all(registry);
    monitoring::register_all(registry);
    messaging::register_all(registry);
    ci::register_all(registry);
}

/// Create a registry with all built-in programs.
pub fn default_registry() -> ProgramRegistry {
    let mut registry = ProgramRegistry::new();
    register_all(&mut registry);
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Category, Colorizer, Theme};

    /// Sample log lines for smoke testing each program category.
    const SAMPLE_LOGS: &[(&str, &str)] = &[
        // Ethereum
        ("lighthouse", "Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385, peers: 47"),
        ("geth", "INFO [12-05|00:12:36.557] Imported new chain segment number=19630289 hash=0x4f6a0b..."),
        ("lodestar", "Dec 05 00:12:36.557[] info: Synced - Peers 47 - Eph 167991/6 - slot 5375712"),
        // DevOps
        ("docker", "2024-01-15T10:30:45.123Z INFO Container started id=abc123def456"),
        ("kubectl", "error: the server doesn't have a resource type \"pods\""),
        ("terraform", "Plan: 3 to add, 1 to change, 0 to destroy."),
        // System
        ("systemd", "Dec 15 10:30:45 hostname systemd[1]: Started My Service."),
        ("dmesg", "[    0.000000] Linux version 5.15.0-generic"),
        // Dev
        ("cargo", "error[E0382]: borrow of moved value: `x`"),
        ("git", "commit abc123def (HEAD -> main, origin/main)"),
        ("npm", "npm ERR! 404 Not Found - GET https://registry.npmjs.org/nonexistent"),
        // Network
        ("nginx", "192.168.1.1 - - [15/Dec/2024:10:30:45 +0000] \"GET /api HTTP/1.1\" 200 1234"),
        ("ping", "64 bytes from 8.8.8.8: icmp_seq=1 ttl=117 time=14.2 ms"),
        ("curl", "HTTP/2 200 OK"),
        // Data
        ("postgres", "2024-01-15 10:30:45.123 UTC [1234] ERROR:  relation \"users\" does not exist"),
        ("redis", "[1234] 15 Dec 10:30:45.123 # Server started, Redis version=7.0.0"),
        // Monitoring
        ("prometheus", "level=info ts=2024-01-15T10:30:45.123Z caller=main.go:123 msg=\"Server is ready\""),
        // Messaging
        ("kafka", "[2024-01-15 10:30:45,123] INFO [Controller id=1] Processing partition"),
        // CI
        ("github-actions", "::error::Process completed with exit code 1."),
    ];

    #[test]
    fn test_all_programs_register() {
        let registry = default_registry();
        // Should have many programs registered
        assert!(
            registry.list().len() > 90,
            "Expected 90+ programs, got {}",
            registry.list().len()
        );
    }

    #[test]
    fn test_all_programs_have_valid_info() {
        let registry = default_registry();
        for info in registry.list() {
            assert!(!info.id.is_empty(), "Program has empty id");
            assert!(!info.name.is_empty(), "Program {} has empty name", info.id);
            assert!(
                !info.description.is_empty(),
                "Program {} has empty description",
                info.id
            );
        }
    }

    #[test]
    fn test_all_programs_have_rules() {
        let registry = default_registry();
        for info in registry.list() {
            let program = registry.get(&info.id).expect("Program should exist");
            let rules = program.rules();
            // All programs should have at least one rule
            assert!(
                !rules.is_empty(),
                "Program {} has no rules",
                info.id
            );
        }
    }

    #[test]
    fn test_all_programs_colorize_without_panic() {
        let registry = default_registry();
        let theme = Theme::default_dark();

        for info in registry.list() {
            let program = registry.get(&info.id).expect("Program should exist");
            let mut colorizer = Colorizer::new(program.rules()).with_theme(theme.clone());

            // Test with a generic log line
            let generic = "ERROR: something failed at 2024-01-15T10:30:45.123Z";
            let result = colorizer.colorize(generic);
            assert!(
                !result.is_empty() || generic.is_empty(),
                "Colorize returned empty for program {}",
                info.id
            );

            // Test with empty line
            let empty_result = colorizer.colorize("");
            assert_eq!(empty_result, "", "Empty line should return empty for {}", info.id);
        }
    }

    #[test]
    fn test_sample_logs_colorize() {
        let registry = default_registry();
        let theme = Theme::default_dark();

        for (program_name, sample_log) in SAMPLE_LOGS {
            if let Some(program) = registry.get(program_name) {
                let mut colorizer = Colorizer::new(program.rules()).with_theme(theme.clone());
                let result = colorizer.colorize(sample_log);

                // Result should contain the original text (possibly with ANSI codes)
                let stripped = strip_ansi(&result);
                assert!(
                    stripped.contains(&sample_log[..sample_log.len().min(20)]),
                    "Program {} changed log content unexpectedly",
                    program_name
                );
            }
        }
    }

    #[test]
    fn test_program_categories() {
        let registry = default_registry();

        let categories = [
            Category::Ethereum,
            Category::DevOps,
            Category::System,
            Category::Dev,
            Category::Network,
            Category::Data,
            Category::Monitoring,
            Category::Messaging,
            Category::CI,
        ];

        for category in categories {
            let programs = registry.list_by_category(category);
            assert!(
                !programs.is_empty(),
                "Category {:?} has no programs",
                category
            );
        }
    }

    #[test]
    fn test_program_detection() {
        let registry = default_registry();

        let test_cases = [
            ("docker logs -f container", "docker"),
            ("kubectl get pods", "kubectl"),
            ("cargo build --release", "cargo"),
            ("git status", "git"),
            ("npm install", "npm"),
        ];

        for (command, expected) in test_cases {
            if let Some(detected) = registry.detect(command) {
                assert_eq!(
                    detected.info().id.split('.').last().unwrap(),
                    expected,
                    "Detection failed for command: {}",
                    command
                );
            }
        }
    }

    /// Strip ANSI escape codes from a string for comparison.
    fn strip_ansi(s: &str) -> String {
        let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        re.replace_all(s, "").to_string()
    }
}
