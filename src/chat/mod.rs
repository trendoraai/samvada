pub mod ask;
pub mod quick;
pub mod constants;
pub mod create;
pub mod lint;

// Internal shared modules
mod api;
mod config;
mod logging;
mod parser;

use clap::ArgMatches;

pub async fn handle_chat_subcommand(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", new_m)) => {
            create::handle_create_subcommand(new_m);
        }
        Some(("lint", lint_m)) => {
            lint::handle_lint_subcommand(lint_m);
        }
        Some(("ask", ask_m)) => {
            ask::handle_ask_subcommand(ask_m).await;
        }
        Some(("quick", quick_m)) => {      // Add this block
            quick::handle_quick_subcommand(quick_m).await;
        }
        _ => println!("No valid chat subcommand was used"),
    }
}

pub fn chat_command() -> clap::Command {
    clap::Command::new("chat")
        .about("Manage chat files")
        .subcommand(create::create_command())
        .subcommand(lint::lint_command())
        .subcommand(ask::ask_command())
        .subcommand(quick::quick_command())   // Add this line
}