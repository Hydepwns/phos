//! Integration tests for phos programs.
//!
//! These tests verify that colorization rules work correctly on real log samples.

use phos::{programs, Colorizer, Theme};

/// Helper to check if output contains ANSI escape codes
fn has_ansi_codes(s: &str) -> bool {
    s.contains("\x1b[")
}

/// Helper to count lines with ANSI codes
fn count_colorized_lines(lines: &[String]) -> usize {
    lines.iter().filter(|l| has_ansi_codes(l)).count()
}

/// Load fixture file and return lines
fn load_fixture(name: &str) -> Vec<String> {
    let path = format!("tests/fixtures/{name}");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture {path}: {e}"))
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect()
}

/// Colorize lines with a program and return results
fn colorize_with_program(program_id: &str, lines: &[String]) -> Vec<String> {
    let registry = programs::default_registry();
    let program = registry
        .get(program_id)
        .unwrap_or_else(|| panic!("Program '{program_id}' not found"));

    let mut colorizer = Colorizer::new(program.rules()).with_theme(Theme::default());

    lines.iter().map(|line| colorizer.colorize(line)).collect()
}

// =============================================================================
// Ethereum Client Tests
// =============================================================================

mod ethereum {
    use super::*;

    #[test]
    fn test_lighthouse_colorization() {
        let lines = load_fixture("ethereum.log");
        let lighthouse_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.contains("INFO Synced") || l.contains("WARN Low") || l.contains("ERRO"))
            .take(5)
            .cloned()
            .collect();

        let colored = colorize_with_program("ethereum.lighthouse", &lighthouse_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Lighthouse logs to be colorized, got {} colorized out of {}",
            colorized_count,
            lighthouse_lines.len()
        );
    }

    #[test]
    fn test_lodestar_colorization() {
        let lines = load_fixture("ethereum.log");
        let lodestar_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.contains("[] info:") || l.contains("[] warn:") || l.contains("[] error:"))
            .cloned()
            .collect();

        let colored = colorize_with_program("ethereum.lodestar", &lodestar_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Lodestar logs to be colorized, got {} colorized out of {}",
            colorized_count,
            lodestar_lines.len()
        );
    }

    #[test]
    fn test_geth_colorization() {
        let lines = load_fixture("ethereum.log");
        let geth_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.starts_with("INFO [")
                    || l.starts_with("WARN [")
                    || l.starts_with("ERROR[")
                    || l.starts_with("DEBUG[")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("ethereum.geth", &geth_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Geth logs to be colorized, got {} colorized out of {}",
            colorized_count,
            geth_lines.len()
        );
    }

    #[test]
    fn test_prysm_colorization() {
        let lines = load_fixture("ethereum.log");
        let prysm_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.contains("]  INFO") || l.contains("]  WARN") || l.contains("] ERROR"))
            .cloned()
            .collect();

        let colored = colorize_with_program("ethereum.prysm", &prysm_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Prysm logs to be colorized, got {} colorized out of {}",
            colorized_count,
            prysm_lines.len()
        );
    }

    #[test]
    fn test_reth_colorization() {
        let lines = load_fixture("ethereum.log");
        let reth_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.contains("reth::"))
            .cloned()
            .collect();

        let colored = colorize_with_program("ethereum.reth", &reth_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Reth logs to be colorized, got {} colorized out of {}",
            colorized_count,
            reth_lines.len()
        );
    }

    #[test]
    fn test_ethereum_patterns_match() {
        // Test that common Ethereum patterns are matched
        let test_lines = vec![
            "slot=12345 epoch=385".to_string(),
            "hash=0x4f6a8b2c1d3e5f7a9b0c2d4e6f8a1b3c5d7e9f0a1b2c3d4e5f6a7b8c9d0e1f2a".to_string(),
            "finalized_epoch: 383".to_string(),
            "peer_count=50".to_string(),
        ];

        let colored = colorize_with_program("ethereum.lighthouse", &test_lines);

        for (original, colorized) in test_lines.iter().zip(colored.iter()) {
            assert!(
                has_ansi_codes(colorized),
                "Expected '{original}' to be colorized"
            );
        }
    }
}

// =============================================================================
// DevOps Program Tests
// =============================================================================

mod devops {
    use super::*;

    #[test]
    fn test_docker_colorization() {
        let lines = load_fixture("devops.log");
        let docker_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Container")
                    || l.contains("Pulling")
                    || l.contains("CONTAINER ID")
                    || l.contains("Digest:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("devops.docker", &docker_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Docker logs to be colorized, got {} colorized out of {}",
            colorized_count,
            docker_lines.len()
        );
    }

    #[test]
    fn test_kubectl_colorization() {
        let lines = load_fixture("devops.log");
        let kubectl_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Running")
                    || l.contains("Pending")
                    || l.contains("Error from server")
                    || l.contains("Warning")
                    || l.contains("Normal")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("devops.kubectl", &kubectl_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected kubectl logs to be colorized, got {} colorized out of {}",
            colorized_count,
            kubectl_lines.len()
        );
    }

    #[test]
    fn test_terraform_colorization() {
        let lines = load_fixture("devops.log");
        let tf_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Terraform")
                    || l.contains("Plan:")
                    || l.contains("Apply complete")
                    || l.contains("Error:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("devops.terraform", &tf_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Terraform logs to be colorized, got {} colorized out of {}",
            colorized_count,
            tf_lines.len()
        );
    }

    #[test]
    fn test_ansible_colorization() {
        let lines = load_fixture("devops.log");
        let ansible_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("PLAY ")
                    || l.contains("TASK ")
                    || l.contains("ok:")
                    || l.contains("changed:")
                    || l.contains("failed:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("devops.ansible", &ansible_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Ansible logs to be colorized, got {} colorized out of {}",
            colorized_count,
            ansible_lines.len()
        );
    }

    #[test]
    fn test_helm_colorization() {
        let lines = load_fixture("devops.log");
        let helm_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("NAME:")
                    || l.contains("STATUS:")
                    || l.contains("Release")
                    || l.contains("Error:")
                    || l.contains("WARNING:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("devops.helm", &helm_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Helm logs to be colorized, got {} colorized out of {}",
            colorized_count,
            helm_lines.len()
        );
    }
}

// =============================================================================
// System Program Tests
// =============================================================================

mod system {
    use super::*;

    #[test]
    fn test_systemd_colorization() {
        let lines = load_fixture("system.log");
        let systemd_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("systemd[")
                    || l.contains("Started")
                    || l.contains("Stopped")
                    || l.contains("Failed")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("system.systemd", &systemd_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected systemd logs to be colorized, got {} colorized out of {}",
            colorized_count,
            systemd_lines.len()
        );
    }

    #[test]
    fn test_dmesg_colorization() {
        let lines = load_fixture("system.log");
        let dmesg_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.starts_with('[') && l.contains(']'))
            .filter(|l| {
                l.contains("Linux version")
                    || l.contains("usb")
                    || l.contains("EXT4")
                    || l.contains("Out of memory")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("system.dmesg", &dmesg_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected dmesg logs to be colorized, got {} colorized out of {}",
            colorized_count,
            dmesg_lines.len()
        );
    }

    #[test]
    fn test_fail2ban_colorization() {
        let lines = load_fixture("system.log");
        let f2b_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.contains("fail2ban"))
            .cloned()
            .collect();

        let colored = colorize_with_program("system.fail2ban", &f2b_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected fail2ban logs to be colorized, got {} colorized out of {}",
            colorized_count,
            f2b_lines.len()
        );
    }

    #[test]
    fn test_iptables_colorization() {
        let lines = load_fixture("system.log");
        let ipt_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Chain")
                    || l.contains("ACCEPT")
                    || l.contains("DROP")
                    || l.contains("iptables:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("system.iptables", &ipt_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected iptables logs to be colorized, got {} colorized out of {}",
            colorized_count,
            ipt_lines.len()
        );
    }

    #[test]
    fn test_lsblk_colorization() {
        let lines = load_fixture("system.log");
        let lsblk_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("sda")
                    || l.contains("nvme")
                    || l.contains("NAME")
                    || l.contains("disk")
                    || l.contains("part")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("system.lsblk", &lsblk_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected lsblk logs to be colorized, got {} colorized out of {}",
            colorized_count,
            lsblk_lines.len()
        );
    }
}

// =============================================================================
// Dev Program Tests
// =============================================================================

mod dev {
    use super::*;

    #[test]
    fn test_cargo_colorization() {
        let lines = load_fixture("dev.log");
        let cargo_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Compiling")
                    || l.contains("Finished")
                    || l.contains("error[")
                    || l.contains("warning:")
                    || l.contains("test result:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.cargo", &cargo_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected cargo logs to be colorized, got {} colorized out of {}",
            colorized_count,
            cargo_lines.len()
        );
    }

    #[test]
    fn test_git_colorization() {
        let lines = load_fixture("dev.log");
        let git_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("On branch")
                    || l.contains("modified:")
                    || l.contains("commit ")
                    || l.contains("fatal:")
                    || l.contains("error:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.git", &git_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected git logs to be colorized, got {} colorized out of {}",
            colorized_count,
            git_lines.len()
        );
    }

    #[test]
    fn test_npm_colorization() {
        let lines = load_fixture("dev.log");
        let npm_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("npm WARN")
                    || l.contains("npm ERR!")
                    || l.contains("added")
                    || l.contains("vulnerabilities")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.npm", &npm_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected npm logs to be colorized, got {} colorized out of {}",
            colorized_count,
            npm_lines.len()
        );
    }

    #[test]
    fn test_go_colorization() {
        let lines = load_fixture("dev.log");
        let go_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("go:")
                    || l.contains("undefined:")
                    || l.contains("PASS")
                    || l.contains("FAIL")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.go", &go_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected go logs to be colorized, got {} colorized out of {}",
            colorized_count,
            go_lines.len()
        );
    }

    #[test]
    fn test_make_colorization() {
        let lines = load_fixture("dev.log");
        let make_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("make[")
                    || l.contains("gcc")
                    || l.contains("Error")
                    || l.contains("warning:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.make", &make_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected make logs to be colorized, got {} colorized out of {}",
            colorized_count,
            make_lines.len()
        );
    }

    #[test]
    fn test_elixir_colorization() {
        let lines = load_fixture("dev.log");
        let elixir_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("Compiling")
                    || l.contains("Generated")
                    || l.contains("** (Mix)")
                    || l.contains("warning:")
                    || l.contains("tests,")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("dev.elixir", &elixir_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected elixir logs to be colorized, got {} colorized out of {}",
            colorized_count,
            elixir_lines.len()
        );
    }
}

// =============================================================================
// Network Program Tests
// =============================================================================

mod network {
    use super::*;

    #[test]
    fn test_ping_colorization() {
        let lines = load_fixture("network.log");
        let ping_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("PING")
                    || l.contains("bytes from")
                    || l.contains("icmp_seq")
                    || l.contains("packet loss")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("network.ping", &ping_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected ping logs to be colorized, got {} colorized out of {}",
            colorized_count,
            ping_lines.len()
        );
    }

    #[test]
    fn test_curl_colorization() {
        let lines = load_fixture("network.log");
        let curl_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("HTTP/")
                    || l.contains("curl:")
                    || l.contains("% Total")
                    || l.contains("Dload")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("network.curl", &curl_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected curl logs to be colorized, got {} colorized out of {}",
            colorized_count,
            curl_lines.len()
        );
    }

    #[test]
    fn test_nginx_colorization() {
        let lines = load_fixture("network.log");
        let nginx_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("[error]")
                    || l.contains("[warn]")
                    || l.contains("[notice]")
                    || l.contains("GET /")
                    || l.contains("POST /")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("network.nginx", &nginx_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected nginx logs to be colorized, got {} colorized out of {}",
            colorized_count,
            nginx_lines.len()
        );
    }

    #[test]
    fn test_dig_colorization() {
        let lines = load_fixture("network.log");
        let dig_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("DiG")
                    || l.contains("ANSWER SECTION")
                    || l.contains("Query time")
                    || l.contains("IN")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("network.dig", &dig_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected dig logs to be colorized, got {} colorized out of {}",
            colorized_count,
            dig_lines.len()
        );
    }

    #[test]
    fn test_ss_colorization() {
        let lines = load_fixture("network.log");
        let ss_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("LISTEN")
                    || l.contains("ESTAB")
                    || l.contains("TIME-WAIT")
                    || l.contains("State")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("network.ss", &ss_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected ss logs to be colorized, got {} colorized out of {}",
            colorized_count,
            ss_lines.len()
        );
    }
}

// =============================================================================
// Data Program Tests
// =============================================================================

mod data {
    use super::*;

    #[test]
    fn test_postgres_colorization() {
        let lines = load_fixture("data.log");
        let pg_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("LOG:")
                    || l.contains("WARNING:")
                    || l.contains("ERROR:")
                    || l.contains("FATAL:")
                    || l.contains("STATEMENT:")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("data.postgres", &pg_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected PostgreSQL logs to be colorized, got {} colorized out of {}",
            colorized_count,
            pg_lines.len()
        );
    }

    #[test]
    fn test_redis_colorization() {
        let lines = load_fixture("data.log");
        let redis_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains(":M ")
                    || l.contains(":S ")
                    || l.contains("127.0.0.1:6379")
                    || l.contains("(error)")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("data.redis", &redis_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Redis logs to be colorized, got {} colorized out of {}",
            colorized_count,
            redis_lines.len()
        );
    }

    #[test]
    fn test_mysql_colorization() {
        let lines = load_fixture("data.log");
        let mysql_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("[Note]")
                    || l.contains("[Warning]")
                    || l.contains("[ERROR]")
                    || l.contains("mysql>")
                    || l.contains("ERROR 1045")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("data.mysql", &mysql_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected MySQL logs to be colorized, got {} colorized out of {}",
            colorized_count,
            mysql_lines.len()
        );
    }

    #[test]
    fn test_elasticsearch_colorization() {
        let lines = load_fixture("data.log");
        let es_lines: Vec<_> = lines
            .iter()
            .filter(|l| {
                l.contains("[INFO ]")
                    || l.contains("[WARN ]")
                    || l.contains("[ERROR]")
                    || l.contains("cluster_name")
            })
            .cloned()
            .collect();

        let colored = colorize_with_program("data.elasticsearch", &es_lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected Elasticsearch logs to be colorized, got {} colorized out of {}",
            colorized_count,
            es_lines.len()
        );
    }
}

// =============================================================================
// Cross-cutting Tests
// =============================================================================

mod common_patterns {
    use super::*;

    #[test]
    fn test_ip_addresses_colorized() {
        let lines = vec![
            "Connection from 192.168.1.100".to_string(),
            "Server: 10.0.0.1:8080".to_string(),
            "IPv6: 2001:db8::1".to_string(),
        ];

        // Test with nginx which has IP rules
        let colored = colorize_with_program("network.nginx", &lines);

        for (original, colorized) in lines.iter().zip(colored.iter()) {
            assert!(
                has_ansi_codes(colorized),
                "Expected IP in '{original}' to be colorized"
            );
        }
    }

    #[test]
    fn test_log_levels_colorized() {
        // Test with syslog program which explicitly has log level rules
        let lines = vec![
            "Dec 15 10:30:45 host process: error occurred".to_string(),
            "Dec 15 10:30:45 host process: warning message".to_string(),
            "Dec 15 10:30:45 host process: info logged".to_string(),
        ];

        let colored = colorize_with_program("system.syslog", &lines);

        // At least error and warning should be colorized
        let colorized_count = count_colorized_lines(&colored);
        assert!(
            colorized_count >= 2,
            "Expected at least error and warning to be colorized, got {colorized_count}"
        );
    }

    #[test]
    fn test_timestamps_colorized() {
        let lines = vec![
            "2024-01-15T10:30:45.123Z Event occurred".to_string(),
            "[2024-01-15 10:30:45] Something happened".to_string(),
            "Jan 15 10:30:45 hostname process".to_string(),
        ];

        // Test with systemd which has timestamp rules
        let colored = colorize_with_program("system.systemd", &lines);
        let colorized_count = count_colorized_lines(&colored);

        assert!(
            colorized_count > 0,
            "Expected timestamps to be colorized, got {colorized_count}"
        );
    }

    #[test]
    fn test_empty_lines_unchanged() {
        let lines = vec![String::new(), "   ".to_string()];

        let colored = colorize_with_program("dev.cargo", &lines);

        assert_eq!(colored[0], "");
        // Whitespace-only lines may or may not be modified depending on rules
    }

    #[test]
    fn test_no_crash_on_special_characters() {
        let lines = vec![
            "Special chars: <>&\"'`$()[]{}".to_string(),
            "Unicode: \u{1F600} \u{2603} \u{2764}".to_string(),
            "Escape sequences: \\n \\t \\r".to_string(),
            "ANSI already: \x1b[31mred\x1b[0m".to_string(),
        ];

        // Should not panic
        let colored = colorize_with_program("dev.cargo", &lines);
        assert_eq!(colored.len(), 4);
    }
}

// =============================================================================
// Registry Tests
// =============================================================================

mod registry {
    use super::*;

    #[test]
    fn test_all_programs_have_rules() {
        let registry = programs::default_registry();

        for info in registry.list() {
            let program = registry.get(&info.id).expect("Program should exist");
            let rules = program.rules();
            assert!(
                !rules.is_empty(),
                "Program '{}' has no rules",
                info.id
            );
        }
    }

    #[test]
    fn test_program_detection() {
        let registry = programs::default_registry();

        // Test detection for various commands
        let test_cases = vec![
            ("docker logs myapp", Some("devops.docker")),
            ("kubectl get pods", Some("devops.kubectl")),
            ("cargo build", Some("dev.cargo")),
            ("npm install", Some("dev.npm")),
            ("ping 8.8.8.8", Some("network.ping")),
            ("git status", Some("dev.git")),
            ("journalctl -f", Some("system.systemd")),
        ];

        for (command, expected_id) in test_cases {
            let detected = registry.detect(command);
            match expected_id {
                Some(id) => {
                    assert!(
                        detected.is_some(),
                        "Expected '{command}' to detect program '{id}'"
                    );
                    assert_eq!(
                        detected.unwrap().info().id,
                        id,
                        "Command '{command}' detected wrong program"
                    );
                }
                None => {
                    assert!(
                        detected.is_none(),
                        "Expected '{command}' to not detect any program"
                    );
                }
            }
        }
    }

    #[test]
    fn test_program_count() {
        let registry = programs::default_registry();
        let count = registry.len();

        // Should have 98 programs as documented
        assert_eq!(
            count, 98,
            "Expected 98 programs in registry, got {count}"
        );
    }
}
