//! Built-in program implementations.
//!
//! Provides high-performance, compiled-in programs for various log formats.

pub mod ci;
pub mod common;
pub mod data;
pub mod dev;
pub mod devops;
pub mod ethereum;
pub mod messaging;
pub mod monitoring;
pub mod network;
pub mod system;

use crate::program::ProgramRegistry;

/// Register all built-in programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    ethereum::register_all(registry);
    devops::register_all(registry);
    system::register_all(registry);
    dev::register_all(registry);
    network::register_all(registry);
    data::register_all(registry);
    monitoring::register_all(registry);
    messaging::register_all(registry);
    ci::register_all(registry);
}

/// Create a registry with all built-in programs.
pub fn default_registry() -> ProgramRegistry {
    let mut registry = ProgramRegistry::new();
    register_all(&mut registry);
    registry
}
