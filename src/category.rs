//! Program category definitions.
//!
//! Provides type-safe category classification for programs.

use std::fmt;
use std::str::FromStr;

/// Categories for grouping programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    /// Ethereum consensus and execution clients
    Ethereum,
    /// Container orchestration and cloud tools
    DevOps,
    /// System utilities and logs
    System,
    /// Development tools and build systems
    Dev,
    /// Network diagnostics and servers
    Network,
    /// Database systems
    Data,
    /// Observability and metrics
    Monitoring,
    /// Message queues and event streaming
    Messaging,
    /// Continuous integration pipelines
    CI,
    /// User-defined custom programs
    Custom,
}

impl Category {
    /// Returns the string representation used in program IDs.
    #[must_use] pub fn as_str(&self) -> &'static str {
        match self {
            Category::Ethereum => "ethereum",
            Category::DevOps => "devops",
            Category::System => "system",
            Category::Dev => "dev",
            Category::Network => "network",
            Category::Data => "data",
            Category::Monitoring => "monitoring",
            Category::Messaging => "messaging",
            Category::CI => "ci",
            Category::Custom => "custom",
        }
    }

    /// Returns a human-readable description of the category.
    #[must_use] pub fn description(&self) -> &'static str {
        match self {
            Category::Ethereum => "Ethereum consensus and execution clients",
            Category::DevOps => "Container orchestration and cloud tools",
            Category::System => "System utilities and logs",
            Category::Dev => "Development tools and build systems",
            Category::Network => "Network diagnostics and servers",
            Category::Data => "Database systems",
            Category::Monitoring => "Observability and metrics",
            Category::Messaging => "Message queues and event streaming",
            Category::CI => "Continuous integration pipelines",
            Category::Custom => "User-defined custom programs",
        }
    }

    /// Returns a display name for the category.
    #[must_use] pub fn display_name(&self) -> &'static str {
        match self {
            Category::Ethereum => "Ethereum",
            Category::DevOps => "DevOps",
            Category::System => "System",
            Category::Dev => "Development",
            Category::Network => "Network",
            Category::Data => "Data",
            Category::Monitoring => "Monitoring",
            Category::Messaging => "Messaging",
            Category::CI => "CI/CD",
            Category::Custom => "Custom",
        }
    }

    /// Returns all category variants.
    #[must_use] pub fn all() -> &'static [Category] {
        &[
            Category::Ethereum,
            Category::DevOps,
            Category::System,
            Category::Dev,
            Category::Network,
            Category::Data,
            Category::Monitoring,
            Category::Messaging,
            Category::CI,
            Category::Custom,
        ]
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Error type for parsing a category from a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCategoryError(pub String);

impl fmt::Display for ParseCategoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown category '{}'. Valid categories: {}",
            self.0,
            Category::all()
                .iter()
                .map(Category::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::error::Error for ParseCategoryError {}

impl FromStr for Category {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" => Ok(Category::Ethereum),
            "devops" => Ok(Category::DevOps),
            "system" => Ok(Category::System),
            "dev" => Ok(Category::Dev),
            "network" => Ok(Category::Network),
            "data" => Ok(Category::Data),
            "monitoring" => Ok(Category::Monitoring),
            "messaging" => Ok(Category::Messaging),
            "ci" => Ok(Category::CI),
            "custom" => Ok(Category::Custom),
            _ => Err(ParseCategoryError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_as_str() {
        assert_eq!(Category::Ethereum.as_str(), "ethereum");
        assert_eq!(Category::DevOps.as_str(), "devops");
        assert_eq!(Category::CI.as_str(), "ci");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(format!("{}", Category::Ethereum), "ethereum");
        assert_eq!(format!("{}", Category::DevOps), "devops");
    }

    #[test]
    fn test_category_from_str() {
        assert_eq!("ethereum".parse::<Category>().unwrap(), Category::Ethereum);
        assert_eq!("DEVOPS".parse::<Category>().unwrap(), Category::DevOps);
        assert_eq!("Dev".parse::<Category>().unwrap(), Category::Dev);
    }

    #[test]
    fn test_category_from_str_error() {
        let err = "invalid".parse::<Category>().unwrap_err();
        assert_eq!(err.0, "invalid");
        assert!(err.to_string().contains("unknown category"));
    }

    #[test]
    fn test_category_all() {
        let all = Category::all();
        assert_eq!(all.len(), 10);
        assert!(all.contains(&Category::Ethereum));
        assert!(all.contains(&Category::CI));
        assert!(all.contains(&Category::Custom));
    }

    #[test]
    fn test_category_display_name() {
        assert_eq!(Category::Ethereum.display_name(), "Ethereum");
        assert_eq!(Category::DevOps.display_name(), "DevOps");
        assert_eq!(Category::CI.display_name(), "CI/CD");
    }
}
