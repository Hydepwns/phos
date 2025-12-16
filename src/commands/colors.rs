//! Ethereum brand colors command.

use anyhow::Result;
use phos::programs::ethereum;

/// Display Ethereum client brand colors.
#[allow(clippy::unnecessary_wraps)]
pub fn show_colors() -> Result<()> {
    println!("Ethereum Client Brand Colors:\n");

    for name in ethereum::all_client_names() {
        let name_lower = name.to_lowercase();
        let hex = ethereum::brand_color(&name_lower).unwrap_or("#888888");
        let (r, g, b) = phos::parse_hex_rgb(hex).unwrap_or((128, 128, 128));

        println!("  \x1b[38;2;{r};{g};{b}m##\x1b[0m {name_lower:12} {hex}");
    }

    Ok(())
}
