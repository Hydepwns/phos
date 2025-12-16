//! Color types and ANSI code generation.
//!
//! This module provides color representations for terminal output:
//!
//! - [`Color`]: Concrete color values (named, hex, RGB, or semantic)
//! - [`SemanticColor`]: Abstract colors resolved by themes (Error, Warn, Info, etc.)
//! - [`ColorSpec`]: Flexible color specification for rule definitions
//!
//! # Examples
//!
//! ```rust
//! use phos::colors::{Color, SemanticColor};
//!
//! // Create colors in different formats
//! let named = Color::named("red");
//! let hex = Color::hex("#FF5555");
//! let rgb = Color::rgb(255, 85, 85);
//! let semantic = Color::semantic(SemanticColor::Error);
//! ```

use nu_ansi_term::{Color as AnsiColor, Style};

/// Color representation for styling text.
///
/// Colors can be specified in multiple formats:
/// - **Named**: Standard ANSI colors like "red", "`bright_blue`"
/// - **Hex**: Web colors like "#FF5555"
/// - **RGB**: Explicit RGB values
/// - **Semantic**: Abstract colors resolved by the current theme
///
/// # Examples
///
/// ```rust
/// use phos::Color;
///
/// let color = Color::hex("#FF5555");
/// let style = color.to_style();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Named color (e.g., "red", "`bright_blue`")
    Named(String),
    /// Hex color (e.g., "#FF5555")
    Hex(String),
    /// RGB color
    Rgb { r: u8, g: u8, b: u8 },
    /// Semantic color resolved by theme
    Semantic(SemanticColor),
}

/// Semantic colors that themes resolve to actual colors.
///
/// Semantic colors are abstract color concepts that themes map to concrete colors.
/// This allows rules to be written once and work across all themes.
///
/// # Categories
///
/// - **Log levels**: Error, Warn, Info, Debug, Trace
/// - **Data types**: Number, String, Boolean
/// - **Structure**: Timestamp, Key, Value
/// - **Status**: Success, Failure
/// - **Identifiers**: Identifier, Label, Metric
///
/// # Examples
///
/// ```rust
/// use phos::{SemanticColor, Theme};
///
/// let theme = Theme::dracula();
/// let color = theme.resolve(SemanticColor::Error);
/// assert!(color.is_some()); // Dracula defines all semantic colors
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticColor {
    // Log levels
    Error,
    Warn,
    Info,
    Debug,
    Trace,

    // Data types
    Number,
    String,
    Boolean,

    // Structure
    Timestamp,
    Key,
    Value,

    // Status
    Success,
    Failure,

    // Generic identifiers
    Identifier, // Generic hash, ID, UUID, etc.
    Label,      // Generic tag, label, category
    Metric,     // Numeric measurements, durations
}

/// Color specification that can reference universal semantics or domain colors.
///
/// Used in configuration files to specify colors flexibly. The parser
/// tries to interpret color names in this order:
/// 1. Semantic color (error, warn, info, etc.)
/// 2. Hex color (starts with #)
/// 3. Named ANSI color (red, blue, etc.)
/// 4. Domain-specific color (resolved by program)
#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpec {
    /// Universal semantic color (resolved by theme)
    Semantic(SemanticColor),
    /// Domain-specific color (resolved by program's `domain_colors`)
    Domain(String),
    /// Named ANSI color
    Named(String),
    /// Hex color
    Hex(String),
}

impl ColorSpec {
    /// Create from a color name, trying to parse as semantic first.
    #[must_use] pub fn from_name(name: &str) -> Self {
        if let Some(semantic) = SemanticColor::from_name(name) {
            Self::Semantic(semantic)
        } else if name.starts_with('#') {
            Self::Hex(name.to_string())
        } else if is_ansi_color(name) {
            Self::Named(name.to_string())
        } else {
            // Assume it's a domain-specific color
            Self::Domain(name.to_string())
        }
    }
}

impl SemanticColor {
    /// All semantic color variants for validation.
    pub const ALL: &'static [SemanticColor] = &[
        SemanticColor::Error,
        SemanticColor::Warn,
        SemanticColor::Info,
        SemanticColor::Debug,
        SemanticColor::Trace,
        SemanticColor::Number,
        SemanticColor::String,
        SemanticColor::Boolean,
        SemanticColor::Timestamp,
        SemanticColor::Key,
        SemanticColor::Value,
        SemanticColor::Success,
        SemanticColor::Failure,
        SemanticColor::Identifier,
        SemanticColor::Label,
        SemanticColor::Metric,
    ];

    /// Parse a semantic color from its name.
    #[must_use] pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            // Log levels
            "error" => Some(Self::Error),
            "warn" | "warning" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            "trace" => Some(Self::Trace),
            // Data types
            "number" => Some(Self::Number),
            "string" => Some(Self::String),
            "boolean" | "bool" => Some(Self::Boolean),
            // Structure
            "timestamp" | "time" => Some(Self::Timestamp),
            "key" => Some(Self::Key),
            "value" => Some(Self::Value),
            // Status
            "success" => Some(Self::Success),
            "failure" | "fail" => Some(Self::Failure),
            // Generic identifiers
            "identifier" | "id" => Some(Self::Identifier),
            "label" | "tag" => Some(Self::Label),
            "metric" | "measure" => Some(Self::Metric),

            _ => None,
        }
    }
}

/// Check if a name is a standard ANSI color.
fn is_ansi_color(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "black"
            | "red"
            | "green"
            | "yellow"
            | "blue"
            | "magenta"
            | "purple"
            | "cyan"
            | "white"
            | "gray"
            | "grey"
            | "bright_black"
            | "bright_red"
            | "bright_green"
            | "bright_yellow"
            | "bright_blue"
            | "bright_magenta"
            | "bright_cyan"
            | "bright_white"
    )
}

impl Color {
    /// Create a semantic color.
    #[must_use] pub fn semantic(s: SemanticColor) -> Self {
        Self::Semantic(s)
    }

    /// Create a named color.
    #[must_use] pub fn named(name: &str) -> Self {
        Self::Named(name.to_string())
    }

    /// Create a hex color.
    #[must_use] pub fn hex(hex: &str) -> Self {
        Self::Hex(hex.to_string())
    }

    /// Create an RGB color.
    #[must_use] pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Convert to `nu_ansi_term` Style.
    #[must_use] pub fn to_style(&self) -> Style {
        match self {
            Color::Named(name) => Self::named_to_style(name),
            Color::Hex(hex) => Self::hex_to_style(hex),
            Color::Rgb { r, g, b } => Style::new().fg(AnsiColor::Rgb(*r, *g, *b)),
            Color::Semantic(_) => Style::new(), // Resolved by theme
        }
    }

    fn named_to_style(name: &str) -> Style {
        let color = match name.to_lowercase().as_str() {
            "black" => AnsiColor::Black,
            "red" => AnsiColor::Red,
            "green" => AnsiColor::Green,
            "yellow" => AnsiColor::Yellow,
            "blue" => AnsiColor::Blue,
            "magenta" | "purple" => AnsiColor::Magenta,
            "cyan" => AnsiColor::Cyan,
            "white" => AnsiColor::White,
            "bright_black" | "gray" | "grey" => AnsiColor::DarkGray,
            "bright_red" => AnsiColor::LightRed,
            "bright_green" => AnsiColor::LightGreen,
            "bright_yellow" => AnsiColor::LightYellow,
            "bright_blue" => AnsiColor::LightBlue,
            "bright_magenta" => AnsiColor::LightMagenta,
            "bright_cyan" => AnsiColor::LightCyan,
            "bright_white" => AnsiColor::LightGray,
            _ => AnsiColor::Default,
        };
        Style::new().fg(color)
    }

    fn hex_to_style(hex: &str) -> Style {
        parse_hex_rgb(hex)
            .map(|(r, g, b)| Style::new().fg(AnsiColor::Rgb(r, g, b)))
            .unwrap_or_default()
    }
}

/// Parse a hex color string to RGB components.
///
/// Accepts formats: "#RRGGBB", "RRGGBB"
/// Returns None if the string is invalid.
#[must_use] pub fn parse_hex_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Brand colors for Ethereum clients.
pub mod brands {
    /// Get brand color hex for a client.
    #[must_use] pub fn color(client: &str) -> Option<&'static str> {
        match client.to_lowercase().as_str() {
            "lighthouse" => Some("#9933FF"),
            "prysm" => Some("#22CC88"),
            "teku" => Some("#3366FF"),
            "nimbus" => Some("#CC9933"),
            "lodestar" => Some("#AA44FF"),
            "grandine" => Some("#FF6633"),
            "lambda" => Some("#9966FF"),
            "geth" => Some("#6699FF"),
            "nethermind" => Some("#33CCCC"),
            "besu" => Some("#009999"),
            "erigon" => Some("#66CC33"),
            "reth" => Some("#FF9966"),
            "mana" => Some("#CC66FF"),
            "charon" => Some("#6633FF"),
            "mevboost" | "mev-boost" | "mev_boost" => Some("#FF6699"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        let color = Color::hex("#FF5555");
        let _style = color.to_style();
    }

    #[test]
    fn test_brand_colors() {
        assert!(brands::color("geth").is_some());
        assert!(brands::color("unknown").is_none());
    }
}
