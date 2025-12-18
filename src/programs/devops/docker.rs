//! Docker container colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn docker_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Container status (running, exited, created, restarting, paused, dead)
    rules.extend(common::container_status_rules());

    // Container/image IDs (64-char and 12-char)
    rules.extend(common::hex_id_rules().into_iter().take(2));

    // Image names with tags
    rules.extend([
        Rule::new(r"[\w\-\.]+/[\w\-\.]+:[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"[\w\-\.]+:[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Ports
    rules.extend([
        Rule::new(r"\d{1,5}->\d{1,5}/\w+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Common patterns
    rules.push(common::size_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::iso_timestamp_space_rule());
    rules.push(common::percentage_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn docker_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.docker",
            "Docker",
            "Docker container logs and commands",
            Category::DevOps,
            docker_rules(),
        )
        .with_detect_patterns(vec!["docker", "docker-compose", "podman"]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // COMPILE TESTS
    // =========================================================================

    #[test]
    fn test_docker_rules_compile() {
        let rules = docker_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_docker_program_info() {
        let program = docker_program();
        assert_eq!(program.info().id, "devops.docker");
        assert_eq!(program.info().name, "Docker");
    }

    // =========================================================================
    // CONTAINER STATUS MATCHING
    // =========================================================================

    #[test]
    fn test_container_status_running() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "CONTAINER ID  STATUS: running"));
        assert!(any_rule_matches(&rules, "mycontainer Running 5 hours"));
        assert!(any_rule_matches(&rules, "container status: Up 2 days"));
    }

    #[test]
    fn test_container_status_exited() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "CONTAINER ID  STATUS: exited"));
        assert!(any_rule_matches(
            &rules,
            "container Exited (0) 5 minutes ago"
        ));
        assert!(any_rule_matches(&rules, "status: stopped"));
    }

    #[test]
    fn test_container_status_other() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "status: created"));
        assert!(any_rule_matches(&rules, "Container is restarting"));
        assert!(any_rule_matches(&rules, "Container paused"));
        assert!(any_rule_matches(&rules, "status: dead"));
    }

    // =========================================================================
    // CONTAINER ID MATCHING
    // =========================================================================

    #[test]
    fn test_container_id_long() {
        let rules = docker_rules();
        // 64-char container/image IDs (sha256 digests)
        assert!(any_rule_matches(
            &rules,
            "sha256:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
        ));
    }

    #[test]
    fn test_container_id_git_style() {
        let rules = docker_rules();
        // 40-char IDs (git-style)
        assert!(any_rule_matches(
            &rules,
            "Image: a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
        ));
    }

    // =========================================================================
    // IMAGE NAME MATCHING
    // =========================================================================

    #[test]
    fn test_image_name_with_registry() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "docker.io/library/nginx:latest"));
        assert!(any_rule_matches(&rules, "gcr.io/project/image:v1.0.0"));
        assert!(any_rule_matches(&rules, "registry.example.com/app:1.2.3"));
    }

    #[test]
    fn test_image_name_with_tag() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "nginx:latest"));
        assert!(any_rule_matches(&rules, "postgres:15-alpine"));
        assert!(any_rule_matches(&rules, "myapp:v1.0.0"));
    }

    // =========================================================================
    // PORT MAPPING MATCHING
    // =========================================================================

    #[test]
    fn test_port_mapping() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "8080->80/tcp"));
        assert!(any_rule_matches(&rules, "3306->3306/tcp"));
        assert!(any_rule_matches(&rules, "53->53/udp"));
    }

    #[test]
    fn test_port_with_ip() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "0.0.0.0:8080"));
        assert!(any_rule_matches(&rules, "127.0.0.1:3000"));
        assert!(any_rule_matches(&rules, "192.168.1.1:443"));
    }

    // =========================================================================
    // LOG LEVEL MATCHING
    // =========================================================================

    #[test]
    fn test_log_levels() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "ERROR: container failed to start"));
        assert!(any_rule_matches(&rules, "WARN: deprecated feature"));
        assert!(any_rule_matches(&rules, "INFO: container started"));
        assert!(any_rule_matches(&rules, "DEBUG: connecting to network"));
    }

    // =========================================================================
    // SIZE AND METRICS MATCHING
    // =========================================================================

    #[test]
    fn test_size_matching() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "SIZE: 100 MB"));
        assert!(any_rule_matches(&rules, "Image size: 1.5 GB"));
        assert!(any_rule_matches(&rules, "Layer: 256 KB"));
    }

    #[test]
    fn test_percentage_matching() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "CPU: 50%"));
        assert!(any_rule_matches(&rules, "Memory: 75%"));
    }

    // =========================================================================
    // TIMESTAMP MATCHING
    // =========================================================================

    #[test]
    fn test_iso_timestamp() {
        let rules = docker_rules();
        assert!(any_rule_matches(&rules, "2024-01-15T10:30:45.123Z"));
        assert!(any_rule_matches(&rules, "2024-01-15 10:30:45"));
    }

    // =========================================================================
    // REAL LOG LINE EXAMPLES
    // =========================================================================

    #[test]
    fn test_docker_ps_output() {
        let rules = docker_rules();
        let line = "abc123def456  nginx:latest  Up 2 hours  0.0.0.0:80->80/tcp  web";
        assert!(any_rule_matches(&rules, line));
    }

    #[test]
    fn test_docker_logs_output() {
        let rules = docker_rules();
        assert!(any_rule_matches(
            &rules,
            "2024-01-15T10:30:45.123Z INFO Starting application"
        ));
        assert!(any_rule_matches(
            &rules,
            "2024-01-15T10:30:46.456Z ERROR Connection refused"
        ));
    }

    #[test]
    fn test_docker_stats_output() {
        let rules = docker_rules();
        let line = "abc123  nginx  50%  100 MB / 512 MB  25%";
        assert!(any_rule_matches(&rules, line));
    }
}
