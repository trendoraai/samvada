use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use clap::{Arg, ArgMatches, Command};
use crate::chat::constants::FRONTMATTER_TEMPLATE;

/// Handles the lint subcommand based on provided CLI arguments.
pub fn handle_lint_subcommand(matches: &ArgMatches) {
    let path = matches.get_one::<String>("path").expect("Path argument is required.");

    if !is_valid_path(path) {
        eprintln!("Error: Invalid path.");
        std::process::exit(1);
    }

    if let Err(e) = lint_path(path) {
        eprintln!("Error linting chat: {}", e);
        std::process::exit(1);
    }
}

/// Lints the provided path, which can be a file or directory.
pub fn lint_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);

    if path.is_file() {
        validate_chat_file(path)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_path = entry.path();
            if file_path.is_file() {
                validate_chat_file(&file_path)?;
            }
        }
    } else {
        return Err("Invalid path: Not a file or directory.".to_string());
    }

    Ok(())
}

/// Validates a single chat file.
fn validate_chat_file(file_path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

    validate_frontmatter(&content)
        .map_err(|e| format!("{}: {}", file_path.display(), e))?;

    validate_chat_structure(&content, file_path)?;

    println!("{} is valid.", file_path.display());
    Ok(())
}

/// Validates the frontmatter of the chat content.
fn validate_frontmatter(content: &str) -> Result<(), String> {
    let key_pattern = Regex::new(r"\{(\w+)\}")
        .map_err(|e| format!("Regex compilation error: {}", e))?;
    let keys: Vec<String> = key_pattern.captures_iter(FRONTMATTER_TEMPLATE)
        .map(|cap| cap[1].to_string())
        .collect();

    for key in keys {
        let key_regex = Regex::new(&format!(r"{}:\s*.+", regex::escape(&key)))
            .map_err(|e| format!("Regex compilation error for key '{}': {}", key, e))?;
        if !key_regex.is_match(content) {
            return Err(format!(
                "Frontmatter error: '{}' is missing or has incorrect format.",
                key
            ));
        }
    }

    Ok(())
}

/// Validates the structure of the chat content.
fn validate_chat_structure(content: &str, file_path: &Path) -> Result<(), String> {
    let frontmatter_end = content.rfind("---")
        .ok_or("Missing frontmatter end delimiter '---'.")?;
    let chat_content = &content[frontmatter_end + 3..];

    if chat_content.trim().is_empty() {
        return Err("Chat structure error: No content after frontmatter.".to_string());
    }

    if !is_first_entry_user(chat_content) {
        return Err("Chat structure error: First entry after frontmatter must start with 'user:'.".to_string());
    }

    if !validate_alternating_entries(chat_content) {
        return Err("Chat structure error: Entries must alternate between 'user:' and 'assistant:'.".to_string());
    }

    if !is_last_entry_user(chat_content) {
        return Err("Chat structure error: Last entry must start with 'user:'.".to_string());
    }

    if !validate_file_references(chat_content, file_path) {
        // Errors are logged within `validate_file_references`.
        return Err("File reference validation failed.".to_string());
    }

    Ok(())
}

/// Validates file references within the chat content.
/// Returns `false` if any referenced file does not exist, logging the error.
fn validate_file_references(content: &str, current_file_path: &Path) -> bool {
    let user_pattern = Regex::new(r"(?m)^user:")
        .expect("Failed to compile user entry regex.");
    let file_pattern = Regex::new(r"\[\[([^\]\n]+)\]\]")
        .expect("Failed to compile file reference regex.");

    let user_messages: Vec<&str> = user_pattern.split(content).skip(1).collect();
    let mut all_valid = true;

    for message in user_messages {
        for cap in file_pattern.captures_iter(message) {
            if let Some(file_ref) = cap.get(1) {
                let referenced_path = file_ref.as_str();
                let resolved_path = resolve_referenced_path(referenced_path, current_file_path);
                if !resolved_path.exists() {
                    eprintln!("File reference error: '{}' not found.", referenced_path);
                    all_valid = false;
                }
            }
        }
    }

    all_valid
}

/// Resolves the full path of a referenced file based on the current file's path.
fn resolve_referenced_path(file_path: &str, current_file_path: &Path) -> PathBuf {
    let sanitized_path = file_path.replace(r"\ ", " ");  // Handle escaped spaces
    let path = Path::new(&sanitized_path);

    if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_file_path.parent().map_or(path.to_path_buf(), |parent| parent.join(path))
    }
}

/// Checks if the first non-empty entry starts with 'user:'.
fn is_first_entry_user(content: &str) -> bool {
    let first_non_empty_line = content.lines()
        .map(|line| line.trim())
        .find(|line| !line.is_empty());

    match first_non_empty_line {
        Some(line) => line.starts_with("user:"),
        None => false,
    }
}

/// Ensures that entries alternate between 'user:' and 'assistant:'.
fn validate_alternating_entries(content: &str) -> bool {
    let mut expect_user = true;
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        if expect_user && trimmed_line.starts_with("user:") {
            expect_user = false;
        } else if !expect_user && trimmed_line.starts_with("assistant:") {
            expect_user = true;
        } else {
            return false;
        }

        // Skip content until next "user:" or "assistant:"
        while let Some(next_line) = lines.peek() {
            if next_line.trim().starts_with("user:") || next_line.trim().starts_with("assistant:") {
                break;
            }
            lines.next();
        }
    }

    true
}

/// Checks if the last non-empty entry starts with 'user:'.
fn is_last_entry_user(content: &str) -> bool {
    content.trim_end().lines()
        .rev()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .map_or(false, |line| line.starts_with("user:"))
}

/// Validates that the provided path exists and is accessible.
pub fn is_valid_path(path: &str) -> bool {
    Path::new(path).exists()
}

/// Defines the lint command for the CLI application.
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