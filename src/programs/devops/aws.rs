//! AWS CLI colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn aws_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // ARN patterns
    rules.push(
        Rule::new(r"arn:aws:[\w\-]+:[\w\-]*:\d*:[\w\-/\*]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // AWS error patterns
    rules.extend([
        Rule::new(r"An error occurred\s+\([^\)]+\)")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(AccessDenied|InvalidParameter|ResourceNotFound|ValidationError)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
    ]);

    // Service names
    rules.push(
        Rule::new(r"\b(s3|ec2|lambda|rds|iam|cloudformation|cloudwatch|sns|sqs|dynamodb|ecs|eks|route53)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Status values
    rules.extend([
        Rule::new(r"\b(ACTIVE|AVAILABLE|CREATE_COMPLETE|UPDATE_COMPLETE|running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(PENDING|IN_PROGRESS|CREATE_IN_PROGRESS|UPDATE_IN_PROGRESS|pending)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(FAILED|DELETE_FAILED|ROLLBACK_COMPLETE|stopped|terminated)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Resource IDs
    rules.extend([
        Rule::new(r"\bi-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bvpc-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bsubnet-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bsg-[a-f0-9]{8,17}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Table headers (common in aws cli output)
    rules.push(
        Rule::new(r"^[\|\+][\-\+]+[\|\+]$")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    );

    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn aws_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.aws",
            "aws",
            "AWS CLI output",
            Category::DevOps,
            aws_rules(),
        )
        .with_detect_patterns(vec!["aws"]),
    )
}
