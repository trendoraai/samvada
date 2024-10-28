use clap::{Arg, ArgMatches, Command};

fn main() {
    let matches = parse_arguments();

    match matches.subcommand() {
        Some(("greeting", sub_m)) => {
            let names = get_names(sub_m);
            let caps = sub_m.get_flag("caps");
            let surname = sub_m.get_one::<String>("surname").map(String::as_str);

            generate_greetings(names, caps, surname);
        }
        Some(("goodbye", sub_m)) => {
            let names = get_names(sub_m);
            let caps = sub_m.get_flag("caps");
            let surname = sub_m.get_one::<String>("surname").map(String::as_str);

            generate_goodbyes(names, caps, surname);
        }
        _ => println!("No valid subcommand was used"),
    }
}

fn parse_arguments() -> ArgMatches {
    Command::new("hello_world")
        .about("A simple greeting program")
        .subcommand(
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
                ),
        )
        .subcommand(
            Command::new("goodbye")
                .about("Say goodbye")
                .arg(
                    Arg::new("names")
                        .help("Names to say goodbye to")
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
                ),
        )
        .get_matches()
}

fn get_names(matches: &ArgMatches) -> Vec<&str> {
    matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect()
}

fn generate_greetings(names: Vec<&str>, caps: bool, surname: Option<&str>) {
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

fn generate_goodbyes(names: Vec<&str>, caps: bool, surname: Option<&str>) {
    if !names.is_empty() {
        for name in names {
            let full_name = match surname {
                Some(surname) => format!("{} {}", name, surname),
                None => name.to_string(),
            };

            if caps {
                println!("Goodbye, {}!", full_name.to_uppercase());
            } else {
                println!("Goodbye, {}!", full_name);
            }
        }
    } else {
        println!("Goodbye, world!");
    }
}