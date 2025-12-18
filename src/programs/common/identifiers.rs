//! Identifier patterns (hex IDs, UUIDs, device names).

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Hex identifiers (12-char short, 64-char full - common for container/commit IDs).
#[must_use]
pub fn hex_id_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b[a-f0-9]{64}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{40}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{12}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{7,8}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]
}

/// UUID patterns.
#[must_use]
pub fn uuid_rule() -> Rule {
    Rule::new(r"\b[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// Device names (sda, nvme0n1, eth0, etc.).
#[must_use]
pub fn device_name_rule() -> Rule {
    Rule::new(r"\b(sd[a-z]+\d*|nvme\d+n\d+p?\d*|eth\d+|enp\d+s\d+|ens\d+|wlan\d+|wlp\d+s\d+|lo)\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// File permission patterns (rwxr-xr-x, drwxr-xr-x).
#[must_use]
pub fn permission_rule() -> Rule {
    Rule::new(r"[-dlbcps][-rwxsStT]{9}")
        .unwrap()
        .semantic(SemanticColor::Label)
        .build()
}

/// Compiler error location (<file:line:col>).
#[must_use]
pub fn compiler_location_rule() -> Rule {
    Rule::new(r"[^\s:]+:\d+:\d+:")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}
