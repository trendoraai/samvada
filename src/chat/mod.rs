pub mod create;
pub mod lint;
pub mod constants;

use clap::ArgMatches;

pub fn handle_chat_subcommand(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", new_m)) => {
            create::handle_create_subcommand(new_m);
        }
        Some(("lint", lint_m)) => {
            lint::handle_lint_subcommand(lint_m);
        }
        _ => println!("No valid chat subcommand was used"),
    }
}

pub fn chat_command() -> clap::Command {
    clap::Command::new("chat")
        .about("Manage chat files")
        .subcommand(create::create_command())
        .subcommand(lint::lint_command())
}