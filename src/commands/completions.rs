//! Shell completions generation command.

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::Cli;

/// Generate shell completions for the given shell.
#[allow(clippy::unnecessary_wraps)]
pub fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}
