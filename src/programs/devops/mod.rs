//! DevOps tool programs.
//!
//! Provides Program implementations for Docker, Kubernetes, Terraform, etc.

mod ansible;
mod aws;
mod docker;
mod docker_compose;
mod helm;
mod k9s;
mod kubectl;
mod terraform;

use crate::program::ProgramRegistry;

pub use ansible::ansible_program;
pub use aws::aws_program;
pub use docker::docker_program;
pub use docker_compose::docker_compose_program;
pub use helm::helm_program;
pub use k9s::k9s_program;
pub use kubectl::kubectl_program;
pub use terraform::terraform_program;

/// Register all DevOps programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(docker_program());
    registry.register(kubectl_program());
    registry.register(terraform_program());
    registry.register(k9s_program());
    registry.register(helm_program());
    registry.register(ansible_program());
    registry.register(docker_compose_program());
    registry.register(aws_program());
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
