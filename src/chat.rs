use clap::{Arg, ArgMatches, Command};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::Utc;
use regex::Regex;

pub fn handle_chat_subcommand(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("new", new_m)) => {
            let name = new_m.get_one::<String>("name").unwrap();
            let dir = new_m.get_one::<String>("dir").map(String::as_str);

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
        Some(("lint", lint_m)) => {
            let path = lint_m.get_one::<String>("path").unwrap();

            if !validate_path(path) {
                eprintln!("Error: Invalid path.");
                std::process::exit(1);
            }

            if let Err(e) = lint_chat(path) {
                eprintln!("Error linting chat: {}", e);
                std::process::exit(1);
            }
        }
        _ => println!("No valid chat subcommand was used"),
    }
}

pub fn create_chat(name: &str, dir: Option<&str>) -> io::Result<()> {
    let created_at = Utc::now().to_rfc3339();
    let updated_at = created_at.clone();

    let frontmatter = format!(
        "---\nname: {}\nsystem: \ncreated_at: {}\nupdated_at: {}\ntags: []\n---\n\nuser:\n",
        name, created_at, updated_at
    );

    let mut file_path = PathBuf::from(dir.unwrap_or("."));
    file_path.push(format!("{}.md", name));

    let mut file = File::create(file_path)?;
    file.write_all(frontmatter.as_bytes())?;
    Ok(())
}

pub fn lint_chat(path: &str) -> Result<(), String> {
    let path = Path::new(path);

    if path.is_file() {
        lint_file(path)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                lint_file(&path)?;
            }
        }
    } else {
        return Err("Invalid path".to_string());
    }

    Ok(())
}

fn lint_file(file_path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let re = Regex::new(r"(?m)^---\nname: .+\nsystem: .*\ncreated_at: .+\nupdated_at: .+\ntags: \[.*\]\n---").unwrap();

    if re.is_match(&content) {
        println!("{} is valid.", file_path.display());
        Ok(())
    } else {
        Err(format!("{} is missing or has incorrect frontmatter", file_path.display()))
    }
}

fn validate_directory(dir: &str) -> bool {
    Path::new(dir).is_dir()
}

fn validate_path(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn chat_command() -> Command {
    Command::new("chat")
        .about("Manage chat files")
        .subcommand(
            Command::new("new")
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
                ),
        )
        .subcommand(
            Command::new("lint")
                .about("Lint a chat file or directory")
                .arg(
                    Arg::new("path")
                        .help("Path to the file or directory")
                        .required(true)
                        .num_args(1),
                ),
        )
}