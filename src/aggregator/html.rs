//! ANSI to HTML conversion for web display.

/// Convert ANSI escape sequences to HTML spans.
///
/// This uses the `ansi-to-html` crate to convert terminal colors
/// to inline CSS styles that can be rendered in a browser.
pub fn ansi_to_html(input: &str) -> String {
    ansi_to_html::convert(input).unwrap_or_else(|_| html_escape(input))
}

/// Escape HTML special characters.
fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_ansi_to_html_plain() {
        let result = ansi_to_html("plain text");
        assert!(result.contains("plain text"));
    }
}
