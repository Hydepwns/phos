//! Development tool programs.
//!
//! Provides Program implementations for git, cargo, npm, etc.

mod build;
mod lang;
mod node;
mod rust;
mod vcs;

use super::common;
use crate::program::ProgramRegistry;

/// Register all Dev programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    // Version control
    registry.register(vcs::git_program());
    registry.register(vcs::diff_program());
    registry.register(vcs::wdiff_program());

    // Rust
    registry.register(rust::cargo_program());

    // Node.js
    registry.register(node::npm_program());
    registry.register(node::yarn_program());
    registry.register(node::pnpm_program());

    // Build tools
    registry.register(build::make_program());
    registry.register(build::gcc_program());
    registry.register(build::configure_program());
    registry.register(build::ant_program());
    registry.register(build::mvn_program());

    // Languages
    registry.register(lang::go_program());
    registry.register(lang::elixir_program());
    registry.register(lang::php_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 15);
    }

    #[test]
    fn test_git_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("git diff");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "git");
    }

    #[test]
    fn test_cargo_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("cargo test");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "cargo");
    }

    #[test]
    fn test_npm_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("npm install");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "npm");
    }
}
