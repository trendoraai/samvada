mod greeting;
mod goodbye;
mod chat;

use clap::{ArgMatches, Command};

#[tokio::main]
async fn main() {
    let matches = parse_arguments();

    match matches.subcommand() {
        Some(("greeting", sub_m)) => {
            greeting::handle_greeting_subcommand(sub_m);
        }
        Some(("goodbye", sub_m)) => {
            goodbye::handle_goodbye_subcommand(sub_m);
        }
        Some(("chat", sub_m)) => {
            chat::handle_chat_subcommand(sub_m).await;
        }
        _ => println!("No valid subcommand was used"),
    }
}

fn parse_arguments() -> ArgMatches {
    Command::new("hello_world")
        .about("A simple greeting program")
        .subcommand(greeting::greeting_command())
        .subcommand(goodbye::goodbye_command())
        .subcommand(chat::chat_command())
        .get_matches()
}
