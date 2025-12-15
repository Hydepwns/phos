//! Version control system programs.
//!
//! Provides Program implementations for git, diff, and wdiff.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// GIT
// =============================================================================

fn git_rules() -> Vec<Rule> {
    let mut rules = vec![
        // Diff: additions
        Rule::new(r"^\+[^+].*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\+\+\+.*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        // Diff: deletions
        Rule::new(r"^-[^-].*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^---.*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Diff: hunk headers
        Rule::new(r"^@@.*@@")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Diff: file headers
        Rule::new(r"^diff --git.*$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^index [a-f0-9]+\.\.[a-f0-9]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ];

    // Commit hashes (40-char and 7-8 char)
    rules.extend(
        common::hex_id_rules()
            .into_iter()
            .filter(|_| true) // Take 40-char and 7-8 char
            .take(2)
            .skip(1), // Skip 64-char, take 40-char
    );
    rules.push(
        Rule::new(r"\b[a-f0-9]{7,8}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Branch names and status
    rules.extend([
        Rule::new(r"\b(HEAD|master|main|develop|feature|bugfix|hotfix|release)/?\S*")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"^\s*M\s")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^\s*A\s")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\s*D\s")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^\s*R\s")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*\?\?\s")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Status keywords
    rules.extend([
        Rule::new(r"\b(modified|new file|deleted|renamed|copied)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(Untracked files|Changes not staged|Changes to be committed)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        Rule::new(r"\b(fetch|push|pull|merge|rebase|cherry-pick)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(origin|upstream)/\S+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Author/date and files
    rules.extend([
        Rule::new(r"Author:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"Date:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\s[\w\-\./]+\.(rs|js|ts|py|go|java|c|cpp|h|hpp|md|json|yaml|yml|toml)\b")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        Rule::new(r"\b\d+\s+(insertions?|deletions?|files?\s+changed)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    rules
}

pub fn git_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.git",
            "git",
            "Git commands and output",
            Category::Dev,
            git_rules(),
        )
        .with_detect_patterns(vec!["git"]),
    )
}

// =============================================================================
// DIFF
// =============================================================================

fn diff_rules() -> Vec<Rule> {
    // Use the common diff rules as base
    let mut rules = common::diff_rules();

    // Context lines (lines starting with space in unified diff)
    rules.push(
        Rule::new(r"^ .*$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Binary file notices
    rules.push(
        Rule::new(r"^Binary files .* differ$")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );

    // Only in directory notices
    rules.push(
        Rule::new(r"^Only in .*$")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    );

    // Common subdirectories
    rules.push(
        Rule::new(r"^Common subdirectories:.*$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Line change indicators (old format)
    rules.extend([
        Rule::new(r"^< .*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^> .*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^---$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Change range indicators (old format)
    rules.push(
        Rule::new(r"^\d+(,\d+)?[acd]\d+(,\d+)?$")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn diff_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.diff",
            "diff",
            "File comparison output",
            Category::Dev,
            diff_rules(),
        )
        // Use specific patterns to avoid matching "git diff"
        .with_detect_patterns(vec!["colordiff", "sdiff", "/diff ", "diff -"]),
    )
}

// =============================================================================
// WDIFF (Word Diff)
// =============================================================================

fn wdiff_rules() -> Vec<Rule> {
    vec![
        // Deleted words (in brackets by default)
        Rule::new(r"\[-[^\]]+\-\]")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Added words (in braces by default)
        Rule::new(r"\{\+[^}]+\+\}")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Statistics line
        Rule::new(r"^\d+ words? (deleted|inserted|changed)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"^\d+% common")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // File header
        Rule::new(r"^={5,}.*={5,}$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        common::number_rule(),
    ]
}

pub fn wdiff_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.wdiff",
            "wdiff",
            "Word diff output",
            Category::Dev,
            wdiff_rules(),
        )
        .with_detect_patterns(vec!["wdiff", "dwdiff"]),
    )
}
