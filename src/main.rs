use clap::{Arg, Command};

fn main() {
    let matches = Command::new("program")
        .about("A simple greeting program")
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
        .get_matches();

    let names: Vec<&str> = matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let caps = matches.get_flag("caps");
    let surname = matches.get_one::<String>("surname").map(String::as_str);

    generate_greetings(names, caps, surname);
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