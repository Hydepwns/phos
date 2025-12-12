//! Configuration directory discovery and loading.

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use super::config::{ConfigError, ProgramConfig};
use super::{Program, ProgramRegistry};

/// Get the user configuration directory.
/// Returns ~/.config/phos on Unix, or appropriate equivalent on other platforms.
pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("phos"))
}

/// Get the programs directory.
pub fn programs_dir() -> Option<PathBuf> {
    config_dir().map(|p| p.join("programs"))
}

/// Get the themes directory.
pub fn themes_dir() -> Option<PathBuf> {
    config_dir().map(|p| p.join("themes"))
}

/// Load all user-defined programs from the config directory.
pub fn load_user_programs(registry: &mut ProgramRegistry) -> Vec<ConfigError> {
    let programs_path = match programs_dir() {
        Some(p) if p.exists() => p,
        _ => return Vec::new(),
    };

    let entries = match fs::read_dir(&programs_path) {
        Ok(entries) => entries,
        Err(e) => return vec![ConfigError::ReadError(e)],
    };

    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| matches!(ext.to_lowercase().as_str(), "yaml" | "yml" | "json"))
        })
        .filter_map(|path| {
            load_program_from_file(&path)
                .map(|program| { registry.register(program); None })
                .unwrap_or_else(Some)
        })
        .collect()
}

/// Load a single program from a configuration file.
pub fn load_program_from_file(path: &PathBuf) -> Result<Arc<dyn Program>, ConfigError> {
    let config = ProgramConfig::load(path)?;
    config.to_program()
}

/// Ensure the config directory structure exists.
pub fn ensure_config_dirs() -> std::io::Result<()> {
    [programs_dir(), themes_dir()]
        .into_iter()
        .flatten()
        .try_for_each(fs::create_dir_all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let dir = config_dir();
        assert!(dir.is_some());
        let dir = dir.unwrap();
        assert!(dir.to_string_lossy().contains("phos"));
    }

    #[test]
    fn test_programs_dir() {
        let dir = programs_dir();
        assert!(dir.is_some());
        let dir = dir.unwrap();
        assert!(dir.to_string_lossy().contains("programs"));
    }
}
