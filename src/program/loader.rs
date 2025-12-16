//! Configuration directory discovery and loading.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::config::{ConfigError, ProgramConfig};
use super::{Program, ProgramRegistry};

/// Result of loading a user program - includes file path for error reporting.
#[derive(Debug)]
pub struct LoadResult {
    /// Path to the config file that failed to load.
    pub path: PathBuf,
    /// The error that occurred during loading.
    pub error: ConfigError,
}

impl LoadResult {
    /// Format the error with file context.
    pub fn format(&self) -> String {
        let path_str = self.path.display().to_string();
        match &self.error {
            ConfigError::YamlError(e) => {
                let mut msg = format!("{path_str}: YAML parse error");
                if let Some(loc) = e.location() {
                    msg.push_str(&format!(" at line {}", loc.line()));
                }
                msg.push_str(&format!(": {e}"));
                msg
            }
            ConfigError::JsonError(e) => {
                format!("{path_str}: JSON parse error at line {}, column {}: {e}",
                    e.line(), e.column())
            }
            ConfigError::RegexError(e) => {
                format!("{path_str}: invalid regex pattern: {e}\n  Hint: Check for unescaped special characters like \\, [, ], (, )")
            }
            ConfigError::ReadError(e) => {
                format!("{path_str}: failed to read file: {e}")
            }
            ConfigError::UnknownFormat(ext) => {
                format!("{path_str}: unknown file format '.{ext}'\n  Hint: Use .yaml, .yml, or .json")
            }
            _ => format!("{path_str}: {}", self.error),
        }
    }
}

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

/// Get the global configuration file path.
pub fn global_config_path() -> Option<PathBuf> {
    config_dir().map(|p| p.join("config.yaml"))
}

/// Load all user-defined programs from the config directory.
/// Returns a list of load errors with file paths for detailed error reporting.
pub fn load_user_programs(registry: &mut ProgramRegistry) -> Vec<LoadResult> {
    let programs_path = match programs_dir() {
        Some(p) if p.exists() => p,
        _ => return Vec::new(),
    };

    let entries = match fs::read_dir(&programs_path) {
        Ok(entries) => entries,
        Err(e) => return vec![LoadResult {
            path: programs_path,
            error: ConfigError::ReadError(e)
        }],
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
            match load_program_from_file(&path) {
                Ok(program) => {
                    registry.register(program);
                    None
                }
                Err(error) => Some(LoadResult { path, error }),
            }
        })
        .collect()
}

/// Load a single program from a configuration file.
pub fn load_program_from_file(path: &Path) -> Result<Arc<dyn Program>, ConfigError> {
    let config = ProgramConfig::load(path)?;
    config.to_program()
}

/// Validate a program configuration file without loading it into the registry.
/// Returns Ok with the program info if valid, Err with detailed error if not.
pub fn validate_program_file(path: &Path) -> Result<String, LoadResult> {
    match ProgramConfig::load(path) {
        Ok(config) => {
            // Try to convert to program to catch regex errors etc.
            match config.to_program() {
                Ok(program) => {
                    let info = program.info();
                    Ok(format!("{} ({}) - {} rules",
                        info.name, info.id, program.rules().len()))
                }
                Err(error) => Err(LoadResult { path: path.to_path_buf(), error }),
            }
        }
        Err(error) => Err(LoadResult { path: path.to_path_buf(), error }),
    }
}

/// List all config files in the programs directory.
pub fn list_program_files() -> Vec<PathBuf> {
    let programs_path = match programs_dir() {
        Some(p) if p.exists() => p,
        _ => return Vec::new(),
    };

    match fs::read_dir(&programs_path) {
        Ok(entries) => entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| matches!(ext.to_lowercase().as_str(), "yaml" | "yml" | "json"))
            })
            .collect(),
        Err(_) => Vec::new(),
    }
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
