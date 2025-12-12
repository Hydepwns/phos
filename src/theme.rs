//! Theme system for semantic color resolution.

use std::collections::HashMap;

use crate::colors::{Color, SemanticColor};

/// A theme that maps semantic colors to actual colors.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Semantic color mappings
    colors: HashMap<SemanticColor, Color>,
}

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
    /// This is the preferred way to create themes - data-driven and concise.
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

    /// Get a built-in theme by name.
    pub fn builtin(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "default-dark" => Some(Self::default_dark()),
            "dracula" => Some(Self::dracula()),
            "nord" => Some(Self::nord()),
            "catppuccin" => Some(Self::catppuccin()),
            "synthwave84" => Some(Self::synthwave84()),
            "gruvbox" => Some(Self::gruvbox()),
            "monokai" => Some(Self::monokai()),
            "solarized" => Some(Self::solarized()),
            "matrix" => Some(Self::matrix()),
            "phosphor" => Some(Self::phosphor()),
            "tokyo-night" => Some(Self::tokyo_night()),
            "horizon" => Some(Self::horizon()),
            "high-contrast" => Some(Self::high_contrast()),
            _ => None,
        }
    }

    /// List available built-in themes.
    pub fn list_builtin() -> Vec<&'static str> {
        vec![
            "default-dark",
            "dracula",
            "nord",
            "catppuccin",
            "synthwave84",
            "gruvbox",
            "monokai",
            "solarized",
            "matrix",
            "phosphor",
            "tokyo-night",
            "horizon",
            "high-contrast",
        ]
    }

    /// Default dark theme.
    pub fn default_dark() -> Self {
        use SemanticColor::*;
        Self::from_palette("default-dark", "Default dark theme", &[
            // Log levels
            (Error, "#FF5555"), (Warn, "#FFAA00"), (Info, "#55AAFF"),
            (Debug, "#888888"), (Trace, "#666666"),
            // Data types
            (Number, "#AAFFAA"), (String, "#FFFF88"), (Boolean, "#FF88FF"),
            // Ethereum
            (Hash, "#88AAFF"), (Address, "#FFAA88"), (Slot, "#88FFAA"),
            (Epoch, "#AAFFFF"), (BlockNumber, "#FFAAFF"), (PeerId, "#AAAAFF"),
            // Validator
            (Validator, "#AA88FF"), (Pubkey, "#88DDFF"), (Duty, "#FFBB55"), (Committee, "#88AADD"),
            // Consensus
            (Finality, "#88FF88"), (Root, "#88DDFF"), (Attestation, "#FF88DD"),
            // MEV
            (MevValue, "#FFDD55"), (Relay, "#999999"), (Builder, "#FF99BB"),
            // Status
            (Success, "#55FF55"), (Failure, "#FF5555"), (Syncing, "#FFFF55"),
            // Structure
            (Timestamp, "#888888"), (Key, "#88FFFF"), (Value, "#FFFFFF"),
            // Generic
            (Identifier, "#88AAFF"), (Label, "#88FFFF"), (Metric, "#AAFFAA"),
        ])
    }

    /// Dracula theme.
    pub fn dracula() -> Self {
        use SemanticColor::*;
        // Dracula palette: red=#FF5555, orange=#FFB86C, yellow=#F1FA8C, green=#50FA7B
        // cyan=#8BE9FD, purple=#BD93F9, pink=#FF79C6, comment=#6272A4
        Self::from_palette("dracula", "Dracula color scheme", &[
            (Error, "#FF5555"), (Warn, "#FFB86C"), (Info, "#8BE9FD"),
            (Debug, "#6272A4"), (Trace, "#6272A4"),
            (Number, "#BD93F9"), (String, "#F1FA8C"), (Boolean, "#BD93F9"),
            (Hash, "#8BE9FD"), (Address, "#FF79C6"), (Slot, "#50FA7B"),
            (Epoch, "#BD93F9"), (BlockNumber, "#FFB86C"), (PeerId, "#8BE9FD"),
            (Validator, "#BD93F9"), (Pubkey, "#8BE9FD"), (Duty, "#FFB86C"), (Committee, "#BD93F9"),
            (Finality, "#50FA7B"), (Root, "#8BE9FD"), (Attestation, "#FF79C6"),
            (MevValue, "#F1FA8C"), (Relay, "#6272A4"), (Builder, "#FF79C6"),
            (Success, "#50FA7B"), (Failure, "#FF5555"), (Syncing, "#F1FA8C"),
            (Timestamp, "#6272A4"), (Key, "#8BE9FD"), (Value, "#F8F8F2"),
            (Identifier, "#8BE9FD"), (Label, "#8BE9FD"), (Metric, "#BD93F9"),
        ])
    }

    /// Nord theme.
    pub fn nord() -> Self {
        use SemanticColor::*;
        // Nord palette: red=#BF616A, orange=#D08770, yellow=#EBCB8B, green=#A3BE8C
        // cyan=#88C0D0, blue=#81A1C1, purple=#B48EAD, comment=#4C566A
        Self::from_palette("nord", "Nord arctic theme", &[
            (Error, "#BF616A"), (Warn, "#D08770"), (Info, "#81A1C1"),
            (Debug, "#4C566A"), (Trace, "#4C566A"),
            (Number, "#B48EAD"), (String, "#A3BE8C"), (Boolean, "#B48EAD"),
            (Hash, "#88C0D0"), (Address, "#D08770"), (Slot, "#A3BE8C"),
            (Epoch, "#81A1C1"), (BlockNumber, "#EBCB8B"), (PeerId, "#88C0D0"),
            (Validator, "#B48EAD"), (Pubkey, "#88C0D0"), (Duty, "#D08770"), (Committee, "#81A1C1"),
            (Finality, "#A3BE8C"), (Root, "#88C0D0"), (Attestation, "#B48EAD"),
            (MevValue, "#EBCB8B"), (Relay, "#4C566A"), (Builder, "#B48EAD"),
            (Success, "#A3BE8C"), (Failure, "#BF616A"), (Syncing, "#EBCB8B"),
            (Timestamp, "#4C566A"), (Key, "#88C0D0"), (Value, "#ECEFF4"),
            (Identifier, "#88C0D0"), (Label, "#88C0D0"), (Metric, "#B48EAD"),
        ])
    }

    /// Catppuccin Mocha theme.
    pub fn catppuccin() -> Self {
        use SemanticColor::*;
        // Catppuccin Mocha: red=#F38BA8, peach=#FAB387, yellow=#F9E2AF, green=#A6E3A1
        // teal=#94E2D5, blue=#89B4FA, mauve=#CBA6F7, overlay=#6C7086
        Self::from_palette("catppuccin", "Catppuccin Mocha", &[
            (Error, "#F38BA8"), (Warn, "#FAB387"), (Info, "#89B4FA"),
            (Debug, "#6C7086"), (Trace, "#6C7086"),
            (Number, "#CBA6F7"), (String, "#A6E3A1"), (Boolean, "#CBA6F7"),
            (Hash, "#94E2D5"), (Address, "#FAB387"), (Slot, "#A6E3A1"),
            (Epoch, "#89B4FA"), (BlockNumber, "#F9E2AF"), (PeerId, "#94E2D5"),
            (Validator, "#CBA6F7"), (Pubkey, "#94E2D5"), (Duty, "#FAB387"), (Committee, "#89B4FA"),
            (Finality, "#A6E3A1"), (Root, "#94E2D5"), (Attestation, "#CBA6F7"),
            (MevValue, "#F9E2AF"), (Relay, "#6C7086"), (Builder, "#CBA6F7"),
            (Success, "#A6E3A1"), (Failure, "#F38BA8"), (Syncing, "#F9E2AF"),
            (Timestamp, "#6C7086"), (Key, "#94E2D5"), (Value, "#CDD6F4"),
            (Identifier, "#94E2D5"), (Label, "#94E2D5"), (Metric, "#CBA6F7"),
        ])
    }

    /// Synthwave84 retro-futuristic theme.
    pub fn synthwave84() -> Self {
        use SemanticColor::*;
        Self::from_palette("synthwave84", "Retro-futuristic neon aesthetic", &[
            (Error, "#FE4450"), (Warn, "#FEDE5D"), (Info, "#03EDF9"),
            (Debug, "#848BBD"), (Trace, "#495495"),
            (Number, "#F97E72"), (String, "#FF8B39"), (Boolean, "#FF7EDB"),
            (Hash, "#36F9F6"), (Address, "#FF7EDB"), (Slot, "#03EDF9"),
            (Epoch, "#FF7EDB"), (BlockNumber, "#FEDE5D"), (PeerId, "#36F9F6"),
            (Validator, "#FF7EDB"), (Pubkey, "#36F9F6"), (Duty, "#FEDE5D"), (Committee, "#03EDF9"),
            (Finality, "#72F1B8"), (Root, "#36F9F6"), (Attestation, "#FF7EDB"),
            (MevValue, "#FEDE5D"), (Relay, "#848BBD"), (Builder, "#FF7EDB"),
            (Success, "#72F1B8"), (Failure, "#FE4450"), (Syncing, "#03EDF9"),
            (Timestamp, "#848BBD"), (Key, "#FEDE5D"), (Value, "#FFFFFF"),
            (Identifier, "#36F9F6"), (Label, "#03EDF9"), (Metric, "#FF7EDB"),
        ])
    }

    /// Gruvbox dark theme - retro earthy tones.
    pub fn gruvbox() -> Self {
        use SemanticColor::*;
        // Gruvbox: red=#FB4934, orange=#FE8019, yellow=#FABD2F, green=#B8BB26
        // aqua=#8EC07C, blue=#83A598, purple=#D3869B, gray=#928374
        Self::from_palette("gruvbox", "Retro groove with earthy colors", &[
            (Error, "#FB4934"), (Warn, "#FE8019"), (Info, "#83A598"),
            (Debug, "#928374"), (Trace, "#928374"),
            (Number, "#D3869B"), (String, "#B8BB26"), (Boolean, "#D3869B"),
            (Hash, "#8EC07C"), (Address, "#FE8019"), (Slot, "#B8BB26"),
            (Epoch, "#83A598"), (BlockNumber, "#FABD2F"), (PeerId, "#8EC07C"),
            (Validator, "#D3869B"), (Pubkey, "#8EC07C"), (Duty, "#FE8019"), (Committee, "#83A598"),
            (Finality, "#B8BB26"), (Root, "#8EC07C"), (Attestation, "#D3869B"),
            (MevValue, "#FABD2F"), (Relay, "#928374"), (Builder, "#D3869B"),
            (Success, "#B8BB26"), (Failure, "#FB4934"), (Syncing, "#FABD2F"),
            (Timestamp, "#928374"), (Key, "#8EC07C"), (Value, "#EBDBB2"),
            (Identifier, "#8EC07C"), (Label, "#8EC07C"), (Metric, "#D3869B"),
        ])
    }

    /// Monokai classic editor theme.
    pub fn monokai() -> Self {
        use SemanticColor::*;
        // Monokai: red=#F92672, orange=#FD971F, yellow=#E6DB74, green=#A6E22E
        // cyan=#66D9EF, purple=#AE81FF, comment=#75715E
        Self::from_palette("monokai", "Classic editor color scheme", &[
            (Error, "#F92672"), (Warn, "#FD971F"), (Info, "#66D9EF"),
            (Debug, "#75715E"), (Trace, "#75715E"),
            (Number, "#AE81FF"), (String, "#E6DB74"), (Boolean, "#AE81FF"),
            (Hash, "#66D9EF"), (Address, "#F92672"), (Slot, "#A6E22E"),
            (Epoch, "#AE81FF"), (BlockNumber, "#FD971F"), (PeerId, "#66D9EF"),
            (Validator, "#AE81FF"), (Pubkey, "#66D9EF"), (Duty, "#FD971F"), (Committee, "#AE81FF"),
            (Finality, "#A6E22E"), (Root, "#66D9EF"), (Attestation, "#F92672"),
            (MevValue, "#E6DB74"), (Relay, "#75715E"), (Builder, "#F92672"),
            (Success, "#A6E22E"), (Failure, "#F92672"), (Syncing, "#E6DB74"),
            (Timestamp, "#75715E"), (Key, "#66D9EF"), (Value, "#F8F8F2"),
            (Identifier, "#66D9EF"), (Label, "#66D9EF"), (Metric, "#AE81FF"),
        ])
    }

    /// Solarized dark theme - precision colors for machines and people.
    pub fn solarized() -> Self {
        use SemanticColor::*;
        // Solarized: red=#DC322F, orange=#CB4B16, yellow=#B58900, green=#859900
        // cyan=#2AA198, blue=#268BD2, violet=#6C71C4, base01=#586E75
        Self::from_palette("solarized", "Precision colors, dark variant", &[
            (Error, "#DC322F"), (Warn, "#CB4B16"), (Info, "#268BD2"),
            (Debug, "#586E75"), (Trace, "#586E75"),
            (Number, "#6C71C4"), (String, "#859900"), (Boolean, "#6C71C4"),
            (Hash, "#2AA198"), (Address, "#CB4B16"), (Slot, "#859900"),
            (Epoch, "#268BD2"), (BlockNumber, "#B58900"), (PeerId, "#2AA198"),
            (Validator, "#6C71C4"), (Pubkey, "#2AA198"), (Duty, "#CB4B16"), (Committee, "#268BD2"),
            (Finality, "#859900"), (Root, "#2AA198"), (Attestation, "#6C71C4"),
            (MevValue, "#B58900"), (Relay, "#586E75"), (Builder, "#6C71C4"),
            (Success, "#859900"), (Failure, "#DC322F"), (Syncing, "#B58900"),
            (Timestamp, "#586E75"), (Key, "#2AA198"), (Value, "#93A1A1"),
            (Identifier, "#2AA198"), (Label, "#2AA198"), (Metric, "#6C71C4"),
        ])
    }

    /// Matrix green monochrome theme.
    pub fn matrix() -> Self {
        use SemanticColor::*;
        // Matrix: bright=#00FF00, normal=#00DD00, dim=#00AA00, dark=#007700, darker=#005500
        Self::from_palette("matrix", "Green monochrome hacker aesthetic", &[
            (Error, "#00FF00"), (Warn, "#00DD00"), (Info, "#00AA00"),
            (Debug, "#007700"), (Trace, "#005500"),
            (Number, "#00DD00"), (String, "#00AA00"), (Boolean, "#00DD00"),
            (Hash, "#00FF00"), (Address, "#00FF00"), (Slot, "#00DD00"),
            (Epoch, "#00DD00"), (BlockNumber, "#00FF00"), (PeerId, "#00AA00"),
            (Validator, "#00FF00"), (Pubkey, "#00DD00"), (Duty, "#00DD00"), (Committee, "#00AA00"),
            (Finality, "#00FF00"), (Root, "#00DD00"), (Attestation, "#00DD00"),
            (MevValue, "#00FF00"), (Relay, "#007700"), (Builder, "#00DD00"),
            (Success, "#00FF00"), (Failure, "#00FF00"), (Syncing, "#00DD00"),
            (Timestamp, "#007700"), (Key, "#00DD00"), (Value, "#00AA00"),
            (Identifier, "#00FF00"), (Label, "#00DD00"), (Metric, "#00DD00"),
        ])
    }

    /// Phosphor amber monochrome theme - old CRT terminal.
    pub fn phosphor() -> Self {
        use SemanticColor::*;
        // Phosphor: bright=#FFCC00, normal=#FFAA00, dim=#DD8800, dark=#AA6600, darker=#774400
        Self::from_palette("phosphor", "Amber CRT terminal nostalgia", &[
            (Error, "#FFCC00"), (Warn, "#FFAA00"), (Info, "#DD8800"),
            (Debug, "#AA6600"), (Trace, "#774400"),
            (Number, "#FFAA00"), (String, "#DD8800"), (Boolean, "#FFAA00"),
            (Hash, "#FFCC00"), (Address, "#FFCC00"), (Slot, "#FFAA00"),
            (Epoch, "#FFAA00"), (BlockNumber, "#FFCC00"), (PeerId, "#DD8800"),
            (Validator, "#FFCC00"), (Pubkey, "#FFAA00"), (Duty, "#FFAA00"), (Committee, "#DD8800"),
            (Finality, "#FFCC00"), (Root, "#FFAA00"), (Attestation, "#FFAA00"),
            (MevValue, "#FFCC00"), (Relay, "#AA6600"), (Builder, "#FFAA00"),
            (Success, "#FFCC00"), (Failure, "#FFCC00"), (Syncing, "#FFAA00"),
            (Timestamp, "#AA6600"), (Key, "#FFAA00"), (Value, "#DD8800"),
            (Identifier, "#FFCC00"), (Label, "#FFAA00"), (Metric, "#FFAA00"),
        ])
    }

    /// Tokyo Night theme - modern city lights aesthetic.
    pub fn tokyo_night() -> Self {
        use SemanticColor::*;
        // Tokyo Night: red=#F7768E, orange=#FF9E64, yellow=#E0AF68, green=#9ECE6A
        // teal=#73DACA, cyan=#7DCFFF, blue=#7AA2F7, purple=#BB9AF7, comment=#565F89
        Self::from_palette("tokyo-night", "Modern city lights aesthetic", &[
            (Error, "#F7768E"), (Warn, "#FF9E64"), (Info, "#7AA2F7"),
            (Debug, "#565F89"), (Trace, "#565F89"),
            (Number, "#FF9E64"), (String, "#9ECE6A"), (Boolean, "#BB9AF7"),
            (Hash, "#7DCFFF"), (Address, "#F7768E"), (Slot, "#73DACA"),
            (Epoch, "#BB9AF7"), (BlockNumber, "#E0AF68"), (PeerId, "#7DCFFF"),
            (Validator, "#BB9AF7"), (Pubkey, "#7DCFFF"), (Duty, "#FF9E64"), (Committee, "#7AA2F7"),
            (Finality, "#9ECE6A"), (Root, "#7DCFFF"), (Attestation, "#BB9AF7"),
            (MevValue, "#E0AF68"), (Relay, "#565F89"), (Builder, "#F7768E"),
            (Success, "#9ECE6A"), (Failure, "#F7768E"), (Syncing, "#E0AF68"),
            (Timestamp, "#565F89"), (Key, "#73DACA"), (Value, "#A9B1D6"),
            (Identifier, "#7DCFFF"), (Label, "#73DACA"), (Metric, "#BB9AF7"),
        ])
    }

    /// Horizon theme - warm sunset colors.
    pub fn horizon() -> Self {
        use SemanticColor::*;
        // Horizon: red=#E95678, orange=#FAB795, yellow=#FAC29A, green=#29D398
        // cyan=#59E3E3, blue=#26BBD9, purple=#EE64AE, comment=#6C6F93
        Self::from_palette("horizon", "Warm sunset colors", &[
            (Error, "#E95678"), (Warn, "#FAB795"), (Info, "#26BBD9"),
            (Debug, "#6C6F93"), (Trace, "#6C6F93"),
            (Number, "#FAB795"), (String, "#FAC29A"), (Boolean, "#EE64AE"),
            (Hash, "#59E3E3"), (Address, "#E95678"), (Slot, "#29D398"),
            (Epoch, "#EE64AE"), (BlockNumber, "#FAB795"), (PeerId, "#59E3E3"),
            (Validator, "#EE64AE"), (Pubkey, "#59E3E3"), (Duty, "#FAB795"), (Committee, "#26BBD9"),
            (Finality, "#29D398"), (Root, "#59E3E3"), (Attestation, "#EE64AE"),
            (MevValue, "#FAC29A"), (Relay, "#6C6F93"), (Builder, "#E95678"),
            (Success, "#29D398"), (Failure, "#E95678"), (Syncing, "#FAC29A"),
            (Timestamp, "#6C6F93"), (Key, "#59E3E3"), (Value, "#FDF0ED"),
            (Identifier, "#59E3E3"), (Label, "#59E3E3"), (Metric, "#EE64AE"),
        ])
    }

    /// High contrast theme for accessibility.
    pub fn high_contrast() -> Self {
        use SemanticColor::*;
        Self::from_palette("high-contrast", "Maximum readability", &[
            (Error, "#FF0000"), (Warn, "#FFFF00"), (Info, "#00FFFF"),
            (Debug, "#888888"), (Trace, "#666666"),
            (Number, "#FF00FF"), (String, "#00FF00"), (Boolean, "#FF00FF"),
            (Hash, "#00FFFF"), (Address, "#FFFF00"), (Slot, "#00FF00"),
            (Epoch, "#00FFFF"), (BlockNumber, "#FFFF00"), (PeerId, "#00FFFF"),
            (Validator, "#FF00FF"), (Pubkey, "#00FFFF"), (Duty, "#FFFF00"), (Committee, "#00FFFF"),
            (Finality, "#00FF00"), (Root, "#00FFFF"), (Attestation, "#FF00FF"),
            (MevValue, "#FFFF00"), (Relay, "#888888"), (Builder, "#FF00FF"),
            (Success, "#00FF00"), (Failure, "#FF0000"), (Syncing, "#FFFF00"),
            (Timestamp, "#888888"), (Key, "#00FFFF"), (Value, "#FFFFFF"),
            (Identifier, "#00FFFF"), (Label, "#00FFFF"), (Metric, "#FF00FF"),
        ])
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
}
