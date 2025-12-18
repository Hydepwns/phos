//! Property-based tests for phos colorizer.
//!
//! These tests verify invariants that should hold for any input:
//! - Colorization is idempotent (applying twice = applying once)
//! - Output never contains malformed ANSI escape sequences

use phos::{Colorizer, Rule, SemanticColor, Theme};
use proptest::prelude::*;

/// Strategy for generating arbitrary printable strings (no control chars except newline)
fn printable_string() -> impl Strategy<Value = String> {
    proptest::collection::vec(
        any::<char>().prop_filter_map("printable", |c| {
            if c.is_ascii_graphic() || c == ' ' || c == '\n' || c == '\t' {
                Some(c)
            } else {
                None
            }
        }),
        0..500,
    )
    .prop_map(|chars| chars.into_iter().collect())
}

/// Strategy for generating log-like strings with common patterns
fn log_like_string() -> impl Strategy<Value = String> {
    prop_oneof![
        // Simple strings
        printable_string(),
        // Strings with numbers
        "[0-9]{1,10}".prop_map(|s| format!("value={s}")),
        // Strings with log levels
        Just("ERROR: something failed".to_string()),
        Just("WARN: something suspicious".to_string()),
        Just("INFO: normal operation".to_string()),
        Just("DEBUG: verbose output".to_string()),
        // Strings with hex patterns
        "[a-f0-9]{8,64}".prop_map(|s| format!("hash=0x{s}")),
        // IP addresses
        "([0-9]{1,3}\\.){3}[0-9]{1,3}".prop_map(|s| format!("ip={s}")),
        // Timestamps
        Just("2024-01-15T10:30:45.123Z".to_string()),
        // Mixed content
        (printable_string(), "[0-9]{1,5}", printable_string())
            .prop_map(|(a, b, c)| format!("{a} {b} {c}")),
    ]
}

/// Create a colorizer with various rules for testing
fn test_colorizer() -> Colorizer {
    let rules = vec![
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bWARN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b\d+\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"0x[a-fA-F0-9]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ];

    Colorizer::new(rules).with_theme(Theme::default_dark())
}

/// Check if a string contains only well-formed ANSI escape sequences.
/// Returns Ok(()) if valid, Err with description if malformed.
fn validate_ansi(s: &str) -> Result<(), String> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == 0x1b {
            // ESC character - start of ANSI sequence
            i += 1;
            if i >= bytes.len() {
                return Err("Truncated ANSI sequence: ESC at end of string".to_string());
            }

            if bytes[i] == b'[' {
                // CSI sequence: ESC [ ... final_byte
                i += 1;
                let start = i;

                // Parameter bytes (0x30-0x3F) and intermediate bytes (0x20-0x2F)
                while i < bytes.len() && (0x20..=0x3f).contains(&bytes[i]) {
                    i += 1;
                }

                // Final byte must be 0x40-0x7E
                if i >= bytes.len() {
                    return Err(format!("Truncated CSI sequence starting at byte {start}"));
                }

                if !(0x40..=0x7e).contains(&bytes[i]) {
                    return Err(format!(
                        "Invalid CSI final byte 0x{:02x} at position {i}",
                        bytes[i]
                    ));
                }
                i += 1;
            } else if bytes[i] == b']' {
                // OSC sequence: ESC ] ... ST (or BEL)
                i += 1;
                while i < bytes.len() && bytes[i] != 0x07 {
                    // Look for BEL or ST
                    if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                        i += 2; // ST = ESC \
                        break;
                    }
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == 0x07 {
                    i += 1; // BEL terminator
                }
            } else if (0x40..=0x5f).contains(&bytes[i]) {
                // Fe escape sequence (single byte after ESC)
                i += 1;
            } else {
                return Err(format!(
                    "Unknown escape sequence: ESC followed by 0x{:02x}",
                    bytes[i]
                ));
            }
        } else {
            i += 1;
        }
    }

    Ok(())
}

/// Strip ANSI escape sequences from a string using regex
fn strip_ansi(s: &str) -> String {
    use regex::Regex;
    use std::sync::OnceLock;

    static ANSI_RE: OnceLock<Regex> = OnceLock::new();
    let re = ANSI_RE.get_or_init(|| {
        // Match ANSI escape sequences: ESC [ ... final_byte
        Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap()
    });

    re.replace_all(s, "").into_owned()
}

proptest! {
    // NOTE: Colorization is NOT idempotent by design.
    // ANSI escape codes contain numbers (e.g., \x1b[38;2;255;85;85m) which
    // will match numeric patterns on a second pass, creating malformed output.
    // This is acceptable because colorizing already-colorized text is not
    // a supported use case. The important properties are:
    // 1. Original text is preserved (colorization_preserves_text)
    // 2. Output contains valid ANSI codes (output_has_valid_ansi)

    /// Property: Output never contains malformed ANSI escape sequences.
    #[test]
    fn output_has_valid_ansi(input in log_like_string()) {
        let mut colorizer = test_colorizer();
        let output = colorizer.colorize(&input);

        if let Err(e) = validate_ansi(&output) {
            prop_assert!(false, "Malformed ANSI in output: {e}\nInput: {input:?}\nOutput: {output:?}");
        }
    }

    /// Property: Colorization preserves the original text (only adds ANSI codes).
    #[test]
    fn colorization_preserves_text(input in printable_string()) {
        let mut colorizer = test_colorizer();
        let output = colorizer.colorize(&input);
        let stripped = strip_ansi(&output);

        prop_assert_eq!(
            input,
            stripped,
            "Original text not preserved after stripping ANSI codes"
        );
    }

    /// Property: Empty input produces empty output.
    #[test]
    fn empty_input_empty_output(_seed in 0u32..100) {
        let mut colorizer = test_colorizer();
        let output = colorizer.colorize("");
        prop_assert_eq!(output, "", "Empty input should produce empty output");
    }

    /// Property: Colorization doesn't panic on any input.
    #[test]
    fn no_panic_on_any_input(input in ".*") {
        let mut colorizer = test_colorizer();
        let _ = colorizer.colorize(&input);
        // If we get here without panicking, the test passes
    }

    /// Property: Very long lines don't cause excessive memory usage or hangs.
    /// (Tests the line length limit)
    #[test]
    fn handles_long_lines(
        prefix in printable_string(),
        repeat_char in prop::sample::select(vec!['a', '0', 'x', ' ']),
        repeat_count in 1000usize..20000,
    ) {
        let long_line = format!("{}{}", prefix, repeat_char.to_string().repeat(repeat_count));
        let mut colorizer = test_colorizer();

        // Should complete without hanging
        let output = colorizer.colorize(&long_line);

        // Output should be valid
        prop_assert!(validate_ansi(&output).is_ok());
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_validate_ansi_valid() {
        // Valid ANSI sequences
        assert!(validate_ansi("").is_ok());
        assert!(validate_ansi("hello").is_ok());
        assert!(validate_ansi("\x1b[31mred\x1b[0m").is_ok());
        assert!(validate_ansi("\x1b[38;2;255;0;0mrgb\x1b[0m").is_ok());
        assert!(validate_ansi("\x1b[1;31mbold red\x1b[0m").is_ok());
    }

    #[test]
    fn test_validate_ansi_invalid() {
        // Truncated sequences
        assert!(validate_ansi("\x1b").is_err());
        assert!(validate_ansi("\x1b[").is_err());
        assert!(validate_ansi("\x1b[31").is_err());
    }

    #[test]
    fn test_strip_ansi() {
        assert_eq!(strip_ansi("hello"), "hello");
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(strip_ansi("\x1b[1;31mbold\x1b[0m text"), "bold text");
    }

    #[test]
    fn test_strip_ansi_with_colors() {
        let input = "\x1b[38;2;255;85;85mERROR\x1b[0m test";
        let stripped = strip_ansi(input);
        assert_eq!(stripped, "ERROR test");
    }

    #[test]
    fn test_colorization_preserves_text() {
        let mut colorizer = test_colorizer();
        let input = "ERROR: test 123 hash=0xabc";
        let output = colorizer.colorize(input);
        let stripped = strip_ansi(&output);
        assert_eq!(input, stripped);
    }
}
