//! Program info command.

use anyhow::Result;
use phos::programs::ethereum;
use phos::{Category, ProgramRegistry};
use serde::Serialize;

use crate::OutputFormat;

/// Extended program info for JSON output (includes Ethereum metadata when applicable).
#[derive(Serialize)]
pub struct ProgramInfoJson {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub rules: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
}

/// Show detailed information about a program.
pub fn show_info(registry: &ProgramRegistry, name: &str, format: OutputFormat) -> Result<()> {
    let program = registry.get(name).ok_or_else(|| {
        anyhow::anyhow!("Unknown program: {name}. Run 'phos list' to see available programs.")
    })?;

    let info = program.info();

    match format {
        OutputFormat::Json => {
            let meta = if info.category == Category::Ethereum {
                ethereum::client_meta(&info.name)
            } else {
                None
            };

            let output = ProgramInfoJson {
                id: info.id.to_string(),
                name: info.name.to_string(),
                description: info.description.to_string(),
                category: info.category.to_string(),
                rules: program.rules().len(),
                layer: meta.map(|m| format!("{:?}", m.layer)),
                language: meta.map(|m| m.language.to_string()),
                website: meta.map(|m| m.website.to_string()),
                brand_color: meta.map(|m| m.brand_color.to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Table => {
            println!("{}", info.name);
            println!("  ID:          {}", info.id);
            println!("  Description: {}", info.description);
            println!("  Category:    {}", info.category);
            println!("  Rules:       {}", program.rules().len());

            // Show extra info for Ethereum clients
            if info.category == Category::Ethereum {
                if let Some(meta) = ethereum::client_meta(&info.name) {
                    println!("  Layer:       {:?}", meta.layer);
                    println!("  Language:    {}", meta.language);
                    println!("  Website:     {}", meta.website);
                    println!("  Brand color: {}", meta.brand_color);
                }
            }
        }
    }

    Ok(())
}
