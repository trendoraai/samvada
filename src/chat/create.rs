use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::Utc;
use clap::{Arg, ArgMatches, Command};
use crate::chat::constants::FRONTMATTER_TEMPLATE;

pub fn handle_create_subcommand(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let dir = matches.get_one::<String>("dir").map(String::as_str);

    if let Some(directory) = dir {
        if !validate_directory(directory) {
            eprintln!("Error: Directory does not exist.");
            std::process::exit(1);
        }
    }

    if let Err(e) = create_chat(name, dir) {
        eprintln!("Error creating chat: {}", e);
        std::process::exit(1);
    }
}

pub fn create_chat(name: &str, dir: Option<&str>) -> io::Result<()> {
    let created_at = Utc::now().to_rfc3339();
    let updated_at = created_at.clone();

    // Convert name to title case for frontmatter
    let title = name.to_string(); // Assuming name is already in title case or needs no conversion

    // Use the frontmatter template
    let frontmatter = FRONTMATTER_TEMPLATE
        .replace("{title}", &title)
        .replace("{system}", "") // Replace with actual system value
        .replace("{model}", "gpt-4o")
        .replace("{created_at}", &created_at)
        .replace("{updated_at}", &updated_at)
        .replace("{tags}", "[]") // Replace with actual tags if needed
        .replace("{summary}", ""); // Replace with actual summary if needed

    let mut file_path = PathBuf::from(dir.unwrap_or("."));
    file_path.push(format!("{}.md", name));

    let mut file = File::create(file_path)?;
    file.write_all(frontmatter.as_bytes())?;
    Ok(())
}

pub fn validate_directory(dir: &str) -> bool {
    std::path::Path::new(dir).is_dir()
}

pub fn create_command() -> Command {
    Command::new("create")
        .about("Create a new chat file")
        .arg(
            Arg::new("name")
                .help("Name of the chat file")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .help("Directory to create the file in")
                .num_args(1),
        )
}
