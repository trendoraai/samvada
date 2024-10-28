use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{BufRead, BufReader, Write, Error as IoError};
use std::path::{Path, PathBuf};
use clap::{Arg, ArgMatches, Command};
use reqwest::Client;
use serde_json::{json, Value, to_string_pretty};
use dotenv::dotenv;
use log::{info, error, debug};
use simplelog::{
    ConfigBuilder, LevelFilter, WriteLogger, TermLogger, TerminalMode, CombinedLogger,
    LevelPadding, ThreadLogMode,
};
use simplelog::*;
use chrono::{DateTime, Local, TimeZone};
use time::macros::format_description;
use time::UtcOffset;


pub async fn handle_ask_subcommand(matches: &ArgMatches) {
    dotenv().ok(); // Load environment variables from .env file

    let file_path = matches.get_one::<String>("file").unwrap();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    // Setup logging
    let _log_path = setup_logging(file_path);
    info!("Starting processing for file: {}", file_path);

    match process_file_and_query_openai(file_path, &api_key).await {
        Ok((answer, response_body)) => {
            println!("Answer: {}", answer);
            info!("Successfully processed file and received answer");
            
            // Append the answer to the markdown file
            if let Err(e) = append_answer_to_file(file_path, &answer, &response_body) {
                error!("Failed to append answer to file: {}", e);
                eprintln!("Failed to append answer to file: {}", e);
            } else {
                info!("Successfully appended answer to file");
            }
        },
        Err(e) => {
            error!("Error processing file and querying OpenAI: {}", e);
            eprintln!("Error processing file and querying OpenAI: {}", e);
            std::process::exit(1);
        }
    }
}

fn append_answer_to_file(file_path: &str, answer: &str, response_body: &Value) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "\n\nassistant:")?;
    writeln!(file, "{}\n\n", answer)?;
    
    // Extract and format the required information
    let created = response_body["created"].as_i64().unwrap_or_default();
    let created_datetime: DateTime<Local> = Local.timestamp_opt(created, 0)
        .single()
        .unwrap_or_else(|| Local::now());
    let created_formatted = created_datetime.format("%Y-%m-%d %H:%M:%S %:z").to_string(); // Include timezone
    
    let id = response_body["id"].as_str().unwrap_or_default();
    let model = response_body["model"].as_str().unwrap_or_default();
    let total_tokens = response_body["usage"]["total_tokens"].as_i64().unwrap_or_default();

    // Write the formatted comments
    writeln!(file, "<!-- model_name: {} -->", model)?;
    writeln!(file, "<!-- id: {} -->", id)?;
    writeln!(file, "<!-- created: {} -->", created_formatted)?;
    writeln!(file, "<!-- total_tokens: {} -->", total_tokens)?;

    writeln!(file, "\n\nuser:")?;

    Ok(())
}


fn setup_logging(file_path: &str) -> PathBuf {
    let path = Path::new(file_path);
    let stem = path.file_stem().unwrap_or_default();
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let log_path = parent.join(format!("{}.log", stem.to_str().unwrap()));

    // Get the local time offset using chrono
    let offset_in_sec = Local::now().offset().local_minus_utc();
    eprintln!("Detected local offset: {} seconds", offset_in_sec);

    // Convert chrono offset to time::UtcOffset
    let local_offset = UtcOffset::from_whole_seconds(offset_in_sec)
        .unwrap_or_else(|_| {
            eprintln!("Invalid offset: {}. Defaulting to UTC", offset_in_sec);
            UtcOffset::UTC
        });

    eprintln!("Using UTC offset: {:?}", local_offset);

    // Customize the logging configuration
    let config = ConfigBuilder::new()
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Right)
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3] [offset_hour sign:mandatory]:[offset_minute]"
        ))
        .set_time_offset(local_offset)
        .build();

    // Initialize both WriteLogger and TermLogger
    CombinedLogger::init(
        vec![
            WriteLogger::new(
                LevelFilter::Debug,
                config.clone(),
                File::create(&log_path).expect("Failed to create log file"),
            ),
            TermLogger::new(
                LevelFilter::Info,
                config,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
        ]
    ).expect("Failed to initialize loggers");

    log_path
}

async fn process_file_and_query_openai(file_path: &str, api_key: &str) -> Result<(String, Value), Box<dyn std::error::Error>> {
    let (system_prompt, model, messages) = parse_file(file_path)?;
    let api_messages = prepare_api_messages(&system_prompt, &messages);
    
    debug!("Prepared API messages:\n{}", to_string_pretty(&api_messages)?);
    
    query_openai(api_key, &model, api_messages).await
}

fn parse_file(file_path: &str) -> Result<(String, String, Vec<(String, String)>), std::io::Error> {
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

fn parse_frontmatter<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<(String, String), std::io::Error> {
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

fn parse_messages<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<Vec<(String, String)>, IoError> {
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

fn is_new_message(line: &str) -> bool {
    line.starts_with("user:") || line.starts_with("assistant:")
}

fn finish_current_message(messages: &mut Vec<(String, String)>, role: &mut String, content: &mut String) {
    if !role.is_empty() {
        messages.push((role.clone(), content.trim().to_string()));
        content.clear();
    }
}

fn start_new_message(line: &str, role: &mut String, content: &mut String) {
    *role = if line.starts_with("user:") { "user" } else { "assistant" }.to_string();
    *content = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
}

fn process_message_line(line: &str, role: &str, content: &mut String) -> Result<(), IoError> {
    match role {
        "user" => process_user_line(line, content),
        "assistant" => process_assistant_line(line, content),
        _ => Ok(()),
    }
}

fn process_user_line(line: &str, content: &mut String) -> Result<(), IoError> {
    if is_file_reference(line) {
        expand_file_reference(line, content)?;
    } else if !line.starts_with("<c>") {
        append_line(content, line);
    }
    Ok(())
}

fn process_assistant_line(line: &str, content: &mut String) -> Result<(), IoError> {
    if !line.trim().starts_with("<!--") && !line.trim().ends_with("-->") {
        append_line(content, line);
    }
    Ok(())
}

fn is_file_reference(line: &str) -> bool {
    line.trim().starts_with("[[") && line.trim().ends_with("]]") && !line.contains('\n')
}

fn expand_file_reference(line: &str, content: &mut String) -> Result<(), IoError> {
    let file_path = line.trim().trim_start_matches("[[").trim_end_matches("]]");
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            content.push_str(&format!("\n\n[[{}]]\n\n{}\n\n", file_path, file_content));
            Ok(())
        },
        Err(e) => {
            content.push_str(&format!("\n\nFailed to read file: {}\n\n", file_path));
            Err(e)
        }
    }
}

fn append_line(content: &mut String, line: &str) {
    if !content.is_empty() {
        content.push('\n');
    }
    content.push_str(line.trim());
}

fn prepare_api_messages(system_prompt: &str, messages: &[(String, String)]) -> Vec<Value> {
    let mut api_messages = vec![
        json!({"role": "system", "content": system_prompt}),
    ];

    for (role, content) in messages {
        api_messages.push(json!({"role": role, "content": content}));
    }

    debug!("Prepared {} API messages", api_messages.len());
    api_messages
}

async fn query_openai(api_key: &str, model: &str, messages: Vec<Value>) -> Result<(String, Value), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    info!("Sending request to OpenAI API using model: {}", model);
    debug!("Request payload:\n{}", to_string_pretty(&json!({
        "model": model,
        "messages": messages
    }))?);

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": messages
        }))
        .send()
        .await?;

    let response_body: Value = response.json().await?;
    
    debug!("Received response:\n{}", to_string_pretty(&response_body)?);

    let answer = response_body["choices"][0]["message"]["content"].as_str()
        .ok_or("Failed to extract answer from API response")?
        .to_string();

    info!("Successfully received and parsed answer from OpenAI API");

    Ok((answer, response_body))
}

pub fn ask_command() -> Command {
    Command::new("ask")
        .about("Process a chat file and query OpenAI")
        .arg(
            Arg::new("file")
                .help("Path to the chat file")
                .required(true)
                .num_args(1),
        )
}
