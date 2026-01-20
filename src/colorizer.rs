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
use smallvec::SmallVec;

use crate::rule::{CountMode, Rule};
use crate::theme::Theme;

/// Type alias for match ranges - stack-allocated for typical cases (0-8 matches)
type MatchRanges = SmallVec<[(usize, usize, usize); 8]>;

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
    /// Pre-computed styles for each rule (parallel to rules)
    rule_styles: Arc<[Style]>,
    /// Pre-computed indices of rules that can produce colored output
    colorizable_indices: Arc<[usize]>,
    /// Theme for semantic color resolution
    theme: Theme,
    /// Whether currently in block coloring mode
    in_block: bool,
    /// Block coloring style (if `in_block` is true)
    block_style: Option<Style>,
    /// Whether color output is enabled (false = pass-through mode)
    color_enabled: bool,
}

impl Colorizer {
    /// Create a new colorizer with the given rules.
    pub fn new(rules: impl Into<Arc<[Rule]>>) -> Self {
        let rules: Arc<[Rule]> = rules.into();
        let theme = Theme::default();
        let rule_styles = Self::compute_styles(&rules, &theme);
        let colorizable_indices = Self::compute_colorizable_indices(&rules);

        Self {
            rules,
            rule_styles,
            colorizable_indices,
            theme,
            in_block: false,
            block_style: None,
            color_enabled: true,
        }
    }

    /// Set the theme (recomputes styles).
    #[must_use]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.rule_styles = Self::compute_styles(&self.rules, &theme);
        self.theme = theme;
        self
    }

    /// Compute styles for all rules with a given theme.
    fn compute_styles(rules: &[Rule], theme: &Theme) -> Arc<[Style]> {
        rules
            .iter()
            .map(|rule| {
                let base = rule
                    .colors
                    .iter()
                    .filter_map(|color| theme.resolve_color(color).to_style().foreground)
                    .fold(Style::new(), |style, fg| style.fg(fg));

                if rule.bold {
                    base.bold()
                } else {
                    base
                }
            })
            .collect()
    }

    /// Pre-compute indices of rules that can produce colored output.
    fn compute_colorizable_indices(rules: &[Rule]) -> Arc<[usize]> {
        rules
            .iter()
            .enumerate()
            .filter(|(_, rule)| !(rule.skip || rule.replace.is_some() && rule.colors.is_empty()))
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Enable or disable color output.
    ///
    /// When disabled, the colorizer passes through text unchanged.
    /// Useful for piping to files or commands that don't support ANSI.
    #[must_use]
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
    #[inline]
    pub fn colorize(&mut self, line: &str) -> String {
        self.colorize_with_match_info(line).0
    }

    /// Colorize a single line and return whether it had any matches.
    /// Returns (`colorized_string`, `had_matches`).
    #[inline]
    pub fn colorize_with_match_info(&mut self, line: &str) -> (String, bool) {
        match self.colorize_opt_with_match_info(line) {
            Some((output, had_match)) => (output, had_match),
            None => (String::new(), true), // Line was skipped
        }
    }

    /// Colorize a single line, returning None if the line should be skipped.
    /// Returns `Some((colorized_string`, `had_matches`)) or None if skipped.
    #[inline]
    pub fn colorize_opt(&mut self, line: &str) -> Option<String> {
        self.colorize_opt_with_match_info(line).map(|(s, _)| s)
    }

    /// Colorize with skip support and match info.
    /// Returns None if a skip rule matched, otherwise Some((output, `had_matches`)).
    pub fn colorize_opt_with_match_info(&mut self, line: &str) -> Option<(String, bool)> {
        // Handle edge cases
        match line.len() {
            0 => return Some((String::new(), false)),
            n if n > MAX_LINE_LENGTH => return Some((line.to_string(), false)),
            _ => {}
        }

        // Phase 1: Check skip rules
        let should_skip = self
            .rules
            .iter()
            .any(|rule| rule.skip && rule.is_match(line));
        if should_skip {
            return None;
        }

        // Phase 2: Apply replacements functionally (avoid clone when no replacement)
        let line: Cow<str> =
            self.rules
                .iter()
                .fold(Cow::Borrowed(line), |acc, rule| match &rule.replace {
                    Some(replacement) if rule.is_match(&acc) => {
                        Cow::Owned(rule.regex.replace_all(&acc, replacement).into_owned())
                    }
                    _ => acc,
                });

        // Phase 3: Update block mode state (side effect isolated here)
        self.update_block_state(&line);

        // Phase 4: Collect colored ranges functionally
        let colored_ranges = self.collect_colored_ranges(&line);
        let had_matches = !colored_ranges.is_empty();

        // Phase 5: Build output
        if !self.color_enabled {
            return Some((line.into_owned(), had_matches));
        }

        // Note: colored_ranges is already sorted by collect_colored_ranges (binary search insert)
        Some((
            self.build_colored_output(&line, &colored_ranges),
            had_matches,
        ))
    }

    /// Update block mode state based on rules (isolated side effect).
    fn update_block_state(&mut self, line: &str) {
        // Find first matching block/unblock rule (early exit for common case)
        let block_match = self
            .rules
            .iter()
            .enumerate()
            .filter(|(_, rule)| matches!(rule.count_mode, CountMode::Block | CountMode::Unblock))
            .find(|(_, rule)| rule.is_match(line));

        // Apply update if found
        if let Some((idx, rule)) = block_match {
            match rule.count_mode {
                CountMode::Block => {
                    self.in_block = true;
                    // Use pre-computed style
                    self.block_style = Some(self.rule_styles[idx]);
                }
                CountMode::Unblock => {
                    self.in_block = false;
                    self.block_style = None;
                }
                _ => {}
            }
        }
    }

    /// Collect non-overlapping colored ranges from all rules.
    /// Returns (start, end, `style_index`) tuples using stack allocation for typical cases.
    fn collect_colored_ranges(&self, line: &str) -> MatchRanges {
        let mut ranges: MatchRanges = SmallVec::new();

        // Process only pre-computed colorizable rules (skip filtering per-line)
        for &idx in self.colorizable_indices.iter() {
            let rule = &self.rules[idx];
            let limit = match rule.count_mode {
                CountMode::Once => 1,
                _ => usize::MAX,
            };

            // Find matches and add non-overlapping ones
            for m in rule.find_iter(line).take(limit) {
                let (start, end) = (m.start(), m.end());

                // Check overlap with existing ranges using binary search on sorted ranges
                // Find insertion point for this start position
                let insert_pos = ranges
                    .binary_search_by_key(&start, |(s, _, _)| *s)
                    .unwrap_or_else(|pos| pos);

                // Check for overlap with adjacent ranges only (O(1) instead of O(n))
                let overlaps_prev = insert_pos > 0
                    && ranges
                        .get(insert_pos - 1)
                        .is_some_and(|(_, prev_end, _)| *prev_end > start);
                let overlaps_next = ranges
                    .get(insert_pos)
                    .is_some_and(|(next_start, _, _)| *next_start < end);

                if !overlaps_prev && !overlaps_next {
                    ranges.insert(insert_pos, (start, end, idx));
                }
            }
        }

        ranges
    }

    /// Build the final colored output string.
    fn build_colored_output(&self, line: &str, ranges: &[(usize, usize, usize)]) -> String {
        use std::fmt::Write;

        if ranges.is_empty() {
            return self.format_segment(line).into_owned();
        }

        // Pre-allocate result buffer (line length + estimated ANSI overhead)
        // Use checked_mul to prevent overflow on pathological inputs
        let ansi_overhead = ranges.len().checked_mul(20).unwrap_or(0);
        let capacity = line.len().saturating_add(ansi_overhead);
        let mut result = String::with_capacity(capacity);

        // Process each range with fold to track position
        let last_end = ranges
            .iter()
            .fold(0usize, |last_end, &(start, end, style_idx)| {
                // Write uncolored gap
                self.write_segment(&mut result, &line[last_end..start]);
                // Write colored section (look up style by index)
                let _ = write!(
                    result,
                    "{}",
                    self.rule_styles[style_idx].paint(&line[start..end])
                );
                end
            });

        // Write trailing uncolored text
        self.write_segment(&mut result, &line[last_end..]);

        result
    }

    /// Write a segment to a buffer, applying block style if in block mode.
    #[inline]
    fn write_segment(&self, buf: &mut String, text: &str) {
        use std::fmt::Write;
        match (&self.block_style, self.in_block) {
            (Some(style), true) => {
                let _ = write!(buf, "{}", style.paint(text));
            }
            _ => buf.push_str(text),
        }
    }

    /// Format a text segment, applying block style if in block mode.
    /// Returns Cow to avoid allocation when no styling is needed.
    #[inline]
    fn format_segment<'a>(&self, text: &'a str) -> Cow<'a, str> {
        match (&self.block_style, self.in_block) {
            (Some(style), true) => Cow::Owned(style.paint(text).to_string()),
            _ => Cow::Borrowed(text),
        }
    }

    /// Process stdin and write colorized output to stdout.
    /// Lines matching skip rules are not output.
    pub fn process_stdio(&mut self) -> io::Result<()> {
        self.process_stdio_inner(None, None, 0)
    }

    /// Process stdin with statistics collection.
    /// Lines matching skip rules are not output but are counted in stats.
    pub fn process_stdio_with_stats(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
    ) -> io::Result<()> {
        self.process_stdio_inner(Some(stats), None, 0)
    }

    /// Process stdin with both statistics collection and alerting.
    pub fn process_stdio_with_alerts(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
        alert_manager: &mut crate::alert::AlertManager,
    ) -> io::Result<()> {
        self.process_stdio_inner(Some(stats), Some(alert_manager), 0)
    }

    /// Process stdin with statistics and periodic interval output.
    ///
    /// Outputs compact stats to stderr every `interval_secs` seconds.
    pub fn process_stdio_with_stats_interval(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
        interval_secs: u64,
    ) -> io::Result<()> {
        self.process_stdio_inner(Some(stats), None, interval_secs)
    }

    /// Process stdin with statistics, alerting, and periodic interval output.
    pub fn process_stdio_with_alerts_interval(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
        alert_manager: &mut crate::alert::AlertManager,
        interval_secs: u64,
    ) -> io::Result<()> {
        self.process_stdio_inner(Some(stats), Some(alert_manager), interval_secs)
    }

    /// Core stdin processing implementation.
    ///
    /// Handles colorization with optional stats, alerts, and periodic output.
    fn process_stdio_inner(
        &mut self,
        mut stats: Option<&mut crate::stats::StatsCollector>,
        mut alert_manager: Option<&mut crate::alert::AlertManager>,
        interval_secs: u64,
    ) -> io::Result<()> {
        use std::time::Instant;

        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        let use_interval = interval_secs > 0 && stats.is_some();
        let interval = std::time::Duration::from_secs(interval_secs);
        let mut last_output = Instant::now();

        for line in stdin.lock().lines() {
            let line = line?;

            // Process with or without match info based on whether stats are enabled
            let (output, had_match) = stats
                .as_ref()
                .map(|_| {
                    self.colorize_opt_with_match_info(&line)
                        .map(|(colored, matched)| (Some(colored), matched))
                        .unwrap_or((None, true)) // Skip rule matched
                })
                .unwrap_or_else(|| (self.colorize_opt(&line), false));

            // Record stats if enabled
            if let Some(ref mut s) = stats {
                s.process_line(&line, had_match);
                if output.is_none() {
                    s.record_skipped();
                }
            }

            // Check alerts if enabled
            if let (Some(ref mut alerts), Some(ref s)) = (&mut alert_manager, &stats) {
                alerts.check_line(&line, s.error_count(), s.peer_count(), s.slot());
            }

            // Write colorized output
            if let Some(colored) = output {
                writeln!(stdout, "{colored}")?;
            }

            // Periodic stats output
            if use_interval && last_output.elapsed() >= interval {
                if let Some(ref s) = stats {
                    writeln!(io::stderr(), "{}", s.to_compact())?;
                }
                last_output = Instant::now();
            }
        }

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
        let rules = vec![Rule::new(r"\berror\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build()];
        let colorizer = Colorizer::new(rules);
        assert_eq!(colorizer.rules.len(), 1);
    }

    #[test]
    fn test_colorize_line() {
        let rules = vec![Rule::new(r"\berror\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build()];
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
        let rules = vec![Rule::new(r"\berror\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build()];
        let mut colorizer = Colorizer::new(rules);
        let result = colorizer.colorize("all good here");

        // Should be unchanged (no ANSI codes)
        assert_eq!(result, "all good here");
    }

    #[test]
    fn test_skip_rule() {
        let rules = vec![
            Rule::new(r"DEBUG").unwrap().skip().build(),
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
        let rules = vec![Rule::new(r"(\d{2}):(\d{2}):(\d{2})")
            .unwrap()
            .replace("${1}h${2}m${3}s")
            .semantic(SemanticColor::Timestamp)
            .build()];
        let mut colorizer = Colorizer::new(rules).with_color_enabled(false);

        let result = colorizer.colorize("Time: 12:34:56");
        assert!(result.contains("12h34m56s"));
    }

    #[test]
    fn test_skip_and_replace_together() {
        let rules = vec![
            // Skip DEBUG lines
            Rule::new(r"^\[DEBUG\]").unwrap().skip().build(),
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
