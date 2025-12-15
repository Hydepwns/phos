//! Command handlers for phos CLI.
//!
//! Each subcommand has its own module with the handler function.

pub mod colors;
pub mod completions;
pub mod config;
pub mod info;
pub mod list;
pub mod man;
pub mod preview;
pub mod run;
pub mod shell_init;
pub mod themes;

pub use colors::show_colors;
pub use completions::generate_completions;
pub use config::{handle_config_action, ConfigAction};
pub use info::show_info;
pub use list::list_programs;
pub use man::generate_man_page;
pub use preview::preview_themes;
pub use run::run_command;
pub use shell_init::generate_shell_init;
pub use themes::list_themes;
