//! Filesystem-related programs.
//!
//! Provides Program implementations for ls, df, du, stat, and mount.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// LS (Directory Listing)
// =============================================================================

fn ls_rules() -> Vec<Rule> {
    let mut rules = vec![
        common::permission_rule(),
        Rule::new(r"^d[-rwxsStT]{9}")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
        Rule::new(r"^l[-rwxsStT]{9}")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^-[-rw]{2}x")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ];

    rules.extend([
        Rule::new(r"\b(root|wheel|admin)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b\d+\s+(root|wheel|\w+)\s+(root|wheel|\w+)\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    rules.push(common::size_rule());
    rules.push(
        Rule::new(r"\b\d+[KMGT]?\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    rules.extend([
        Rule::new(r"\b\w{3}\s+\d{1,2}\s+\d{2}:\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\b\w{3}\s+\d{1,2}\s+\d{4}\b")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\s->\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.extend([
        Rule::new(r"\S+\.(tar|gz|bz2|xz|zip|7z|rar)$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\S+\.(jpg|jpeg|png|gif|bmp|svg|ico)$")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\S+\.(mp3|mp4|avi|mkv|wav|flac)$")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules
}

pub fn ls_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.ls",
            "ls",
            "Directory listing output",
            "system",
            ls_rules(),
        )
        .with_detect_patterns(vec!["ls", "exa", "eza", "lsd"]),
    )
}

// =============================================================================
// DF (Disk Free)
// =============================================================================

fn df_rules() -> Vec<Rule> {
    let mut rules = vec![
        common::filesystem_type_rule(),
        common::device_name_rule(),
        Rule::new(r"/[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ];

    rules.push(
        Rule::new(r"\b(9\d|100)%")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );
    rules.push(
        Rule::new(r"\b[78]\d%")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );
    rules.push(
        Rule::new(r"\b[0-6]?\d%")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

    rules.push(common::size_rule());
    rules.push(
        Rule::new(r"\b(Filesystem|Size|Used|Avail|Use%|Mounted on|1K-blocks|1M-blocks)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn df_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.df",
            "df",
            "Disk space usage output",
            "system",
            df_rules(),
        )
        .with_detect_patterns(vec!["df"]),
    )
}

// =============================================================================
// DU (Disk Usage)
// =============================================================================

fn du_rules() -> Vec<Rule> {
    let mut rules = vec![
        common::size_rule(),
        Rule::new(r"^\s*\d+[KMGT]?\s")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"^\s*\d+[KMGT]?\s+total$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"/[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\t\.$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ];

    rules.push(common::number_rule());
    rules
}

pub fn du_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.du",
            "du",
            "Disk usage output",
            "system",
            du_rules(),
        )
        .with_detect_patterns(vec!["du", "ncdu", "dust"]),
    )
}

// =============================================================================
// STAT (File Statistics)
// =============================================================================

fn stat_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^\s*(File|Size|Blocks|IO Block|Device|Inode|Links|Access|Uid|Gid|Context|Birth|Modify|Change):")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"File:\s*'([^']+)'")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"File:\s*(\S+)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(regular file|directory|symbolic link|block special file|character special file|socket|FIFO)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        common::permission_rule(),
        common::size_rule(),
        common::iso_timestamp_rule(),
        common::iso_timestamp_space_rule(),
        Rule::new(r"\(\s*\d+/\s*\w+\)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b0[0-7]{3,4}\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\b[a-f0-9]+h/\d+d\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        common::number_rule(),
    ]
}

pub fn stat_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.stat",
            "stat",
            "File statistics output",
            "system",
            stat_rules(),
        )
        // Use space prefix to avoid matching "git status" -> "git [stat]us"
        .with_detect_patterns(vec![" stat "]),
    )
}

// =============================================================================
// MOUNT
// =============================================================================

fn mount_rules() -> Vec<Rule> {
    let mut rules = vec![
        common::device_name_rule(),
        common::filesystem_type_rule(),
        Rule::new(r"\bon /[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\(([^)]+)\)")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ];

    rules.extend([
        Rule::new(r"\b(ro|rw)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bnoexec\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bnosuid\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    rules.push(
        Rule::new(r"\btype\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn mount_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.mount",
            "mount",
            "Filesystem mount output",
            "system",
            mount_rules(),
        )
        .with_detect_patterns(vec!["mount", "findmnt"]),
    )
}
