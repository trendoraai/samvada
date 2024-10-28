pub mod frontmatter;

use std::fs;
use std::path::Path;
use regex::Regex;
use clap::{Arg, ArgMatches, Command};

pub fn handle_lint_subcommand(matches: &ArgMatches) {
    let path = matches.get_one::<String>("path").unwrap();

    if !validate_path(path) {
        eprintln!("Error: Invalid path.");
        std::process::exit(1);
    }

    if let Err(e) = lint_chat(path) {
        eprintln!("Error linting chat: {}", e);
        std::process::exit(1);
    }
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
    let re = Regex::new(frontmatter::FRONTMATTER_TEMPLATE).unwrap();

    if re.is_match(&content) {
        println!("{} is valid.", file_path.display());
        Ok(())
    } else {
        Err(format!("{} is missing or has incorrect frontmatter", file_path.display()))
    }
}

pub fn validate_path(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn lint_command() -> Command {
    Command::new("lint")
        .about("Lint a chat file or directory")
        .arg(
            Arg::new("path")
                .help("Path to the file or directory")
                .required(true)
                .num_args(1),
        )
}
