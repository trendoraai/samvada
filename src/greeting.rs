use clap::{Arg, ArgMatches, Command};

pub fn handle_greeting_subcommand(matches: &ArgMatches) {
    let names = get_names(matches);
    let caps = matches.get_flag("caps");
    let surname = matches.get_one::<String>("surname").map(String::as_str);

    generate_greetings(names, caps, surname);
}

pub fn generate_greetings(names: Vec<&str>, caps: bool, surname: Option<&str>) {
    if !names.is_empty() {
        for name in names {
            let full_name = match surname {
                Some(surname) => format!("{} {}", name, surname),
                None => name.to_string(),
            };

            if caps {
                println!("Hello, {}!", full_name.to_uppercase());
            } else {
                println!("Hello, {}!", full_name);
            }
        }
    } else {
        println!("Hello, world!");
    }
}

fn get_names(matches: &ArgMatches) -> Vec<&str> {
    matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect()
}

pub fn greeting_command() -> Command {
    Command::new("greeting")
        .about("Greet someone")
        .arg(
            Arg::new("names")
                .help("Names to greet")
                .num_args(1..),
        )
        .arg(
            Arg::new("caps")
                .long("caps")
                .help("Print names in uppercase")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("surname")
                .long("surname")
                .num_args(1)
                .help("Append the surname to each name"),
        )
}