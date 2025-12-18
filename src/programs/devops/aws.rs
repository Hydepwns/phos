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

#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // COMPILE TESTS
    // =========================================================================

    #[test]
    fn test_aws_rules_compile() {
        let rules = aws_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_aws_program_info() {
        let program = aws_program();
        assert_eq!(program.info().id, "devops.aws");
        assert_eq!(program.info().name, "aws");
    }

    // =========================================================================
    // ARN PATTERN MATCHING
    // =========================================================================

    #[test]
    fn test_arn_patterns() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "arn:aws:s3:::my-bucket"));
        assert!(any_rule_matches(
            &rules,
            "arn:aws:iam::123456789012:role/my-role"
        ));
        assert!(any_rule_matches(
            &rules,
            "arn:aws:lambda:us-east-1:123456789012:function:my-function"
        ));
        assert!(any_rule_matches(
            &rules,
            "arn:aws:ec2:us-west-2:123456789012:instance/i-1234567890abcdef0"
        ));
    }

    // =========================================================================
    // ERROR PATTERN MATCHING
    // =========================================================================

    #[test]
    fn test_error_occurred() {
        let rules = aws_rules();
        assert!(any_rule_matches(
            &rules,
            "An error occurred (AccessDenied) when calling the GetObject operation"
        ));
        assert!(any_rule_matches(
            &rules,
            "An error occurred (ResourceNotFoundException)"
        ));
    }

    #[test]
    fn test_error_types() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "AccessDenied"));
        assert!(any_rule_matches(&rules, "InvalidParameter"));
        assert!(any_rule_matches(&rules, "ResourceNotFound"));
        assert!(any_rule_matches(&rules, "ValidationError"));
    }

    // =========================================================================
    // SERVICE NAME MATCHING
    // =========================================================================

    #[test]
    fn test_service_names() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "aws s3 ls"));
        assert!(any_rule_matches(&rules, "aws ec2 describe-instances"));
        assert!(any_rule_matches(&rules, "aws lambda invoke"));
        assert!(any_rule_matches(&rules, "aws rds describe-db-instances"));
        assert!(any_rule_matches(&rules, "aws iam list-users"));
        assert!(any_rule_matches(
            &rules,
            "aws cloudformation describe-stacks"
        ));
        assert!(any_rule_matches(&rules, "aws cloudwatch get-metric-data"));
        assert!(any_rule_matches(&rules, "aws sns publish"));
        assert!(any_rule_matches(&rules, "aws sqs send-message"));
        assert!(any_rule_matches(&rules, "aws dynamodb scan"));
        assert!(any_rule_matches(&rules, "aws ecs list-clusters"));
        assert!(any_rule_matches(&rules, "aws eks list-clusters"));
        assert!(any_rule_matches(&rules, "aws route53 list-hosted-zones"));
    }

    // =========================================================================
    // STATUS VALUE MATCHING
    // =========================================================================

    #[test]
    fn test_status_active() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "Status: ACTIVE"));
        assert!(any_rule_matches(&rules, "State: AVAILABLE"));
        assert!(any_rule_matches(&rules, "StackStatus: CREATE_COMPLETE"));
        assert!(any_rule_matches(&rules, "Status: UPDATE_COMPLETE"));
        assert!(any_rule_matches(&rules, "State: running"));
    }

    #[test]
    fn test_status_pending() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "Status: PENDING"));
        assert!(any_rule_matches(&rules, "State: IN_PROGRESS"));
        assert!(any_rule_matches(&rules, "StackStatus: CREATE_IN_PROGRESS"));
        assert!(any_rule_matches(&rules, "Status: UPDATE_IN_PROGRESS"));
        assert!(any_rule_matches(&rules, "State: pending"));
    }

    #[test]
    fn test_status_failed() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "Status: FAILED"));
        assert!(any_rule_matches(&rules, "StackStatus: DELETE_FAILED"));
        assert!(any_rule_matches(&rules, "Status: ROLLBACK_COMPLETE"));
        assert!(any_rule_matches(&rules, "State: stopped"));
        assert!(any_rule_matches(&rules, "State: terminated"));
    }

    // =========================================================================
    // RESOURCE ID MATCHING
    // =========================================================================

    #[test]
    fn test_ec2_instance_id() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "i-1234567890abcdef0"));
        assert!(any_rule_matches(&rules, "InstanceId: i-abcdef12"));
    }

    #[test]
    fn test_vpc_id() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "vpc-1234567890abcdef0"));
        assert!(any_rule_matches(&rules, "VpcId: vpc-abcdef12"));
    }

    #[test]
    fn test_subnet_id() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "subnet-1234567890abcdef0"));
        assert!(any_rule_matches(&rules, "SubnetId: subnet-abcdef12"));
    }

    #[test]
    fn test_security_group_id() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "sg-1234567890abcdef0"));
        assert!(any_rule_matches(&rules, "SecurityGroupId: sg-abcdef12"));
    }

    // =========================================================================
    // TABLE HEADER MATCHING
    // =========================================================================

    #[test]
    fn test_table_headers() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "+-------------+"));
        assert!(any_rule_matches(&rules, "|-------------|"));
    }

    // =========================================================================
    // LOG LEVELS
    // =========================================================================

    #[test]
    fn test_log_levels() {
        let rules = aws_rules();
        assert!(any_rule_matches(&rules, "ERROR: Failed to connect"));
        assert!(any_rule_matches(&rules, "WARNING: Rate limit exceeded"));
    }

    // =========================================================================
    // REAL OUTPUT EXAMPLES
    // =========================================================================

    #[test]
    fn test_aws_ec2_output() {
        let rules = aws_rules();
        let line = "i-1234567890abcdef0  t2.micro  running  us-east-1a";
        assert!(any_rule_matches(&rules, line));
    }

    #[test]
    fn test_aws_cloudformation_output() {
        let rules = aws_rules();
        assert!(any_rule_matches(
            &rules,
            "arn:aws:cloudformation:us-east-1:123456789012:stack/my-stack/guid"
        ));
        assert!(any_rule_matches(&rules, "StackStatus: CREATE_COMPLETE"));
    }
}
