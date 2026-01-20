//! Macros for reducing boilerplate in Ethereum client definitions.

/// Define a client metadata constant.
///
/// # Example
/// ```ignore
/// define_client!(LIGHTHOUSE, "Lighthouse", "Ethereum consensus client in Rust",
///     Layer::Consensus, "Rust", "https://lighthouse.sigmaprime.io/",
///     ["lighthouse", "lighthouse-bn"], "#9933FF");
/// ```
macro_rules! define_client {
    (
        $const_name:ident,
        $name:literal,
        $desc:literal,
        $layer:expr,
        $lang:literal,
        $website:literal,
        [$($pattern:literal),* $(,)?],
        $color:literal
    ) => {
        pub const $const_name: ClientMeta = ClientMeta {
            name: $name,
            description: $desc,
            layer: $layer,
            language: $lang,
            website: $website,
            detect_patterns: &[$($pattern),*],
            brand_color: $color,
        };
    };
}

/// Define a rule function that returns `Result<Rule, regex::Error>`.
///
/// # Variants
/// - `semantic($color)` - Use a semantic color
/// - `hex($color)` - Use a hex color string
/// - Add `bold` at the end to make the rule bold
///
/// # Examples
/// ```ignore
/// define_rule!(hash_rule, r"0x[a-fA-F0-9]{8,}", hex(HASH_COLOR));
/// define_rule!(error_rule, r"\bERROR\b", semantic(SemanticColor::Error), bold);
/// ```
macro_rules! define_rule {
    ($fn_name:ident, $pattern:literal, semantic($color:expr)) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.semantic($color).build())
        }
    };
    ($fn_name:ident, $pattern:literal, semantic($color:expr), bold) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.semantic($color).bold().build())
        }
    };
    ($fn_name:ident, $pattern:literal, hex($color:expr)) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.hex($color).build())
        }
    };
    ($fn_name:ident, $pattern:literal, hex($color:expr), bold) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.hex($color).bold().build())
        }
    };
    ($fn_name:ident, $pattern:literal, named($color:literal)) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.named($color).build())
        }
    };
    ($fn_name:ident, $pattern:literal, named($color:literal), bold) => {
        pub fn $fn_name() -> Result<Rule, regex::Error> {
            Ok(Rule::new($pattern)?.named($color).bold().build())
        }
    };
}

/// Define a log levels function that returns `Result<Vec<Rule>, regex::Error>`.
///
/// Each entry is `($pattern, $semantic_color)` or `($pattern, $semantic_color, bold)`.
///
/// # Example
/// ```ignore
/// define_log_levels!(rust_log_levels, [
///     (r"\bERROR\b", SemanticColor::Error, bold),
///     (r"\bWARN\b", SemanticColor::Warn, bold),
///     (r"\bINFO\b", SemanticColor::Info),
/// ]);
/// ```
macro_rules! define_log_levels {
    // Entry point - start accumulating rules
    ($fn_name:ident, [$($entries:tt)*]) => {
        define_log_levels!(@impl $fn_name, [], $($entries)*);
    };

    // Base case - no more entries, generate the function
    (@impl $fn_name:ident, [$($rules:expr),*], $(,)?) => {
        pub fn $fn_name() -> Result<Vec<Rule>, regex::Error> {
            Ok(vec![$($rules),*])
        }
    };

    // Match entry with bold
    (@impl $fn_name:ident, [$($rules:expr),*], ($pattern:literal, $color:expr, bold) $(, $($rest:tt)*)?) => {
        define_log_levels!(@impl $fn_name, [$($rules,)* Rule::new($pattern)?.semantic($color).bold().build()], $($($rest)*)?);
    };

    // Match entry without bold
    (@impl $fn_name:ident, [$($rules:expr),*], ($pattern:literal, $color:expr) $(, $($rest:tt)*)?) => {
        define_log_levels!(@impl $fn_name, [$($rules,)* Rule::new($pattern)?.semantic($color).build()], $($($rest)*)?);
    };
}
