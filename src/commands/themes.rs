//! Theme listing command.

use anyhow::Result;
use phos::Theme;

/// List all available built-in themes.
#[allow(clippy::unnecessary_wraps)]
pub fn list_themes() -> Result<()> {
    println!("Available themes:\n");
    Theme::list_builtin()
        .iter()
        .filter_map(|name| Theme::builtin(name).map(|t| (name, t)))
        .for_each(|(name, theme)| {
            println!("  {:12} - {}", name, theme.description);
        });
    Ok(())
}
