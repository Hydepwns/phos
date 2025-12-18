//! Kubernetes kubectl colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn kubectl_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Pod status
    rules.extend([
        Rule::new(r"\b(Running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Pending|ContainerCreating|Init:\d+/\d+)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(Failed|Error|CrashLoopBackOff|ImagePullBackOff|ErrImagePull)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b(Completed|Succeeded)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Terminating|Unknown)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Resource types
    rules.push(
        Rule::new(r"\b(pod|pods|deployment|deployments|service|services|configmap|secret|namespace|node|ingress|pvc|pv)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Kubernetes names (namespace/name pattern)
    rules.push(
        Rule::new(r"\b[a-z0-9]([-a-z0-9]*[a-z0-9])?/[a-z0-9]([-a-z0-9]*[a-z0-9])?\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Ready counts, restarts, resources
    rules.extend([
        Rule::new(r"\b\d+/\d+\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\b\d+[dhms]\b")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"RESTARTS?\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\d+m\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d+(Gi|Mi|Ki)\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Common patterns
    rules.push(common::ipv4_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn kubectl_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.kubectl",
            "kubectl",
            "Kubernetes kubectl commands and logs",
            Category::DevOps,
            kubectl_rules(),
        )
        .with_detect_patterns(vec!["kubectl", "k8s", "kubernetes"]),
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
    fn test_kubectl_rules_compile() {
        let rules = kubectl_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_kubectl_program_info() {
        let program = kubectl_program();
        assert_eq!(program.info().id, "devops.kubectl");
        assert_eq!(program.info().name, "kubectl");
    }

    // =========================================================================
    // POD STATUS MATCHING
    // =========================================================================

    #[test]
    fn test_pod_status_running() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "my-pod  1/1  Running  0  5m"));
        assert!(any_rule_matches(&rules, "STATUS: Running"));
    }

    #[test]
    fn test_pod_status_pending() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "my-pod  0/1  Pending  0  1m"));
        assert!(any_rule_matches(&rules, "status: ContainerCreating"));
        assert!(any_rule_matches(&rules, "Init:1/3"));
    }

    #[test]
    fn test_pod_status_failed() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "my-pod  0/1  Failed  0  10m"));
        assert!(any_rule_matches(&rules, "status: Error"));
        assert!(any_rule_matches(&rules, "CrashLoopBackOff"));
        assert!(any_rule_matches(&rules, "ImagePullBackOff"));
        assert!(any_rule_matches(&rules, "ErrImagePull"));
    }

    #[test]
    fn test_pod_status_completed() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "job-pod  0/1  Completed  0  2h"));
        assert!(any_rule_matches(&rules, "Status: Succeeded"));
    }

    #[test]
    fn test_pod_status_other() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "my-pod  1/1  Terminating  0  1s"));
        assert!(any_rule_matches(&rules, "Status: Unknown"));
    }

    // =========================================================================
    // RESOURCE TYPE MATCHING
    // =========================================================================

    #[test]
    fn test_resource_types() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "kubectl get pod"));
        assert!(any_rule_matches(&rules, "kubectl get pods"));
        assert!(any_rule_matches(&rules, "kubectl get deployment"));
        assert!(any_rule_matches(&rules, "kubectl get service"));
        assert!(any_rule_matches(&rules, "kubectl describe configmap"));
        assert!(any_rule_matches(&rules, "kubectl get secret"));
        assert!(any_rule_matches(&rules, "kubectl get namespace"));
        assert!(any_rule_matches(&rules, "kubectl get node"));
        assert!(any_rule_matches(&rules, "kubectl get ingress"));
        assert!(any_rule_matches(&rules, "kubectl get pvc"));
        assert!(any_rule_matches(&rules, "kubectl get pv"));
    }

    // =========================================================================
    // NAMESPACE/NAME PATTERN MATCHING
    // =========================================================================

    #[test]
    fn test_namespace_name_pattern() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "kube-system/coredns-12345"));
        assert!(any_rule_matches(&rules, "default/my-app"));
        assert!(any_rule_matches(&rules, "production/api-server"));
    }

    // =========================================================================
    // READY COUNTS MATCHING
    // =========================================================================

    #[test]
    fn test_ready_counts() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "READY 1/1"));
        assert!(any_rule_matches(&rules, "0/3 pods ready"));
        assert!(any_rule_matches(&rules, "Replicas: 3/3"));
    }

    // =========================================================================
    // DURATION MATCHING
    // =========================================================================

    #[test]
    fn test_age_duration() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "AGE: 5d"));
        assert!(any_rule_matches(&rules, "Age: 2h"));
        assert!(any_rule_matches(&rules, "running for 30m"));
        assert!(any_rule_matches(&rules, "uptime: 45s"));
    }

    // =========================================================================
    // RESTART COUNT MATCHING
    // =========================================================================

    #[test]
    fn test_restart_count() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "RESTARTS 5"));
        assert!(any_rule_matches(&rules, "RESTART 0"));
    }

    // =========================================================================
    // RESOURCE METRICS MATCHING
    // =========================================================================

    #[test]
    fn test_cpu_metrics() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "cpu: 100m"));
        assert!(any_rule_matches(&rules, "CPU: 500m"));
    }

    #[test]
    fn test_memory_metrics() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "memory: 256Mi"));
        assert!(any_rule_matches(&rules, "Memory: 1Gi"));
        assert!(any_rule_matches(&rules, "512Ki"));
    }

    // =========================================================================
    // IP ADDRESS MATCHING
    // =========================================================================

    #[test]
    fn test_ip_addresses() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "IP: 10.244.0.5"));
        assert!(any_rule_matches(&rules, "ClusterIP: 10.96.0.1"));
        assert!(any_rule_matches(&rules, "NodeIP: 192.168.1.100"));
    }

    // =========================================================================
    // REAL OUTPUT EXAMPLES
    // =========================================================================

    #[test]
    fn test_kubectl_get_pods_output() {
        let rules = kubectl_rules();
        let line = "nginx-deployment-abc123  1/1  Running  0  5d";
        assert!(any_rule_matches(&rules, line));
    }

    #[test]
    fn test_kubectl_describe_output() {
        let rules = kubectl_rules();
        // The rules match specific values, not the label format
        assert!(any_rule_matches(&rules, "default/my-pod"));
        assert!(any_rule_matches(&rules, "192.168.1.10"));
        assert!(any_rule_matches(&rules, "Running"));
    }

    #[test]
    fn test_kubectl_error_output() {
        let rules = kubectl_rules();
        assert!(any_rule_matches(&rules, "Error from server: pod not found"));
        assert!(any_rule_matches(
            &rules,
            "error: the server doesn't have a resource type \"pods\""
        ));
    }
}
