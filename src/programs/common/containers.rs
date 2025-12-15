//! Container and Kubernetes patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Container lifecycle states (Docker, Podman).
pub fn container_status_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(running|Running|RUNNING|Up)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(exited|Exited|EXITED|stopped|Stopped)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\b(created|Created|restarting|Restarting|paused|Paused)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(dead|Dead|removing|Removing)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
    ]
}

/// Kubernetes pod and resource states.
pub fn k8s_status_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(Running|Succeeded|Bound|Available|Ready)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Pending|ContainerCreating|PodInitializing|Terminating)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(Failed|Error|Unknown|CrashLoopBackOff|ImagePullBackOff|ErrImagePull|OOMKilled)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(Evicted|NodeLost|Unschedulable)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]
}

/// Kubernetes resource types.
pub fn k8s_resource_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(pod|deployment|service|configmap|secret|namespace|node|replicaset|daemonset|statefulset|job|cronjob|ingress|pvc|pv)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_status_rules_compile() {
        let rules = container_status_rules();
        assert_eq!(rules.len(), 4);
    }

    #[test]
    fn test_k8s_status_rules_compile() {
        let rules = k8s_status_rules();
        assert_eq!(rules.len(), 4);
    }
}
