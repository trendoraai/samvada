use log::{debug, info};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::fs;

/// Parses a file to extract system prompt, model, and messages.
pub fn parse_file(file_path: &str) -> Result<(String, String, Vec<(String, String)>), std::io::Error> {
    info!("Parsing file: {}", file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let (system_prompt, model) = parse_frontmatter(&mut lines)?;
    let messages = parse_messages(&mut lines)?;

    debug!("Parsed system prompt: {}", system_prompt);
    debug!("Using model: {}", model);
    debug!("Parsed {} messages", messages.len());

    Ok((system_prompt, model, messages))
}

/// Parses the frontmatter section of the file to extract system prompt and model.
fn parse_frontmatter<B: BufRead>(
    lines: &mut std::io::Lines<B>,
) -> Result<(String, String), std::io::Error> {
    let mut system_prompt = String::new();
    let mut model = String::from("gpt-3.5-turbo"); // default model
    let mut in_frontmatter = false;
    let mut reading_system = false;

    for line in lines {
        let line = line?;
        if line.trim() == "---" {
            if in_frontmatter {
                break; // End of frontmatter
            }
            in_frontmatter = true;
            continue;
        }

        if in_frontmatter {
            if line.starts_with("system:") {
                reading_system = true;
                system_prompt = line.trim_start_matches("system:").trim().to_string();
            } else if line.starts_with("model:") {
                model = line.trim_start_matches("model:").trim().to_string();
                reading_system = false;
            } else if reading_system && !line.contains(':') {
                system_prompt.push_str("\n");
                system_prompt.push_str(line.trim());
            } else if line.contains(':') {
                reading_system = false;
            }
        }
    }

    Ok((system_prompt, model))
}

/// Parses the messages section of the file to extract role and content of each message.
fn parse_messages<B: BufRead>(
    lines: &mut std::io::Lines<B>,
) -> Result<Vec<(String, String)>, IoError> {
    let mut messages = Vec::new();
    let mut current_role = String::new();
    let mut current_content = String::new();

    for line in lines {
        let line = line?;
        if is_new_message(&line) {
            finish_current_message(&mut messages, &mut current_role, &mut current_content);
            start_new_message(&line, &mut current_role, &mut current_content);
        } else if !line.is_empty() {
            process_message_line(&line, &current_role, &mut current_content)?;
        }
    }

    finish_current_message(&mut messages, &mut current_role, &mut current_content);
    Ok(messages)
}

/// Checks if a line indicates the start of a new message.
fn is_new_message(line: &str) -> bool {
    line.starts_with("user:") || line.starts_with("assistant:")
}

/// Finalizes the current message being processed.
fn finish_current_message(
    messages: &mut Vec<(String, String)>,
    role: &mut String,
    content: &mut String,
) {
    if !role.is_empty() {
        messages.push((role.clone(), content.trim().to_string()));
        content.clear();
    }
}

/// Starts a new message based on the provided line.
fn start_new_message(line: &str, role: &mut String, content: &mut String) {
    *role = if line.starts_with("user:") {
        "user"
    } else {
        "assistant"
    }
    .to_string();
    *content = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
}

/// Processes a line of a message based on the role.
fn process_message_line(line: &str, role: &str, content: &mut String) -> Result<(), IoError> {
    match role {
        "user" => process_user_line(line, content),
        "assistant" => process_assistant_line(line, content),
        _ => Ok(()),
    }
}

/// Processes a line of a user message.
fn process_user_line(line: &str, content: &mut String) -> Result<(), IoError> {
    if is_file_reference(line) {
        expand_file_reference(line, content)?;
    } else if !line.starts_with("<c>") {
        append_line(content, line);
    }
    Ok(())
}

/// Processes a line of an assistant message.
fn process_assistant_line(line: &str, content: &mut String) -> Result<(), IoError> {
    if !line.trim().starts_with("<!--") && !line.trim().ends_with("-->") {
        append_line(content, line);
    }
    Ok(())
}

/// Checks if a line contains a file reference.
fn is_file_reference(line: &str) -> bool {
    line.trim().starts_with("[[") && line.trim().ends_with("]]") && !line.contains('\n')
}

/// Expands a file reference line to include the content of the referenced file.
fn expand_file_reference(line: &str, content: &mut String) -> Result<(), IoError> {
    let file_path = line.trim().trim_start_matches("[[").trim_end_matches("]]");
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            content.push_str(&format!("\n\n[[{}]]\n\n{}\n\n", file_path, file_content));
            Ok(())
        }
        Err(e) => {
            content.push_str(&format!("\n\nFailed to read file: {}\n\n", file_path));
            Err(e)
        }
    }
}

/// Appends a line to the content, handling newlines.
fn append_line(content: &mut String, line: &str) {
    if !content.is_empty() {
        content.push('\n');
    }
    content.push_str(line.trim());
}

/// Prepares API messages from the system prompt and messages.
pub fn prepare_api_messages(system_prompt: &str, messages: &[(String, String)]) -> Vec<Value> {
    let mut api_messages = vec![json!({"role": "system", "content": system_prompt})];

    for (role, content) in messages {
        api_messages.push(json!({"role": role, "content": content}));
    }

    debug!("Prepared {} API messages", api_messages.len());
    api_messages
}