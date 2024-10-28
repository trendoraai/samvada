use std::env;

fn main() {
    // Parse command-line arguments
    let (names, caps, surname) = parse_args();

    // Generate greetings
    generate_greetings(names, caps, surname);
}

fn parse_args() -> (Vec<String>, bool, Option<String>) {
    let args: Vec<String> = env::args().collect();

    // Check for the --caps flag
    let caps = args.contains(&String::from("--caps"));

    // Find the --surname option and get the surname
    let surname_index = args.iter().position(|arg| arg == "--surname");
    let surname = surname_index.and_then(|index| args.get(index + 1).cloned());

    // Filter out options and their values from the names
    let names: Vec<String> = args.iter()
        .skip(1)
        .filter(|&arg| arg != "--caps" && arg != "--surname" && Some(arg) != surname.as_ref())
        .cloned()
        .collect();

    (names, caps, surname)
}

fn generate_greetings(names: Vec<String>, caps: bool, surname: Option<String>) {
    if !names.is_empty() {
        for name in names {
            let full_name = match &surname {
                Some(surname) => format!("{} {}", name, surname),
                None => name,
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