mod greeting;
mod goodbye;

use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::process;

fn main() {
    let matches = parse_arguments();

    match matches.subcommand() {
        Some(("greeting", sub_m)) => {
            let names = get_names(sub_m);
            let caps = sub_m.get_flag("caps");
            let surname = sub_m.get_one::<String>("surname").map(String::as_str);

            greeting::generate_greetings(names, caps, surname);
        }
        Some(("goodbye", sub_m)) => {
            let names = get_names(sub_m);
            let caps = sub_m.get_flag("caps");
            let surname = sub_m.get_one::<String>("surname").map(String::as_str);
            let date_after = sub_m.get_one::<String>("date-after").map(String::as_str);

            if let Some(date) = date_after {
                if !validate_date_format(date) {
                    eprintln!("Error: Date must be in yyyy-mm-dd format.");
                    process::exit(1);
                }
            }

            goodbye::generate_goodbyes(names, caps, surname, date_after);
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
                .about("Say goodbye to someone")
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
                )
                .arg(
                    Arg::new("date-after")
                        .long("date-after")
                        .num_args(1)
                        .help("Specify a date in yyyy-mm-dd format"),
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

fn validate_date_format(date: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}