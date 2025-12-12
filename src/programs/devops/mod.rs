//! DevOps tool programs.
//!
//! Provides Program implementations for Docker, Kubernetes, Terraform, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all DevOps programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(docker_program()));
    registry.register(Arc::new(kubectl_program()));
    registry.register(Arc::new(terraform_program()));
    registry.register(Arc::new(k9s_program()));
    registry.register(Arc::new(helm_program()));
    registry.register(Arc::new(ansible_program()));
    registry.register(Arc::new(docker_compose_program()));
    registry.register(Arc::new(aws_program()));
}

// =============================================================================
// DOCKER
// =============================================================================

fn docker_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Container status
    rules.extend([
        Rule::new(r"\b(running|Running|RUNNING)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(exited|Exited|EXITED|stopped|Stopped)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\b(created|Created|restarting|Restarting)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

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

fn docker_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.docker",
        "Docker",
        "Docker container logs and commands",
        "devops",
        docker_rules(),
    )
    .with_detect_patterns(vec!["docker", "docker-compose", "podman"])
}

// =============================================================================
// KUBECTL
// =============================================================================

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

fn kubectl_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.kubectl",
        "kubectl",
        "Kubernetes kubectl commands and logs",
        "devops",
        kubectl_rules(),
    )
    .with_detect_patterns(vec!["kubectl", "k8s", "kubernetes", "helm"])
}

// =============================================================================
// TERRAFORM
// =============================================================================

fn terraform_rules() -> Vec<Rule> {
    vec![
        // Plan actions - create/add
        Rule::new(r"^\s*\+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bwill be created\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Plan actions - destroy
        Rule::new(r"^\s*-")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bwill be destroyed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Plan actions - change/update
        Rule::new(r"^\s*~")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bwill be updated in-place\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bmust be replaced\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // Plan actions - read
        Rule::new(r"^\s*<=")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bwill be read\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Resource types
        Rule::new(r"\b(aws|azurerm|google|kubernetes|helm|local|null|random|tls|template)_\w+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Resource addresses
        Rule::new(r"\b(module\.[\w\-]+\.)?[\w\-]+\.[\w\-]+(\[\S+\])?")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Known after apply / sensitive
        Rule::new(r"\(known after apply\)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\(sensitive value\)")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Plan summary
        Rule::new(r"\bPlan:\s*\d+\s*to add")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\d+\s*to change")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\d+\s*to destroy")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Apply status
        Rule::new(r"\b(Apply complete|Creation complete|Destruction complete)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bError:\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWarning:\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        // Strings
        Rule::new(r#""[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        // Boolean
        Rule::new(r"\b(true|false)\b")
            .unwrap()
            .semantic(SemanticColor::Boolean)
            .build(),
        // Numbers (last)
        common::number_rule(),
    ]
}

fn terraform_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.terraform",
        "Terraform",
        "Terraform plan/apply output",
        "devops",
        terraform_rules(),
    )
    .with_detect_patterns(vec!["terraform", "tf", "tofu", "opentofu"])
}

// =============================================================================
// K9S
// =============================================================================

fn k9s_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Kubernetes log level format: level=info
    rules.extend([
        Rule::new(r#"level=error"#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r#"level=warn"#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#"level=info"#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#"level=debug"#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Kubernetes resource names
    rules.extend([
        Rule::new(r"\b(pod|deployment|service|configmap|secret|namespace|node|ingress|pvc)s?/[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bnamespace=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Key=value patterns (common in structured logs)
    rules.push(common::key_value_rule());

    // Timestamps
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

fn k9s_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.k9s",
        "k9s",
        "K9s Kubernetes CLI logs",
        "devops",
        k9s_rules(),
    )
    .with_detect_patterns(vec!["k9s"])
}

// =============================================================================
// HELM
// =============================================================================

fn helm_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Release status
    rules.extend([
        Rule::new(r"\bSTATUS:\s*deployed\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(failed|superseded)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(pending|pending-install|pending-upgrade|pending-rollback)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bSTATUS:\s*uninstalled\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Chart info
    rules.extend([
        Rule::new(r"\bCHART:\s*[\w\-]+[\d\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bNAME:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bNAMESPACE:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bREVISION:\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Hooks and resources
    rules.extend([
        Rule::new(r"\b(pre-install|post-install|pre-upgrade|post-upgrade|pre-delete|post-delete)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"created|configured|unchanged")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

fn helm_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.helm",
        "helm",
        "Helm package manager output",
        "devops",
        helm_rules(),
    )
    .with_detect_patterns(vec!["helm"])
}

// =============================================================================
// ANSIBLE
// =============================================================================

fn ansible_rules() -> Vec<Rule> {
    vec![
        // PLAY and TASK headers
        Rule::new(r"^PLAY\s+\[.*\]\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^TASK\s+\[.*\]\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        Rule::new(r"^PLAY RECAP\s*\*+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Task status
        Rule::new(r"^ok:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^changed:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^failed:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^skipping:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^unreachable:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^fatal:\s*\[.*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        // Recap summary counts
        Rule::new(r"\bok=\d+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bchanged=\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bunreachable=\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bfailed=\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bskipped=\d+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\brescued=\d+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bignored=\d+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Host IPs/names in recap
        common::ipv4_rule(),
        common::number_rule(),
    ]
}

fn ansible_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.ansible",
        "ansible",
        "Ansible playbook output",
        "devops",
        ansible_rules(),
    )
    .with_detect_patterns(vec!["ansible", "ansible-playbook"])
}

// =============================================================================
// DOCKER COMPOSE
// =============================================================================

fn docker_compose_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Service prefix pattern: service-1 | log message
    rules.push(
        Rule::new(r"^[\w\-]+\-\d+\s*\|")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Container lifecycle
    rules.extend([
        Rule::new(r"\b(Creating|Starting|Recreating)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(Started|Created|Running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Stopping|Removing|Killing)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(Stopped|Removed|exited with code \d+)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bAttaching to\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Network and volume operations
    rules.extend([
        Rule::new(r"\bNetwork\s+[\w\-]+\s+(Created|Removed)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bVolume\s+[\w\-]+\s+(Created|Removed)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Container and image IDs
    rules.extend(common::hex_id_rules().into_iter().take(2));
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

fn docker_compose_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.docker-compose",
        "docker-compose",
        "Docker Compose logs and output",
        "devops",
        docker_compose_rules(),
    )
    .with_detect_patterns(vec!["docker-compose", "docker compose"])
}

// =============================================================================
// AWS CLI
// =============================================================================

fn aws_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // ARN patterns
    rules.push(
        Rule::new(r"arn:aws:[\w\-]+:[\w\-]*:\d*:[\w\-/\*]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // AWS error patterns
    rules.extend([
        Rule::new(r"An error occurred\s+\([^\)]+\)")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(AccessDenied|InvalidParameter|ResourceNotFound|ValidationError)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
    ]);

    // Service names
    rules.push(
        Rule::new(r"\b(s3|ec2|lambda|rds|iam|cloudformation|cloudwatch|sns|sqs|dynamodb|ecs|eks|route53)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Status values
    rules.extend([
        Rule::new(r"\b(ACTIVE|AVAILABLE|CREATE_COMPLETE|UPDATE_COMPLETE|running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(PENDING|IN_PROGRESS|CREATE_IN_PROGRESS|UPDATE_IN_PROGRESS|pending)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(FAILED|DELETE_FAILED|ROLLBACK_COMPLETE|stopped|terminated)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Resource IDs
    rules.extend([
        Rule::new(r"\bi-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bvpc-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bsubnet-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bsg-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Table headers (common in aws cli output)
    rules.push(
        Rule::new(r"^[\|\+][\-\+]+[\|\+]$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

fn aws_program() -> SimpleProgram {
    SimpleProgram::new(
        "devops.aws",
        "aws",
        "AWS CLI output",
        "devops",
        aws_rules(),
    )
    .with_detect_patterns(vec!["aws"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devops_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 8);
    }

    #[test]
    fn test_docker_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("docker logs mycontainer");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Docker");
    }

    #[test]
    fn test_kubectl_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("kubectl get pods");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "kubectl");
    }

    #[test]
    fn test_terraform_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("terraform plan");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Terraform");
    }
}
