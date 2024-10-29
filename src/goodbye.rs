use clap::{Arg, ArgMatches, Command};

pub fn handle_goodbye_subcommand(matches: &ArgMatches) {
    let names = get_names(matches);
    let caps = matches.get_flag("caps");
    let surname = matches.get_one::<String>("surname").map(String::as_str);
    let date_after = matches.get_one::<String>("date-after").map(String::as_str);

    if let Some(date) = date_after {
        if !validate_date_format(date) {
            eprintln!("Error: Date must be in yyyy-mm-dd format.");
            std::process::exit(1);
        }
    }

    generate_goodbyes(names, caps, surname, date_after);
}

pub fn generate_goodbyes(
    names: Vec<&str>,
    caps: bool,
    surname: Option<&str>,
    date_after: Option<&str>,
) {
    if !names.is_empty() {
        for name in names {
            let full_name = match surname {
                Some(surname) => format!("{} {}", name, surname),
                None => name.to_string(),
            };

            let message = if caps {
                format!("Goodbye, {}!", full_name.to_uppercase())
            } else {
                format!("Goodbye, {}!", full_name)
            };

            if let Some(date) = date_after {
                println!("{}, see you after {}.", message, date);
            } else {
                println!("{}", message);
            }
        }
    } else {
        println!("Goodbye, world!");
    }
}

fn get_names(matches: &ArgMatches) -> Vec<&str> {
    matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect()
}

fn validate_date_format(date: &str) -> bool {
    let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}

pub fn goodbye_command() -> Command {
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
        )
}
