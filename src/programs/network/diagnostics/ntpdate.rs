//! NTP time synchronization colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::programs::common;
use crate::rule::Rule;

fn ntpdate_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamp
    rules.push(common::syslog_timestamp_rule());

    // Server info
    rules.push(
        Rule::new(r"\bserver\s+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // IP addresses
    rules.extend(common::ip_rules());

    // Stratum
    rules.push(
        Rule::new(r"\bstratum\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Offset values
    rules.extend([
        Rule::new(r"\boffset\s+[\-\+]?\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\boffset\s+[\-\+]?0\.0+\d*\s+sec")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\boffset\s+[\-\+]?[1-9]\d*\.\d+\s+sec")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Delay
    rules.push(
        Rule::new(r"\bdelay\s+[\-\+]?\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Status messages
    rules.extend([
        Rule::new(r"\badjust time server\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bstep time server\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bno server suitable\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bthe NTP socket is in use\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Reach values (reachability register)
    rules.push(
        Rule::new(r"\breach\s+[0-7]+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // Jitter
    rules.push(
        Rule::new(r"\bjitter\s+\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // Reference ID
    rules.push(
        Rule::new(r"\brefid\s+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    // Poll interval
    rules.push(
        Rule::new(r"\bpoll\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    );

    // ntpq/ntpdc specific
    rules.extend([
        Rule::new(r"^\*[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^\+[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^-[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^x[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    rules.push(common::number_rule());
    rules
}

pub fn ntpdate_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.ntpdate",
            "ntpdate",
            "NTP time synchronization output",
            Category::Network,
            ntpdate_rules(),
        )
        .with_detect_patterns(vec!["ntpdate", "ntpd", "ntpq", "chronyd", "chronyc"]),
    )
}
