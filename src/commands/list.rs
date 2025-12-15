//! Program listing command.

use anyhow::Result;
use phos::{Category, ProgramRegistry};
use serde::Serialize;

use crate::OutputFormat;

/// Serializable program info for JSON output.
#[derive(Serialize)]
pub struct ProgramJson {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub rules: usize,
}

#[derive(Serialize)]
pub struct ProgramListJson {
    pub total: usize,
    pub programs: Vec<ProgramJson>,
}

/// List available programs, optionally filtered by category.
pub fn list_programs(
    registry: &ProgramRegistry,
    category: Option<&str>,
    format: OutputFormat,
) -> Result<()> {
    let categories: Vec<Category> = category
        .map(|c| vec![c.parse::<Category>().expect("Invalid category")])
        .unwrap_or_else(|| registry.categories());

    match format {
        OutputFormat::Json => {
            let programs: Vec<ProgramJson> = categories
                .iter()
                .flat_map(|cat| registry.list_by_category(*cat))
                .map(|info| {
                    let program = registry.get(&info.id);
                    ProgramJson {
                        id: info.id.clone(),
                        name: info.name.clone(),
                        description: info.description.clone(),
                        category: info.category.to_string(),
                        rules: program.map(|p| p.rules().len()).unwrap_or(0),
                    }
                })
                .collect();

            let output = ProgramListJson {
                total: programs.len(),
                programs,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Table => {
            println!("Available programs ({} total):\n", registry.len());

            categories
                .iter()
                .filter_map(|cat| {
                    let programs = registry.list_by_category(*cat);
                    (!programs.is_empty()).then_some((*cat, programs))
                })
                .for_each(|(cat, programs)| {
                    println!("{}:", cat.display_name());
                    programs.iter().for_each(|info| {
                        let name = info.id.split('.').next_back().unwrap_or(&info.id);
                        println!("  {:12} - {}", name, info.description);
                    });
                    println!();
                });

            // Also show Ethereum layer info if showing ethereum category
            if category == Some("ethereum") {
                println!("Ethereum clients by layer:");
                println!("  Consensus:  Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda");
                println!("  Execution:  Geth, Nethermind, Besu, Erigon, Reth");
                println!("  Full Node:  Mana");
                println!("  Middleware: Charon, MEV-Boost");
            }
        }
    }

    Ok(())
}
