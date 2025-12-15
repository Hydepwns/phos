//! Core colorization engine.
//!
//! The [`Colorizer`] is the main entry point for colorizing text. It applies
//! rules to text and produces ANSI-colored output.
//!
//! # Examples
//!
//! ```rust
//! use phos::{Colorizer, Rule, SemanticColor, Theme};
//!
//! // Create rules
//! let rules = vec![
//!     Rule::new(r"\bERROR\b").unwrap()
//!         .semantic(SemanticColor::Error)
//!         .build(),
//! ];
//!
//! // Create colorizer with theme
//! let mut colorizer = Colorizer::new(rules)
//!     .with_theme(Theme::dracula());
//!
//! // Colorize text
//! let colored = colorizer.colorize("ERROR: something failed");
//! println!("{}", colored);
//! ```
//!
//! # Stdin Processing
//!
//! For processing log streams, use the stdio methods:
//!
//! ```rust,no_run
//! use phos::{Colorizer, programs};
//!
//! let registry = programs::default_registry();
//! let program = registry.get("docker").unwrap();
//! let mut colorizer = Colorizer::new(program.rules());
//!
//! // Process stdin to stdout
//! colorizer.process_stdio().unwrap();
//! ```

use std::borrow::Cow;
use std::io::{self, BufRead, Write};
use std::sync::Arc;

use nu_ansi_term::Style;

use crate::rule::{CountMode, Rule};
use crate::theme::Theme;

/// Maximum line length to colorize. Lines longer than this are passed through
/// unchanged to prevent performance issues with pathological regex patterns.
const MAX_LINE_LENGTH: usize = 10_000;

/// The colorizer applies rules to text and outputs colored results.
///
/// The colorizer is the main engine of phos. It takes a set of rules and a theme,
/// then applies them to colorize text line by line.
///
/// # Features
///
/// - **Rule matching**: Applies regex-based rules to colorize patterns
/// - **Theme support**: Resolves semantic colors through the active theme
/// - **Skip rules**: Can filter out lines matching certain patterns
/// - **Replace rules**: Can transform text while colorizing
/// - **Block mode**: Color entire sections between markers
/// - **Statistics**: Optional tracking of match counts and patterns
///
/// # Examples
///
/// ```rust
/// use phos::{Colorizer, Rule, SemanticColor};
///
/// let rules = vec![
///     Rule::new(r"\d+").unwrap()
///         .semantic(SemanticColor::Number)
///         .build(),
/// ];
///
/// let mut colorizer = Colorizer::new(rules);
/// let result = colorizer.colorize("Count: 42");
/// assert!(result.contains("\x1b[")); // Contains ANSI codes
/// ```
#[derive(Debug, Clone)]
pub struct Colorizer {
    /// Rules to apply in order (Arc to avoid cloning compiled regexes)
    rules: Arc<[Rule]>,
    /// Theme for semantic color resolution
    theme: Theme,
    /// Whether currently in block coloring mode
    in_block: bool,
    /// Block coloring style (if in_block is true)
    block_style: Option<Style>,
    /// Whether color output is enabled (false = pass-through mode)
    color_enabled: bool,
}

impl Colorizer {
    /// Create a new colorizer with the given rules.
    pub fn new(rules: impl Into<Arc<[Rule]>>) -> Self {
        Self {
            rules: rules.into(),
            theme: Theme::default(),
            in_block: false,
            block_style: None,
            color_enabled: true,
        }
    }

    /// Set the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Enable or disable color output.
    ///
    /// When disabled, the colorizer passes through text unchanged.
    /// Useful for piping to files or commands that don't support ANSI.
    pub fn with_color_enabled(mut self, enabled: bool) -> Self {
        self.color_enabled = enabled;
        self
    }

    /// Reset colorization state between files or streams.
    ///
    /// This clears block mode state that may persist from a previous stream.
    /// Called automatically after `process_stdio()` and `process_stdio_with_stats()`.
    pub fn reset(&mut self) {
        self.in_block = false;
        self.block_style = None;
    }

    /// Colorize a single line of text.
    /// Returns the colorized line, or the original if skipped by a skip rule.
    pub fn colorize(&mut self, line: &str) -> String {
        self.colorize_with_match_info(line).0
    }

    /// Colorize a single line and return whether it had any matches.
    /// Returns (colorized_string, had_matches).
    pub fn colorize_with_match_info(&mut self, line: &str) -> (String, bool) {
        match self.colorize_opt_with_match_info(line) {
            Some((output, had_match)) => (output, had_match),
            None => (String::new(), true), // Line was skipped
        }
    }

    /// Colorize a single line, returning None if the line should be skipped.
    /// Returns Some((colorized_string, had_matches)) or None if skipped.
    pub fn colorize_opt(&mut self, line: &str) -> Option<String> {
        self.colorize_opt_with_match_info(line).map(|(s, _)| s)
    }

    /// Colorize with skip support and match info.
    /// Returns None if a skip rule matched, otherwise Some((output, had_matches)).
    pub fn colorize_opt_with_match_info(&mut self, line: &str) -> Option<(String, bool)> {
        if line.is_empty() {
            return Some((String::new(), false));
        }

        // Skip colorization for extremely long lines to prevent performance issues
        if line.len() > MAX_LINE_LENGTH {
            return Some((line.to_string(), false));
        }

        // Phase 1: Check skip rules first
        for rule in self.rules.iter() {
            if rule.skip && rule.is_match(line) {
                return None; // Skip this line
            }
        }

        // Phase 2: Apply replacements
        let line: Cow<str> = self.rules.iter().fold(Cow::Borrowed(line), |line, rule| {
            if let Some(ref replacement) = rule.replace {
                if rule.is_match(&line) {
                    Cow::Owned(rule.regex.replace_all(&line, replacement).into_owned())
                } else {
                    line
                }
            } else {
                line
            }
        });

        // Phase 3: Colorization
        // Track which parts of the line have been colored
        let mut colored_ranges: Vec<(usize, usize, Style)> = Vec::new();

        for rule in self.rules.iter() {
            // Skip rules that are only for skip/replace (no colors)
            if rule.skip || rule.replace.is_some() && rule.colors.is_empty() {
                continue;
            }

            // Handle block mode
            if rule.count_mode == CountMode::Block && rule.is_match(&line) {
                self.in_block = true;
                self.block_style = Some(self.rule_to_style(rule));
            } else if rule.count_mode == CountMode::Unblock && rule.is_match(&line) {
                self.in_block = false;
                self.block_style = None;
            }

            // Find matches
            let matches: Vec<_> = rule.find_iter(&line).collect();

            for m in matches {
                let start = m.start();
                let end = m.end();

                // Check if this range overlaps with existing colored ranges
                let overlaps = colored_ranges
                    .iter()
                    .any(|(s, e, _)| start < *e && end > *s);

                if !overlaps {
                    let style = self.rule_to_style(rule);
                    colored_ranges.push((start, end, style));

                    // If CountMode::Once, only color first match
                    if rule.count_mode == CountMode::Once {
                        break;
                    }
                }

                // If CountMode::Stop, don't process more rules
                if rule.count_mode == CountMode::Stop {
                    break;
                }
            }
        }

        let had_matches = !colored_ranges.is_empty();

        // If colors disabled, return plain text but still report matches for stats
        if !self.color_enabled {
            return Some((line.into_owned(), had_matches));
        }

        // Sort ranges by start position
        colored_ranges.sort_by_key(|(start, _, _)| *start);

        // Build the output string
        Some((self.build_colored_output(&line, &colored_ranges), had_matches))
    }

    /// Convert a rule to a Style.
    fn rule_to_style(&self, rule: &Rule) -> Style {
        let base_style = rule
            .colors
            .iter()
            .filter_map(|color| {
                self.theme
                    .resolve_color(color)
                    .to_style()
                    .foreground
            })
            .fold(Style::new(), |style, fg| style.fg(fg));

        if rule.bold {
            base_style.bold()
        } else {
            base_style
        }
    }

    /// Build the final colored output string.
    fn build_colored_output(&self, line: &str, ranges: &[(usize, usize, Style)]) -> String {
        if ranges.is_empty() {
            return self.style_segment(line);
        }

        let (result, last_end) = ranges.iter().fold(
            (String::with_capacity(line.len() * 2), 0usize),
            |(mut result, last_end), (start, end, style)| {
                // Add uncolored text before this range
                if *start > last_end {
                    result.push_str(&self.style_segment(&line[last_end..*start]));
                }
                // Add colored text
                result.push_str(&style.paint(&line[*start..*end]).to_string());
                (result, *end)
            },
        );

        // Add remaining uncolored text
        if last_end < line.len() {
            format!("{result}{}", self.style_segment(&line[last_end..]))
        } else {
            result
        }
    }

    /// Style a text segment, applying block style if in block mode.
    fn style_segment(&self, text: &str) -> String {
        self.block_style
            .as_ref()
            .filter(|_| self.in_block)
            .map(|style| style.paint(text).to_string())
            .unwrap_or_else(|| text.to_string())
    }

    /// Process stdin and write colorized output to stdout.
    /// Lines matching skip rules are not output.
    pub fn process_stdio(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            if let Some(colored) = self.colorize_opt(&line) {
                writeln!(stdout, "{colored}")?;
            }
            // Skip rule matched - don't output anything
        }

        // Reset block state for next stream
        self.reset();

        Ok(())
    }

    /// Process stdin with statistics collection.
    /// Lines matching skip rules are not output but are counted in stats.
    pub fn process_stdio_with_stats(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
    ) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            match self.colorize_opt_with_match_info(&line) {
                Some((colored, had_match)) => {
                    stats.process_line(&line, had_match);
                    writeln!(stdout, "{colored}")?;
                }
                None => {
                    // Skip rule matched - count but don't output
                    stats.process_line(&line, true);
                    stats.record_skipped();
                }
            }
        }

        // Reset block state for next stream
        self.reset();

        Ok(())
    }

    /// Process stdin with both statistics collection and alerting.
    pub fn process_stdio_with_alerts(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
        alert_manager: &mut crate::alert::AlertManager,
    ) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            match self.colorize_opt_with_match_info(&line) {
                Some((colored, had_match)) => {
                    stats.process_line(&line, had_match);

                    // Check alert conditions
                    alert_manager.check_line(
                        &line,
                        stats.error_count(),
                        stats.peer_count(),
                        stats.slot(),
                    );

                    writeln!(stdout, "{colored}")?;
                }
                None => {
                    // Skip rule matched - count but don't output
                    stats.process_line(&line, true);
                    stats.record_skipped();
                }
            }
        }

        // Reset block state for next stream
        self.reset();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::SemanticColor;

    #[test]
    fn test_colorizer_creation() {
        let rules = vec![
            Rule::new(r"\berror\b")
                .unwrap()
                .semantic(SemanticColor::Error)
                .build(),
        ];
        let colorizer = Colorizer::new(rules);
        assert_eq!(colorizer.rules.len(), 1);
    }

    #[test]
    fn test_colorize_line() {
        let rules = vec![
            Rule::new(r"\berror\b")
                .unwrap()
                .semantic(SemanticColor::Error)
                .build(),
        ];
        let mut colorizer = Colorizer::new(rules);
        let result = colorizer.colorize("an error occurred");

        // Should contain ANSI codes
        assert!(result.contains("\x1b["));
        assert!(result.contains("error"));
    }

    #[test]
    fn test_empty_line() {
        let mut colorizer = Colorizer::new(vec![]);
        let result = colorizer.colorize("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_matches() {
        let rules = vec![
            Rule::new(r"\berror\b")
                .unwrap()
                .semantic(SemanticColor::Error)
                .build(),
        ];
        let mut colorizer = Colorizer::new(rules);
        let result = colorizer.colorize("all good here");

        // Should be unchanged (no ANSI codes)
        assert_eq!(result, "all good here");
    }

    #[test]
    fn test_skip_rule() {
        let rules = vec![
            Rule::new(r"DEBUG")
                .unwrap()
                .skip()
                .build(),
            Rule::new(r"error")
                .unwrap()
                .semantic(SemanticColor::Error)
                .build(),
        ];
        let mut colorizer = Colorizer::new(rules);

        // DEBUG lines should be skipped
        assert!(colorizer.colorize_opt("DEBUG: some message").is_none());

        // Other lines should be colorized
        let result = colorizer.colorize_opt("ERROR: something went wrong");
        assert!(result.is_some());
    }

    #[test]
    fn test_replace_rule() {
        let rules = vec![
            Rule::new(r"(\d{2}):(\d{2}):(\d{2})")
                .unwrap()
                .replace("${1}h${2}m${3}s")
                .semantic(SemanticColor::Timestamp)
                .build(),
        ];
        let mut colorizer = Colorizer::new(rules).with_color_enabled(false);

        let result = colorizer.colorize("Time: 12:34:56");
        assert!(result.contains("12h34m56s"));
    }

    #[test]
    fn test_skip_and_replace_together() {
        let rules = vec![
            // Skip DEBUG lines
            Rule::new(r"^\[DEBUG\]")
                .unwrap()
                .skip()
                .build(),
            // Replace timestamp format
            Rule::new(r"(\d{4})-(\d{2})-(\d{2})")
                .unwrap()
                .replace("${2}/${3}/${1}")
                .build(),
        ];
        let mut colorizer = Colorizer::new(rules).with_color_enabled(false);

        // DEBUG should be skipped
        assert!(colorizer.colorize_opt("[DEBUG] 2024-01-15 test").is_none());

        // INFO should have replacement
        let result = colorizer.colorize_opt("[INFO] 2024-01-15 test").unwrap();
        assert!(result.contains("01/15/2024"));
    }
}
