use crate::chat::constants::FRONTMATTER_TEMPLATE;
use chrono::Utc;
use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::chat::config::load_config;

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

    // Use the frontmatter template from config
    let frontmatter = get_frontmatter_from_config(&title, &created_at, &updated_at)?;

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

fn get_frontmatter_from_config(
    title: &str,
    created_at: &str,
    updated_at: &str,
) -> io::Result<String> {
    let app_config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    // Use the loaded configurations
    let system_prompt = app_config.system_prompt;
    let model = app_config.model;
    let api_endpoint = app_config.api_endpoint;

    let frontmatter = FRONTMATTER_TEMPLATE
        .replace("{title}", title)
        .replace("{system}", &system_prompt)
        .replace("{model}", &model)
        .replace("{api_endpoint}", &api_endpoint)
        .replace("{created_at}", created_at)
        .replace("{updated_at}", updated_at)
        .replace("{tags}", "[]")
        .replace("{summary}", "");

    Ok(frontmatter)
}
