use atty::Stream;
use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use log::{error, info};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use crate::chat::api::query_openai;
use crate::chat::config::load_config;
use crate::chat::config::{get_api_key, get_env_file_path, save_api_key};
use crate::chat::create::create_chat;
use crate::chat::logging::setup_logging;
use crate::chat::parser::prepare_api_messages;

/// Handles the quick subcommand by saving API key, loading environment variables, processing the question, and querying OpenAI.
pub async fn handle_quick_subcommand(matches: &ArgMatches) {
    // Setup logging FIRST (similar to ask.rs)
    let _log_path = setup_logging(None);
    info!("Starting processing for quick question");

    // Load configurations from the YAML file
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

    let api_key = get_api_key(matches.get_one::<String>("api-key"));

    // Read question from argument or stdin
    let question = if let Some(question_arg) = matches.get_one::<String>("question") {
        question_arg.clone()
    } else if !atty::is(Stream::Stdin) {
        // Read from stdin if stdin is not connected to a terminal (i.e., input is being piped)
        let mut stdin_input = String::new();
        io::stdin()
            .read_to_string(&mut stdin_input)
            .expect("Failed to read from stdin");
        stdin_input.trim().to_string()
    } else {
        eprintln!("No question provided. Please provide a question as an argument or via stdin.");
        std::process::exit(1);
    };

    // Ensure that the question is not empty
    if question.is_empty() {
        eprintln!("No question provided. Please provide a question as an argument or via stdin.");
        std::process::exit(1);
    }

    match process_question_and_query_openai(
        &question,
        &api_key,
        &system_prompt,
        &model,
        &api_endpoint,
    )
    .await
    {
        Ok((answer, response_body)) => {
            println!("\n{}\n", answer);
            info!("Successfully processed question and received answer");

            // Handle logging and saving if required
            if matches.get_flag("save-to-markdown") {
                if let Err(e) = save_conversation_to_markdown(&question, &answer, &response_body) {
                    error!("Failed to save conversation to markdown: {}", e);
                    eprintln!("Failed to save conversation to markdown: {}", e);
                } else {
                    info!("Successfully saved conversation to markdown");
                }
            }
        }
        Err(e) => {
            error!("Error processing question and querying OpenAI: {}", e);
            eprintln!("Error processing question and querying OpenAI: {}", e);
            std::process::exit(1);
        }
    }
}

/// Processes the provided question and queries OpenAI, returning the answer and response body.
async fn process_question_and_query_openai(
    question: &str,
    api_key: &str,
    system_prompt: &str,
    model: &str,
    api_endpoint: &str,
) -> Result<(String, Value), Box<dyn std::error::Error>> {
    let messages = vec![("user".to_string(), question.to_string())];
    let api_messages = prepare_api_messages(system_prompt, &messages);

    query_openai(api_key, model, api_endpoint, api_messages).await
}

/// Saves the conversation between user and assistant to a markdown file with necessary metadata.
fn save_conversation_to_markdown(
    question: &str,
    answer: &str,
    response_body: &Value,
) -> std::io::Result<()> {
    let timestamp = Local::now();
    let file_name = format!("conversation_{}", timestamp.format("%Y%m%d_%H%M%S"));

    // First create the file with proper frontmatter using create_chat
    create_chat(&file_name, None)?;

    // Now append the conversation
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(format!("{}.md", file_name));

    let mut file = OpenOptions::new()
        .append(true)
        .open(&file_path)?;

    // Append conversation
    writeln!(file, "\nuser:\n{}\n", question)?;
    writeln!(file, "assistant:\n{}\n", answer)?;

    // Add metadata as comments
    let id = response_body["id"].as_str().unwrap_or_default();
    let total_tokens = response_body["usage"]["total_tokens"]
        .as_i64()
        .unwrap_or_default();

    writeln!(file, "<!-- id: {} -->", id)?;
    writeln!(file, "<!-- total_tokens: {} -->", total_tokens)?;

    println!("\nSaving conversation to: {}", file_path.display());

    Ok(())
}

/// Defines the 'quick' command for asking a question to OpenAI with options to save the conversation to markdown.
pub fn quick_command() -> Command {
    Command::new("quick")
        .about("Quickly ask a question to OpenAI")
        .arg(
            Arg::new("question")
                .help("The question to ask")
                .required(false) // Make it optional
                .num_args(1),
        )
        .arg(
            Arg::new("api-key")
                .long("api-key")
                .help("Set your OpenAI API key (will be saved for future use)")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("save-to-markdown")
                .long("save-to-markdown")
                .help("Save the conversation to a markdown file")
                .action(clap::ArgAction::SetTrue),
        )
}