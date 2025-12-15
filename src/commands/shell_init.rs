//! Shell integration script generation command.

use anyhow::Result;
use phos::shell::{self, ShellType};
use phos::ProgramRegistry;

/// Generate shell integration script for automatic command colorization.
pub fn generate_shell_init(registry: &ProgramRegistry, shell_name: &str) -> Result<()> {
    let shell_type = ShellType::parse(shell_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown shell: {shell_name}. Supported shells: {}",
            ShellType::supported().join(", ")
        )
    })?;

    let script = shell::generate_script(shell_type, registry);
    print!("{script}");
    Ok(())
}
