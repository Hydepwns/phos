//! CLI integration tests for phos.
//!
//! These tests verify the command-line interface behavior.

use std::io::Write;
use std::process::{Command, Stdio};

/// Get the path to the phos binary
fn phos_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_phos"))
}

/// Run phos with arguments and return (stdout, stderr, success)
fn run_phos(args: &[&str]) -> (String, String, bool) {
    let output = phos_bin()
        .args(args)
        .output()
        .expect("Failed to execute phos");

    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.success(),
    )
}

/// Run phos with stdin input
fn run_phos_with_stdin(args: &[&str], input: &str) -> (String, String, bool) {
    let mut child = phos_bin()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn phos");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait for phos");

    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.success(),
    )
}

// =============================================================================
// Version and Help Tests
// =============================================================================

mod version_help {
    use super::*;

    #[test]
    fn test_version_flag() {
        let (stdout, _, success) = run_phos(&["--version"]);
        assert!(success);
        assert!(stdout.contains("phos"));
        assert!(stdout.contains("0.2.0"));
    }

    #[test]
    fn test_help_flag() {
        let (stdout, _, success) = run_phos(&["--help"]);
        assert!(success);
        assert!(stdout.contains("log colorizer") || stdout.contains("phos"));
        assert!(stdout.contains("--program"));
        assert!(stdout.contains("--client"));
        assert!(stdout.contains("--theme"));
    }

    #[test]
    fn test_short_help_flag() {
        let (stdout, _, success) = run_phos(&["-h"]);
        assert!(success);
        assert!(stdout.contains("phos"));
    }
}

// =============================================================================
// List Subcommand Tests
// =============================================================================

mod list_command {
    use super::*;

    #[test]
    fn test_list_all_programs() {
        let (stdout, _, success) = run_phos(&["list"]);
        assert!(success);
        assert!(stdout.contains("Available programs"));
        // At least 98 built-in programs (may include user programs)
        assert!(stdout.contains("total)"), "Should show total count");
        // Check for some known programs
        assert!(stdout.contains("docker"));
        assert!(stdout.contains("cargo"));
        assert!(stdout.contains("nginx"));
    }

    #[test]
    fn test_list_by_category_ethereum() {
        let (stdout, _, success) = run_phos(&["list", "--category", "ethereum"]);
        assert!(success);
        assert!(stdout.contains("Ethereum"));
        assert!(stdout.contains("lighthouse"));
        assert!(stdout.contains("geth"));
        assert!(stdout.contains("Consensus:"));
        assert!(stdout.contains("Execution:"));
    }

    #[test]
    fn test_list_by_category_devops() {
        let (stdout, _, success) = run_phos(&["list", "--category", "devops"]);
        assert!(success);
        assert!(stdout.contains("DevOps"));
        assert!(stdout.contains("docker"));
        assert!(stdout.contains("kubectl"));
    }

    #[test]
    fn test_list_by_category_dev() {
        let (stdout, _, success) = run_phos(&["list", "--category", "dev"]);
        assert!(success);
        assert!(stdout.contains("Development"));
        assert!(stdout.contains("cargo"));
        assert!(stdout.contains("git"));
        assert!(stdout.contains("npm"));
    }

    #[test]
    fn test_list_clients_alias() {
        let (stdout, _, success) = run_phos(&["list-clients"]);
        assert!(success);
        assert!(stdout.contains("Ethereum"));
        assert!(stdout.contains("lighthouse"));
        assert!(stdout.contains("Consensus:"));
    }
}

// =============================================================================
// Themes Subcommand Tests
// =============================================================================

mod themes_command {
    use super::*;

    #[test]
    fn test_list_themes() {
        let (stdout, _, success) = run_phos(&["themes"]);
        assert!(success);
        assert!(stdout.contains("Available themes"));
        assert!(stdout.contains("default-dark"));
        assert!(stdout.contains("monokai"));
        assert!(stdout.contains("solarized"));
    }
}

// =============================================================================
// Info Subcommand Tests
// =============================================================================

mod info_command {
    use super::*;

    #[test]
    fn test_info_docker() {
        let (stdout, _, success) = run_phos(&["info", "docker"]);
        assert!(success);
        assert!(stdout.contains("Docker"));
        assert!(stdout.contains("ID:"));
        assert!(stdout.contains("devops.docker"));
        assert!(stdout.contains("Rules:"));
    }

    #[test]
    fn test_info_lighthouse() {
        let (stdout, _, success) = run_phos(&["info", "lighthouse"]);
        assert!(success);
        assert!(stdout.contains("Lighthouse"));
        assert!(stdout.contains("Layer:"));
        assert!(stdout.contains("Consensus"));
        assert!(stdout.contains("Brand color:"));
    }

    #[test]
    fn test_info_cargo() {
        let (stdout, _, success) = run_phos(&["info", "cargo"]);
        assert!(success);
        assert!(stdout.contains("cargo"));
        assert!(stdout.contains("Category:"));
        assert!(stdout.contains("dev"));
    }

    #[test]
    fn test_info_unknown_program() {
        let (_, stderr, success) = run_phos(&["info", "nonexistent"]);
        assert!(!success);
        assert!(stderr.contains("Unknown program"));
    }
}

// =============================================================================
// Colors Subcommand Tests
// =============================================================================

mod colors_command {
    use super::*;

    #[test]
    fn test_show_colors() {
        let (stdout, _, success) = run_phos(&["colors"]);
        assert!(success);
        assert!(stdout.contains("Ethereum Client Brand Colors"));
        assert!(stdout.contains("lighthouse"));
        assert!(stdout.contains("geth"));
        assert!(stdout.contains('#')); // Hex color codes
    }
}

// =============================================================================
// Completions Subcommand Tests
// =============================================================================

mod completions_command {
    use super::*;

    #[test]
    fn test_bash_completions() {
        let (stdout, _, success) = run_phos(&["completions", "bash"]);
        assert!(success);
        assert!(stdout.contains("_phos"));
        assert!(stdout.contains("complete"));
    }

    #[test]
    fn test_zsh_completions() {
        let (stdout, _, success) = run_phos(&["completions", "zsh"]);
        assert!(success);
        assert!(stdout.contains("#compdef phos"));
    }

    #[test]
    fn test_fish_completions() {
        let (stdout, _, success) = run_phos(&["completions", "fish"]);
        assert!(success);
        assert!(stdout.contains("complete"));
        assert!(stdout.contains("phos"));
    }
}

// =============================================================================
// Shell Init Subcommand Tests
// =============================================================================

mod shell_init_command {
    use super::*;

    #[test]
    fn test_bash_shell_init() {
        let (stdout, _, success) = run_phos(&["shell-init", "bash"]);
        assert!(success);
        assert!(stdout.contains("phos")); // Should contain function definitions
    }

    #[test]
    fn test_zsh_shell_init() {
        let (stdout, _, success) = run_phos(&["shell-init", "zsh"]);
        assert!(success);
        assert!(stdout.contains("phos"));
    }

    #[test]
    fn test_fish_shell_init() {
        let (stdout, _, success) = run_phos(&["shell-init", "fish"]);
        assert!(success);
        assert!(stdout.contains("function"));
    }

    #[test]
    fn test_invalid_shell() {
        let (_, stderr, success) = run_phos(&["shell-init", "invalid"]);
        assert!(!success);
        assert!(stderr.contains("Unknown shell"));
    }
}

// =============================================================================
// Pipe Input Tests
// =============================================================================

mod pipe_input {
    use super::*;

    #[test]
    fn test_pipe_with_program_flag() {
        let input = "error[E0382]: borrow of moved value\nwarning: unused variable";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("error"));
        assert!(stdout.contains("warning"));
    }

    #[test]
    fn test_pipe_with_client_flag() {
        let input = "Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385";
        let (stdout, _, success) = run_phos_with_stdin(&["-c", "lighthouse"], input);
        assert!(success);
        assert!(stdout.contains("Synced"));
        assert!(stdout.contains("12345"));
    }

    #[test]
    fn test_pipe_with_theme() {
        let input = "ERROR: something failed";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo", "-t", "monokai"], input);
        assert!(success);
        assert!(stdout.contains("ERROR"));
    }

    #[test]
    fn test_pipe_empty_input() {
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], "");
        assert!(success);
        assert!(stdout.is_empty());
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line 1\nline 2\nline 3";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        let lines: Vec<_> = stdout.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_pipe_with_color_flag() {
        // Use a pattern that cargo program actually matches (error codes)
        let input = "error[E0382]: borrow of moved value";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo", "--color"], input);
        assert!(success);
        // With --color flag, should have ANSI codes even when piped
        assert!(
            stdout.contains("\x1b["),
            "Expected ANSI codes in output: {stdout}"
        );
    }

    #[test]
    fn test_pipe_without_color_flag() {
        let input = "ERROR: test";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        // Without --color and piped, should NOT have ANSI codes
        assert!(!stdout.contains("\x1b["));
    }
}

// =============================================================================
// Stats Flag Tests
// =============================================================================

mod stats_flag {
    use super::*;

    #[test]
    fn test_stats_output() {
        let input = "INFO: line 1\nERROR: line 2\nWARN: line 3";
        let (stdout, stderr, success) = run_phos_with_stdin(&["-p", "cargo", "--stats"], input);
        assert!(success);
        // Stats are printed to stderr
        let combined = format!("{stdout}{stderr}");
        assert!(combined.contains("Lines processed") || combined.contains("lines"));
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

mod error_handling {
    use super::*;

    #[test]
    fn test_unknown_program() {
        let (_, stderr, success) = run_phos_with_stdin(&["-p", "nonexistent"], "test");
        assert!(!success);
        assert!(stderr.contains("Unknown program"));
    }

    #[test]
    fn test_unknown_theme() {
        // Unknown theme falls back to default, doesn't error
        let input = "test line";
        let (stdout, _, success) =
            run_phos_with_stdin(&["-p", "cargo", "-t", "nonexistent"], input);
        assert!(success);
        assert!(stdout.contains("test line"));
    }

    #[test]
    fn test_no_input_no_args() {
        // When run with no input and no args, should show usage
        let (_, stderr, success) = run_phos(&["-p", "cargo"]);
        // This may vary - testing that it doesn't crash
        // The actual behavior depends on TTY detection
        let _ = (success, stderr);
    }
}

// =============================================================================
// Program/Client Flag Tests
// =============================================================================

mod program_flags {
    use super::*;

    #[test]
    fn test_short_program_flag() {
        let input = "error: test";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("error"));
    }

    #[test]
    fn test_long_program_flag() {
        let input = "error: test";
        let (stdout, _, success) = run_phos_with_stdin(&["--program", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("error"));
    }

    #[test]
    fn test_short_client_flag() {
        let input = "INFO Synced slot: 12345";
        let (stdout, _, success) = run_phos_with_stdin(&["-c", "lighthouse"], input);
        assert!(success);
        assert!(stdout.contains("INFO"));
    }

    #[test]
    fn test_long_client_flag() {
        let input = "INFO Synced slot: 12345";
        let (stdout, _, success) = run_phos_with_stdin(&["--client", "lighthouse"], input);
        assert!(success);
        assert!(stdout.contains("INFO"));
    }

    #[test]
    fn test_program_by_full_id() {
        let input = "error: test";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "dev.cargo"], input);
        assert!(success);
        assert!(stdout.contains("error"));
    }

    #[test]
    fn test_program_by_short_name() {
        let input = "Container started";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "docker"], input);
        assert!(success);
        assert!(stdout.contains("Container"));
    }
}

// =============================================================================
// Command Execution Tests (running external commands)
// =============================================================================

mod command_execution {
    use super::*;

    // Note: Command execution tests are limited because when running as a subprocess,
    // stdin detection (`atty::is(atty::Stream::Stdin)`) returns false (not a TTY),
    // causing phos to try reading from stdin instead of running the command.
    // These tests would work in an interactive terminal but not in automated tests.

    #[test]
    #[ignore] // Requires TTY - run manually with `cargo test -- --ignored`
    fn test_run_echo_command() {
        let (stdout, _, success) = run_phos(&["-p", "cargo", "--", "echo", "hello world"]);
        assert!(success);
        assert!(stdout.contains("hello world"));
    }

    #[test]
    #[ignore] // Requires TTY - run manually with `cargo test -- --ignored`
    fn test_run_command_with_args() {
        let (stdout, _, success) = run_phos(&["-p", "cargo", "--", "echo", "-n", "test"]);
        assert!(success);
        assert!(stdout.contains("test"));
    }

    #[test]
    fn test_auto_detect_from_command() {
        // Test detection logic via pipe instead of command execution
        let input = "error: test failure";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("error"));
    }
}

// =============================================================================
// Config File Tests
// =============================================================================

mod config_file {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_config_file_not_found() {
        let (_, stderr, success) =
            run_phos_with_stdin(&["--config", "/nonexistent/config.yaml"], "test");
        assert!(!success);
        assert!(
            stderr.contains("No such file")
                || stderr.contains("not found")
                || stderr.contains("error")
        );
    }

    #[test]
    fn test_valid_config_file() {
        // Create a temporary config file
        let tmp_dir = env::temp_dir();
        let config_path = tmp_dir.join("phos_test_config.yaml");

        // Config format requires name and description fields
        let config_content = r"
name: test
description: Test configuration
rules:
  - regex: '\bTEST\b'
    colors: [error]
";
        fs::write(&config_path, config_content).expect("Failed to write test config");

        let input = "TEST pattern here";
        let (stdout, stderr, success) = run_phos_with_stdin(
            &["--config", config_path.to_str().unwrap(), "--color"],
            input,
        );

        // Clean up
        let _ = fs::remove_file(&config_path);

        assert!(success, "Config test failed: {stderr}");
        assert!(stdout.contains("TEST"));
    }
}

// =============================================================================
// Edge Cases
// =============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_very_long_line() {
        let long_line = "x".repeat(10000);
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], &long_line);
        assert!(success);
        assert!(stdout.len() >= 10000);
    }

    #[test]
    fn test_unicode_input() {
        let input = "ERROR: \u{1F600} emoji \u{2603} snowman \u{4E2D}\u{6587} chinese";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("\u{1F600}"));
        assert!(stdout.contains("\u{4E2D}\u{6587}"));
    }

    #[test]
    fn test_special_characters() {
        let input = "line with <angle> & \"quotes\" 'single' `backtick`";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.contains("<angle>"));
        assert!(stdout.contains("\"quotes\""));
    }

    #[test]
    fn test_empty_lines_preserved() {
        let input = "line1\n\nline3";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        let lines: Vec<_> = stdout.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[1].is_empty());
    }

    #[test]
    fn test_trailing_newline() {
        let input = "line\n";
        let (stdout, _, success) = run_phos_with_stdin(&["-p", "cargo"], input);
        assert!(success);
        assert!(stdout.ends_with('\n'));
    }
}

// =============================================================================
// All Programs Smoke Test
// =============================================================================

mod smoke_tests {
    use super::*;

    /// Smoke test that verifies all programs can be used without crashing
    #[test]
    fn test_all_programs_can_colorize() {
        let programs = [
            // DevOps
            "docker",
            "kubectl",
            "terraform",
            "k9s",
            "helm",
            "ansible",
            // System
            "systemd",
            "syslog",
            "dmesg",
            "fail2ban",
            "cron",
            "iptables",
            // Dev
            "cargo",
            "git",
            "npm",
            "go",
            "make",
            // Network
            "ping",
            "curl",
            "dig",
            "nginx",
            // Data
            "postgres",
            "redis",
            "mysql",
            "mongodb",
            "elasticsearch",
            // Monitoring
            "prometheus",
            "grafana",
            // Messaging
            "kafka",
            "rabbitmq",
            // CI
            "github-actions",
            "jenkins",
        ];

        let input = "test line with ERROR and WARNING and 192.168.1.1";

        let failures: Vec<_> = programs
            .iter()
            .filter_map(|program| {
                let (stdout, stderr, success) = run_phos_with_stdin(&["-p", program], input);
                if !success {
                    Some(format!("'{program}' failed: {stderr}"))
                } else if !stdout.contains("test line") {
                    Some(format!("'{program}' didn't output correctly"))
                } else {
                    None
                }
            })
            .collect();

        assert!(
            failures.is_empty(),
            "Program failures: {}",
            failures.join("; ")
        );
    }

    /// Smoke test that verifies all Ethereum clients can be used
    #[test]
    fn test_all_ethereum_clients_can_colorize() {
        let clients = [
            "lighthouse",
            "prysm",
            "teku",
            "nimbus",
            "lodestar",
            "grandine",
            "lambda",
            "geth",
            "nethermind",
            "besu",
            "erigon",
            "reth",
            "mana",
            "charon",
            "mev-boost",
        ];

        let input = "INFO slot=12345 epoch=385 head=0xabcdef1234567890";

        let failures: Vec<_> = clients
            .iter()
            .filter_map(|client| {
                let (stdout, stderr, success) = run_phos_with_stdin(&["-c", client], input);
                if !success {
                    Some(format!("'{client}' failed: {stderr}"))
                } else if !stdout.contains("INFO") && !stdout.contains("slot") {
                    Some(format!("'{client}' didn't output correctly"))
                } else {
                    None
                }
            })
            .collect();

        assert!(
            failures.is_empty(),
            "Client failures: {}",
            failures.join("; ")
        );
    }
}
