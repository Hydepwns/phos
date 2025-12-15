//! Process and resource monitoring programs.
//!
//! Provides Program implementations for ps, free, top, uptime, and lsof.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// PS (Process Status)
// =============================================================================

fn ps_rules() -> Vec<Rule> {
    let mut rules = common::process_state_rules();

    rules.push(
        Rule::new(r"^\s*\d+\s")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );
    rules.push(
        Rule::new(r"\b(PID|TTY|TIME|CMD|USER|%CPU|%MEM|VSZ|RSS|STAT|START|COMMAND|PPID|NI|PRI)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );
    rules.push(
        Rule::new(r"^\s*root\s")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    );
    rules.push(
        Rule::new(r"\b([5-9]\d\.\d|100\.0)\s")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );
    rules.push(
        Rule::new(r"\b(pts/\d+|tty\d+|\?)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );
    rules.push(
        Rule::new(r"\d{1,2}:\d{2}(:\d{2})?")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.push(
        Rule::new(r"\[([\w/\-:]+)\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    rules.push(common::number_rule());
    rules
}

pub fn ps_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.ps",
            "ps",
            "Process status output",
            Category::System,
            ps_rules(),
        )
        .with_detect_patterns(vec!["ps"]),
    )
}

// =============================================================================
// FREE (Memory Usage)
// =============================================================================

fn free_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(total|used|free|shared|buff/cache|available|buffers|cached|Swap)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^(Mem|Swap):")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        common::size_rule(),
        Rule::new(r"\b\d{4,}\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\b0\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        common::number_rule(),
    ]
}

pub fn free_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.free",
            "free",
            "Memory usage output",
            Category::System,
            free_rules(),
        )
        .with_detect_patterns(vec!["free"]),
    )
}

// =============================================================================
// TOP (Process Monitor)
// =============================================================================

fn top_rules() -> Vec<Rule> {
    let mut rules = vec![
        Rule::new(r"load average:\s*(\d+\.\d+,\s*){0,2}\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ];

    rules.extend(common::process_state_rules());

    rules.push(
        Rule::new(r"\b(PID|USER|PR|NI|VIRT|RES|SHR|S|%CPU|%MEM|TIME\+?|COMMAND)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );
    rules.push(
        Rule::new(r"\b(Tasks|total|running|sleeping|stopped|zombie|Cpu|MiB Mem|MiB Swap|us|sy|ni|id|wa|hi|si|st)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );
    rules.push(
        Rule::new(r"\b([5-9]\d\.\d|100\.0)\s")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
    );

    rules.push(common::size_rule());
    rules.push(
        Rule::new(r"\b\d+[mgk]?\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );
    rules.push(
        Rule::new(r"up\s+\d+\s+(days?|hours?|min)")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.push(
        Rule::new(r"\d+\s+users?")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    );

    rules.push(common::percentage_rule());
    rules.push(common::number_rule());
    rules
}

pub fn top_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.top",
            "top",
            "Process monitor output",
            Category::System,
            top_rules(),
        )
        .with_detect_patterns(vec!["top", "htop", "btop", "atop"]),
    )
}

// =============================================================================
// UPTIME
// =============================================================================

fn uptime_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^\s*\d{1,2}:\d{2}(:\d{2})?\s")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"up\s+\d+\s+(days?|hours?|min(utes?)?)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"up\s+\d+:\d{2}")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\d+\s+users?")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"load average:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"load average:.*\b([2-9]\d*\.\d+|\d{2,}\.\d+)")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b1\.\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b0\.\d+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        common::number_rule(),
    ]
}

pub fn uptime_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.uptime",
            "uptime",
            "System uptime output",
            Category::System,
            uptime_rules(),
        )
        .with_detect_patterns(vec!["uptime"]),
    )
}

// =============================================================================
// LSOF (List Open Files)
// =============================================================================

fn lsof_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"^COMMAND\s+PID\s+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\b(COMMAND|PID|TID|USER|FD|TYPE|DEVICE|SIZE/OFF|NODE|NAME)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b(cwd|rtd|txt|mem|DEL|[0-9]+[rwu]?)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(REG|DIR|CHR|BLK|FIFO|unix|IPv4|IPv6|sock|LINK|unknown)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\(deleted\)")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"/[^\s]+\.sock(et)?")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"/[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\broot\s")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        common::number_rule(),
    ]
}

pub fn lsof_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.lsof",
            "lsof",
            "List open files output",
            Category::System,
            lsof_rules(),
        )
        .with_detect_patterns(vec!["lsof"]),
    )
}
