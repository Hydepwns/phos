//! Man page generation command.

use anyhow::Result;
use clap::CommandFactory;
use clap_mangen::Man;
use std::fs;
use std::path::PathBuf;

use crate::Cli;

/// Generate man page(s) for phos.
pub fn generate_man_page(output: Option<String>) -> Result<()> {
    let cmd = Cli::command();
    let man = Man::new(cmd.clone());

    match output {
        Some(dir) => {
            let out_dir = PathBuf::from(&dir);
            fs::create_dir_all(&out_dir)?;

            // Generate main man page
            let main_path = out_dir.join("phos.1");
            let mut file = fs::File::create(&main_path)?;
            man.render(&mut file)?;
            println!("Generated: {}", main_path.display());

            // Generate man pages for subcommands
            for subcommand in cmd.get_subcommands() {
                let name = subcommand.get_name();
                if name == "help" {
                    continue;
                }
                let sub_path = out_dir.join(format!("phos-{name}.1"));
                let mut file = fs::File::create(&sub_path)?;
                let sub_man = Man::new(subcommand.clone());
                sub_man.render(&mut file)?;
                println!("Generated: {}", sub_path.display());
            }

            println!("\nInstall with:");
            println!("  sudo cp {dir}/*.1 /usr/local/share/man/man1/");
            println!("  sudo mandb  # Linux only");
        }
        None => {
            // Print to stdout
            man.render(&mut std::io::stdout())?;
        }
    }

    Ok(())
}
