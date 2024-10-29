use chrono::{DateTime, Local, TimeZone};
use clap::{Arg, ArgMatches, Command};
use log::{debug, error, info};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;

use crate::chat::api::query_openai;
use crate::chat::config::{get_env_file_path, save_api_key};
use crate::chat::logging::setup_logging;
use crate::chat::parser::{parse_file, prepare_api_messages};

/// Handles the 'ask' subcommand, processing the file and querying OpenAI
pub async fn handle_ask_subcommand(matches: &ArgMatches) {
    // If API key is provided as argument, save it
    if let Some(api_key) = matches.get_one::<String>("api-key") {
        if let Err(e) = save_api_key(api_key) {
            eprintln!("Failed to save API key: {}", e);
            std::process::exit(1);
        }
    }

    // Load environment variables from the config directory
    if let Ok(env_path) = get_env_file_path() {
        dotenv::from_path(env_path).ok();
    }

    let file_path = matches.get_one::<String>("file").unwrap();
    let api_key = std::env::var("OPENAI_API_KEY").expect(
        "OpenAI API key not found! Please set it using one of these methods:\n\
        1. Run the command with your API key:\n\
           samvada ask --api-key=your-api-key-here file.md\n\
        2. Set it as an environment variable:\n\
           - Windows (Command Prompt): set OPENAI_API_KEY=your-api-key-here\n\
           - Windows (PowerShell): $env:OPENAI_API_KEY='your-api-key-here'\n\
           - Mac/Linux: export OPENAI_API_KEY=your-api-key-here",
    );

    // Setup logging
    let _log_path = setup_logging(Some(file_path));
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
        }
        Err(e) => {
            error!("Error processing file and querying OpenAI: {}", e);
            eprintln!("Error processing file and querying OpenAI: {}", e);
            std::process::exit(1);
        }
    }
}

/// Processes the file and queries OpenAI with the extracted information
async fn process_file_and_query_openai(
    file_path: &str,
    api_key: &str,
) -> Result<(String, Value), Box<dyn std::error::Error>> {
    let (system_prompt, model, messages) = parse_file(file_path)?;
    let api_messages = prepare_api_messages(&system_prompt, &messages);

    debug!(
        "Prepared API messages:\n{}",
        serde_json::to_string_pretty(&api_messages)?
    );

    query_openai(api_key, &model, api_messages).await
}

/// Appends the answer and metadata to the specified file
fn append_answer_to_file(
    file_path: &str,
    answer: &str,
    response_body: &Value,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "\n\nassistant:")?;
    writeln!(file, "{}\n", answer)?;

    // Extract and format the required information
    let created = response_body["created"].as_i64().unwrap_or_default();
    let created_datetime: DateTime<Local> = Local
        .timestamp_opt(created, 0)
        .single()
        .unwrap_or_else(|| Local::now());
    let created_formatted = created_datetime.format("%Y-%m-%d %H:%M:%S %:z").to_string();

    let id = response_body["id"].as_str().unwrap_or_default();
    let model = response_body["model"].as_str().unwrap_or_default();
    let total_tokens = response_body["usage"]["total_tokens"]
        .as_i64()
        .unwrap_or_default();

    // Write the formatted comments
    writeln!(file, "<!-- model_name: {} -->", model)?;
    writeln!(file, "<!-- id: {} -->", id)?;
    writeln!(file, "<!-- created: {} -->", created_formatted)?;
    writeln!(file, "<!-- total_tokens: {} -->", total_tokens)?;

    writeln!(file, "\n\nuser:")?;

    Ok(())
}

/// Creates and returns the 'ask' command with its arguments
pub fn ask_command() -> Command {
    Command::new("ask")
        .about("Process a chat file and query OpenAI")
        .arg(
            Arg::new("file")
                .help("Path to the chat file")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("api-key")
                .long("api-key")
                .help("Set your OpenAI API key (will be saved for future use)")
                .required(false)
                .num_args(1),
        )
}