//! CI/CD tool programs.
//!
//! Provides Program implementations for GitHub Actions, Jenkins, etc.

mod github_actions;
mod jenkins;

use crate::program::ProgramRegistry;

pub use github_actions::github_actions_program;
pub use jenkins::jenkins_program;

/// Register all CI programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(github_actions_program());
    registry.register(jenkins_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_jenkins_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("jenkins build");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "jenkins");
    }
}
