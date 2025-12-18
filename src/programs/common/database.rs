//! Database-related patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// SQL keywords for database logs.
#[must_use]
pub fn sql_keyword_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER|TRUNCATE)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(FROM|WHERE|JOIN|LEFT|RIGHT|INNER|OUTER|ON|AND|OR|NOT|IN|LIKE|ORDER BY|GROUP BY|HAVING|LIMIT|OFFSET)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b(BEGIN|COMMIT|ROLLBACK|TRANSACTION|SAVEPOINT)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]
}

/// Database connection events.
#[must_use]
pub fn db_connection_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(connected|connection established|authenticated|authorized)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(disconnected|connection closed|connection lost|connection refused)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\b(connecting|authenticating|reconnecting)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_keyword_rules_compile() {
        let rules = sql_keyword_rules();
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn test_db_connection_rules_compile() {
        let rules = db_connection_rules();
        assert_eq!(rules.len(), 3);
    }
}
