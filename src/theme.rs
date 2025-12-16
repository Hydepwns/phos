//! Theme system for semantic color resolution.
//!
//! Themes map semantic colors (Error, Warn, Info, etc.) to actual colors,
//! allowing the same rules to work across different color schemes.
//!
//! # Built-in Themes
//!
//! phos includes 13 built-in themes:
//!
//! | Theme          | Description                    |
//! |----------------|--------------------------------|
//! | default-dark   | Default dark theme             |
//! | dracula        | Popular dark theme             |
//! | nord           | Arctic blue palette            |
//! | catppuccin     | Pastel colors (Mocha variant)  |
//! | gruvbox        | Retro groove colors            |
//! | monokai        | Classic editor theme           |
//! | solarized      | Precision colors               |
//! | synthwave84    | Retro-futuristic neon          |
//! | tokyo-night    | Modern city aesthetic          |
//! | horizon        | Warm sunset colors             |
//! | matrix         | Green monochrome               |
//! | phosphor       | Amber CRT nostalgia            |
//! | high-contrast  | Maximum readability            |
//!
//! # Examples
//!
//! ```rust
//! use phos::{Theme, SemanticColor, Colorizer};
//!
//! // Use a built-in theme
//! let theme = Theme::dracula();
//!
//! // Or load by name
//! let theme = Theme::builtin("nord").unwrap();
//!
//! // Apply to a colorizer
//! let colorizer = Colorizer::new(vec![])
//!     .with_theme(theme);
//! ```

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::colors::{Color, SemanticColor};

/// Configuration for a user-defined theme.
#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    /// Theme description
    #[serde(default)]
    pub description: String,
    /// Color mappings (semantic name -> hex color)
    #[serde(default)]
    pub colors: HashMap<String, String>,
    /// Optional base palette
    #[serde(default)]
    pub palette: Option<PaletteConfig>,
}

/// Palette configuration for user themes.
#[derive(Debug, Deserialize)]
pub struct PaletteConfig {
    pub red: String,
    pub orange: String,
    pub green: String,
    pub cyan: String,
    pub blue: String,
    pub purple: String,
    #[serde(default = "default_gray")]
    pub gray: String,
    #[serde(default = "default_dim")]
    pub dim: String,
    #[serde(default = "default_fg")]
    pub foreground: String,
}

fn default_gray() -> String { "#888888".to_string() }
fn default_dim() -> String { "#666666".to_string() }
fn default_fg() -> String { "#FFFFFF".to_string() }

/// Error loading a theme configuration.
#[derive(Debug, thiserror::Error)]
pub enum ThemeLoadError {
    #[error("Failed to read theme file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Unknown file format: {0}")]
    UnknownFormat(String),
}

/// A theme that maps semantic colors to actual colors.
///
/// Themes provide the color resolution layer between abstract semantic colors
/// and concrete terminal colors. This separation allows rules to be portable
/// across different color schemes.
///
/// # Examples
///
/// ```rust
/// use phos::{Theme, SemanticColor, Color};
///
/// // Use a built-in theme
/// let theme = Theme::nord();
///
/// // Resolve a semantic color
/// if let Some(color) = theme.resolve(SemanticColor::Error) {
///     println!("Error color: {:?}", color);
/// }
///
/// // Create a custom theme
/// let mut custom = Theme::new("my-theme");
/// custom.set(SemanticColor::Error, Color::hex("#FF0000"));
/// ```
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Semantic color mappings
    colors: HashMap<SemanticColor, Color>,
}

/// A color palette defining the base colors for a theme.
///
/// Palettes provide a simplified way to define themes using 9 base colors.
/// Semantic colors are automatically derived from these using standard mappings.
///
/// This is used internally by built-in themes but can also be used to create
/// custom themes with consistent color relationships.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub red: &'static str,
    pub orange: &'static str,
    pub green: &'static str,
    pub cyan: &'static str,
    pub blue: &'static str,
    pub purple: &'static str,
    pub gray: &'static str,       // muted/comment color
    pub dim: &'static str,        // even more muted (for trace)
    pub foreground: &'static str, // main text color
}

impl Palette {
    /// Convert palette to a theme using standard semantic mappings.
    const fn to_colors(self) -> [(SemanticColor, &'static str); 16] {
        use SemanticColor::*;
        [
            (Error, self.red),
            (Warn, self.orange),
            (Info, self.blue),
            (Debug, self.gray),
            (Trace, self.dim),
            (Number, self.purple),
            (String, self.green),
            (Boolean, self.purple),
            (Success, self.green),
            (Failure, self.red),
            (Timestamp, self.gray),
            (Key, self.cyan),
            (Value, self.foreground),
            (Identifier, self.cyan),
            (Label, self.cyan),
            (Metric, self.purple),
        ]
    }
}

/// Built-in theme definition.
struct ThemeDef {
    name: &'static str,
    description: &'static str,
    palette: Palette,
}

// All built-in themes defined as data
static BUILTIN_THEMES: &[ThemeDef] = &[
    ThemeDef {
        name: "default-dark",
        description: "Default dark theme",
        palette: Palette {
            red: "#FF5555", orange: "#FFAA00", green: "#AAFFAA",
            cyan: "#88FFFF", blue: "#55AAFF", purple: "#FF88FF",
            gray: "#888888", dim: "#666666", foreground: "#FFFFFF",
        },
    },
    ThemeDef {
        name: "dracula",
        description: "Dracula color scheme",
        // red=#FF5555, orange=#FFB86C, yellow=#F1FA8C, green=#50FA7B
        // cyan=#8BE9FD, purple=#BD93F9, pink=#FF79C6, comment=#6272A4
        palette: Palette {
            red: "#FF5555", orange: "#FFB86C", green: "#F1FA8C",
            cyan: "#8BE9FD", blue: "#8BE9FD", purple: "#BD93F9",
            gray: "#6272A4", dim: "#6272A4", foreground: "#F8F8F2",
        },
    },
    ThemeDef {
        name: "nord",
        description: "Nord arctic theme",
        // red=#BF616A, orange=#D08770, yellow=#EBCB8B, green=#A3BE8C
        // cyan=#88C0D0, blue=#81A1C1, purple=#B48EAD, comment=#4C566A
        palette: Palette {
            red: "#BF616A", orange: "#D08770", green: "#A3BE8C",
            cyan: "#88C0D0", blue: "#81A1C1", purple: "#B48EAD",
            gray: "#4C566A", dim: "#4C566A", foreground: "#ECEFF4",
        },
    },
    ThemeDef {
        name: "catppuccin",
        description: "Catppuccin Mocha",
        // red=#F38BA8, peach=#FAB387, yellow=#F9E2AF, green=#A6E3A1
        // teal=#94E2D5, blue=#89B4FA, mauve=#CBA6F7, overlay=#6C7086
        palette: Palette {
            red: "#F38BA8", orange: "#FAB387", green: "#A6E3A1",
            cyan: "#94E2D5", blue: "#89B4FA", purple: "#CBA6F7",
            gray: "#6C7086", dim: "#6C7086", foreground: "#CDD6F4",
        },
    },
    ThemeDef {
        name: "synthwave84",
        description: "Retro-futuristic neon aesthetic",
        palette: Palette {
            red: "#FE4450", orange: "#FEDE5D", green: "#72F1B8",
            cyan: "#03EDF9", blue: "#03EDF9", purple: "#FF7EDB",
            gray: "#848BBD", dim: "#495495", foreground: "#FFFFFF",
        },
    },
    ThemeDef {
        name: "gruvbox",
        description: "Retro groove with earthy colors",
        // red=#FB4934, orange=#FE8019, yellow=#FABD2F, green=#B8BB26
        // aqua=#8EC07C, blue=#83A598, purple=#D3869B, gray=#928374
        palette: Palette {
            red: "#FB4934", orange: "#FE8019", green: "#B8BB26",
            cyan: "#8EC07C", blue: "#83A598", purple: "#D3869B",
            gray: "#928374", dim: "#928374", foreground: "#EBDBB2",
        },
    },
    ThemeDef {
        name: "monokai",
        description: "Classic editor color scheme",
        // red=#F92672, orange=#FD971F, yellow=#E6DB74, green=#A6E22E
        // cyan=#66D9EF, purple=#AE81FF, comment=#75715E
        palette: Palette {
            red: "#F92672", orange: "#FD971F", green: "#E6DB74",
            cyan: "#66D9EF", blue: "#66D9EF", purple: "#AE81FF",
            gray: "#75715E", dim: "#75715E", foreground: "#F8F8F2",
        },
    },
    ThemeDef {
        name: "solarized",
        description: "Precision colors, dark variant",
        // red=#DC322F, orange=#CB4B16, yellow=#B58900, green=#859900
        // cyan=#2AA198, blue=#268BD2, violet=#6C71C4, base01=#586E75
        palette: Palette {
            red: "#DC322F", orange: "#CB4B16", green: "#859900",
            cyan: "#2AA198", blue: "#268BD2", purple: "#6C71C4",
            gray: "#586E75", dim: "#586E75", foreground: "#93A1A1",
        },
    },
    ThemeDef {
        name: "matrix",
        description: "Green monochrome hacker aesthetic",
        // All greens: bright=#00FF00, normal=#00DD00, dim=#00AA00, dark=#007700, darker=#005500
        palette: Palette {
            red: "#00FF00", orange: "#00DD00", green: "#00AA00",
            cyan: "#00DD00", blue: "#00AA00", purple: "#00DD00",
            gray: "#007700", dim: "#005500", foreground: "#00AA00",
        },
    },
    ThemeDef {
        name: "phosphor",
        description: "Amber CRT terminal nostalgia",
        // All ambers: bright=#FFCC00, normal=#FFAA00, dim=#DD8800, dark=#AA6600, darker=#774400
        palette: Palette {
            red: "#FFCC00", orange: "#FFAA00", green: "#DD8800",
            cyan: "#FFAA00", blue: "#DD8800", purple: "#FFAA00",
            gray: "#AA6600", dim: "#774400", foreground: "#DD8800",
        },
    },
    ThemeDef {
        name: "tokyo-night",
        description: "Modern city lights aesthetic",
        // red=#F7768E, orange=#FF9E64, yellow=#E0AF68, green=#9ECE6A
        // teal=#73DACA, cyan=#7DCFFF, blue=#7AA2F7, purple=#BB9AF7, comment=#565F89
        palette: Palette {
            red: "#F7768E", orange: "#FF9E64", green: "#9ECE6A",
            cyan: "#73DACA", blue: "#7AA2F7", purple: "#BB9AF7",
            gray: "#565F89", dim: "#565F89", foreground: "#A9B1D6",
        },
    },
    ThemeDef {
        name: "horizon",
        description: "Warm sunset colors",
        // red=#E95678, orange=#FAB795, yellow=#FAC29A, green=#29D398
        // cyan=#59E3E3, blue=#26BBD9, purple=#EE64AE, comment=#6C6F93
        palette: Palette {
            red: "#E95678", orange: "#FAB795", green: "#29D398",
            cyan: "#59E3E3", blue: "#26BBD9", purple: "#EE64AE",
            gray: "#6C6F93", dim: "#6C6F93", foreground: "#FDF0ED",
        },
    },
    ThemeDef {
        name: "high-contrast",
        description: "Maximum readability",
        palette: Palette {
            red: "#FF0000", orange: "#FFFF00", green: "#00FF00",
            cyan: "#00FFFF", blue: "#00FFFF", purple: "#FF00FF",
            gray: "#888888", dim: "#666666", foreground: "#FFFFFF",
        },
    },
];

impl Theme {
    /// Create a new empty theme.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            colors: HashMap::new(),
        }
    }

    /// Create a theme from a palette of color mappings.
    pub fn from_palette(name: &str, desc: &str, colors: &[(SemanticColor, &str)]) -> Self {
        Self {
            name: name.to_string(),
            description: desc.to_string(),
            colors: colors.iter().map(|(sem, hex)| (*sem, Color::hex(hex))).collect(),
        }
    }

    /// Set a semantic color mapping.
    pub fn set(&mut self, semantic: SemanticColor, color: Color) {
        self.colors.insert(semantic, color);
    }

    /// Resolve a semantic color to its actual color.
    pub fn resolve(&self, semantic: SemanticColor) -> Option<&Color> {
        self.colors.get(&semantic)
    }

    /// Resolve a color, handling semantic colors.
    pub fn resolve_color(&self, color: &Color) -> Color {
        match color {
            Color::Semantic(s) => self.resolve(*s).cloned().unwrap_or_else(|| color.clone()),
            _ => color.clone(),
        }
    }

    /// Validate that all semantic colors are defined in this theme.
    ///
    /// Returns a list of missing semantic colors. An empty list means the theme
    /// is complete and will properly colorize all semantic color references.
    pub fn validate(&self) -> Vec<SemanticColor> {
        SemanticColor::ALL
            .iter()
            .copied()
            .filter(|sc| !self.colors.contains_key(sc))
            .collect()
    }

    /// Get a built-in theme by name.
    pub fn builtin(name: &str) -> Option<Self> {
        let name_lower = name.to_lowercase();
        BUILTIN_THEMES
            .iter()
            .find(|def| def.name == name_lower)
            .map(|def| {
                let colors = def.palette.to_colors();
                Self::from_palette(def.name, def.description, &colors)
            })
    }

    /// List available built-in themes.
    pub fn list_builtin() -> Vec<&'static str> {
        BUILTIN_THEMES.iter().map(|def| def.name).collect()
    }

    /// Load a theme from a configuration file.
    ///
    /// Supports YAML and JSON formats. The file should contain a theme
    /// configuration with name, description, and either direct color mappings
    /// or a palette.
    ///
    /// # Examples
    ///
    /// YAML format with direct colors:
    /// ```yaml
    /// name: my-theme
    /// description: My custom theme
    /// colors:
    ///   error: "#FF0000"
    ///   warn: "#FFAA00"
    ///   info: "#00AAFF"
    /// ```
    ///
    /// YAML format with palette:
    /// ```yaml
    /// name: my-theme
    /// description: My custom theme
    /// palette:
    ///   red: "#FF5555"
    ///   orange: "#FFAA00"
    ///   green: "#AAFFAA"
    ///   cyan: "#88FFFF"
    ///   blue: "#55AAFF"
    ///   purple: "#FF88FF"
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ThemeLoadError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let config: ThemeConfig = match extension.to_lowercase().as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&content)?,
            "json" => serde_json::from_str(&content)?,
            ext => return Err(ThemeLoadError::UnknownFormat(ext.to_string())),
        };

        Ok(Self::from_config(config))
    }

    /// Create a theme from a configuration.
    pub fn from_config(config: ThemeConfig) -> Self {
        // Build base colors from palette if provided
        let palette_colors: HashMap<SemanticColor, Color> = config
            .palette
            .map(|palette| {
                [
                    (SemanticColor::Error, &palette.red),
                    (SemanticColor::Warn, &palette.orange),
                    (SemanticColor::Info, &palette.blue),
                    (SemanticColor::Debug, &palette.gray),
                    (SemanticColor::Trace, &palette.dim),
                    (SemanticColor::Number, &palette.purple),
                    (SemanticColor::String, &palette.green),
                    (SemanticColor::Boolean, &palette.purple),
                    (SemanticColor::Success, &palette.green),
                    (SemanticColor::Failure, &palette.red),
                    (SemanticColor::Timestamp, &palette.gray),
                    (SemanticColor::Key, &palette.cyan),
                    (SemanticColor::Value, &palette.foreground),
                    (SemanticColor::Identifier, &palette.cyan),
                    (SemanticColor::Label, &palette.cyan),
                    (SemanticColor::Metric, &palette.purple),
                ]
                .into_iter()
                .map(|(sem, hex)| (sem, Color::hex(hex)))
                .collect()
            })
            .unwrap_or_default();

        // Override with specific color mappings
        let override_colors: HashMap<SemanticColor, Color> = config
            .colors
            .iter()
            .filter_map(|(name, hex)| {
                SemanticColor::from_name(name).map(|sem| (sem, Color::hex(hex)))
            })
            .collect();

        // Merge: overrides take precedence over palette
        let colors = palette_colors
            .into_iter()
            .chain(override_colors)
            .collect();

        Self {
            name: config.name,
            description: config.description,
            colors,
        }
    }

    /// Load a user theme by name from the themes directory.
    ///
    /// Searches `~/.config/phos/themes/` for a file matching the theme name.
    pub fn load_user_theme(name: &str) -> Option<Self> {
        let themes_dir = crate::program::loader::themes_dir()?;
        if !themes_dir.exists() {
            return None;
        }

        // Try common extensions
        for ext in ["yaml", "yml", "json"] {
            let path = themes_dir.join(format!("{name}.{ext}"));
            if path.exists() {
                return Self::load_from_file(&path).ok();
            }
        }

        None
    }

    /// Get a theme by name, checking user themes first, then built-in.
    pub fn get(name: &str) -> Option<Self> {
        Self::load_user_theme(name).or_else(|| Self::builtin(name))
    }

    /// Default dark theme.
    pub fn default_dark() -> Self {
        Self::builtin("default-dark").expect("default-dark theme must exist")
    }

    /// Dracula theme.
    pub fn dracula() -> Self {
        Self::builtin("dracula").expect("dracula theme must exist")
    }

    /// Nord theme.
    pub fn nord() -> Self {
        Self::builtin("nord").expect("nord theme must exist")
    }

    /// Catppuccin Mocha theme.
    pub fn catppuccin() -> Self {
        Self::builtin("catppuccin").expect("catppuccin theme must exist")
    }

    /// Synthwave84 retro-futuristic theme.
    pub fn synthwave84() -> Self {
        Self::builtin("synthwave84").expect("synthwave84 theme must exist")
    }

    /// Gruvbox dark theme.
    pub fn gruvbox() -> Self {
        Self::builtin("gruvbox").expect("gruvbox theme must exist")
    }

    /// Monokai classic theme.
    pub fn monokai() -> Self {
        Self::builtin("monokai").expect("monokai theme must exist")
    }

    /// Solarized dark theme.
    pub fn solarized() -> Self {
        Self::builtin("solarized").expect("solarized theme must exist")
    }

    /// Matrix green monochrome theme.
    pub fn matrix() -> Self {
        Self::builtin("matrix").expect("matrix theme must exist")
    }

    /// Phosphor amber monochrome theme.
    pub fn phosphor() -> Self {
        Self::builtin("phosphor").expect("phosphor theme must exist")
    }

    /// Tokyo Night theme.
    pub fn tokyo_night() -> Self {
        Self::builtin("tokyo-night").expect("tokyo-night theme must exist")
    }

    /// Horizon theme.
    pub fn horizon() -> Self {
        Self::builtin("horizon").expect("horizon theme must exist")
    }

    /// High contrast theme.
    pub fn high_contrast() -> Self {
        Self::builtin("high-contrast").expect("high-contrast theme must exist")
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_themes() {
        for name in Theme::list_builtin() {
            let theme = Theme::builtin(name);
            assert!(theme.is_some(), "Theme '{name}' should exist");
        }
    }

    #[test]
    fn test_semantic_resolution() {
        let theme = Theme::default_dark();
        let resolved = theme.resolve(SemanticColor::Error);
        assert!(resolved.is_some());
    }

    #[test]
    fn test_all_builtin_themes_complete() {
        for name in Theme::list_builtin() {
            let theme = Theme::builtin(name).expect("Theme should exist");
            let missing = theme.validate();
            assert!(
                missing.is_empty(),
                "Theme '{name}' is missing semantic colors: {missing:?}"
            );
        }
    }

    #[test]
    fn test_palette_generates_all_colors() {
        let palette = Palette {
            red: "#FF0000", orange: "#FF8800", green: "#00FF00",
            cyan: "#00FFFF", blue: "#0000FF", purple: "#FF00FF",
            gray: "#888888", dim: "#444444", foreground: "#FFFFFF",
        };
        let colors = palette.to_colors();
        assert_eq!(colors.len(), 16);
    }
}
