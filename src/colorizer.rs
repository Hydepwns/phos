//! Core colorization engine.

use std::io::{self, BufRead, Write};

use nu_ansi_term::Style;

use crate::clients::Client;
use crate::rule::{CountMode, Rule};
use crate::theme::Theme;

/// The colorizer applies rules to text and outputs colored results.
#[derive(Debug, Clone)]
pub struct Colorizer {
    /// Rules to apply in order
    rules: Vec<Rule>,
    /// Theme for semantic color resolution
    theme: Theme,
    /// Whether currently in block coloring mode
    in_block: bool,
    /// Block coloring style (if in_block is true)
    block_style: Option<Style>,
}

impl Colorizer {
    /// Create a new colorizer with the given rules.
    pub fn new(rules: Vec<Rule>) -> Self {
        Self {
            rules,
            theme: Theme::default(),
            in_block: false,
            block_style: None,
        }
    }

    /// Create a colorizer for a specific Ethereum client.
    pub fn for_client(client: Client) -> Self {
        Self::new(client.rules())
    }

    /// Set the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Colorize a single line of text.
    pub fn colorize(&mut self, line: &str) -> String {
        self.colorize_with_match_info(line).0
    }

    /// Colorize a single line and return whether it had any matches.
    /// Returns (colorized_string, had_matches).
    pub fn colorize_with_match_info(&mut self, line: &str) -> (String, bool) {
        if line.is_empty() {
            return (String::new(), false);
        }

        // Track which parts of the line have been colored
        let mut colored_ranges: Vec<(usize, usize, Style)> = Vec::new();

        for rule in &self.rules {
            // Handle block mode
            if rule.count_mode == CountMode::Block && rule.is_match(line) {
                self.in_block = true;
                self.block_style = Some(self.rule_to_style(rule));
            } else if rule.count_mode == CountMode::Unblock && rule.is_match(line) {
                self.in_block = false;
                self.block_style = None;
            }

            // Find matches
            let matches: Vec<_> = rule.find_iter(line).collect();

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

        // Sort ranges by start position
        colored_ranges.sort_by_key(|(start, _, _)| *start);

        // Build the output string
        (self.build_colored_output(line, &colored_ranges), had_matches)
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
    pub fn process_stdio(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            let colored = self.colorize(&line);
            writeln!(stdout, "{colored}")?;
        }

        Ok(())
    }

    /// Process stdin with statistics collection.
    /// Returns the stats collector with accumulated statistics.
    pub fn process_stdio_with_stats(
        &mut self,
        stats: &mut crate::stats::StatsCollector,
    ) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            let (colored, had_match) = self.colorize_with_match_info(&line);
            stats.process_line(&line, had_match);
            writeln!(stdout, "{colored}")?;
        }

        Ok(())
    }
}

/// Try to auto-detect client from command and arguments.
pub fn detect_client(cmd: &str, args: &[String]) -> Option<Client> {
    let full_cmd = format!("{} {}", cmd, args.join(" ")).to_lowercase();

    // Check for client names in command
    Client::all()
        .into_iter()
        .find(|client| {
            let name = format!("{client:?}").to_lowercase();
            full_cmd.contains(&name)
        })
        .or_else(|| {
            // Check for common container/service names
            if full_cmd.contains("beacon") || full_cmd.contains("consensus") {
                Some(Client::Lighthouse)
            } else if full_cmd.contains("execution") || full_cmd.contains("el-") {
                Some(Client::Geth)
            } else {
                None
            }
        })
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
    fn test_client_detection() {
        assert_eq!(
            detect_client("docker", &["logs".to_string(), "lighthouse".to_string()]),
            Some(Client::Lighthouse)
        );
        assert_eq!(
            detect_client("journalctl", &["-u".to_string(), "geth".to_string()]),
            Some(Client::Geth)
        );
    }
}
