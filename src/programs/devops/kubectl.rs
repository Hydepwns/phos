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
