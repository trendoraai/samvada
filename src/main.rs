use std::env;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check for the --caps flag
    let caps = args.contains(&String::from("--caps"));

    // Find the --surname option and get the surname
    let surname_index = args.iter().position(|arg| arg == "--surname");
    let surname = surname_index.and_then(|index| args.get(index + 1));

    // Filter out options and their values from the names
    let names: Vec<&String> = args.iter()
        .skip(1)
        .filter(|&arg| arg != "--caps" && arg != "--surname" && Some(arg) != surname)
        .collect();

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