use clap::{Arg, ArgMatches, Command};
use log::{debug, error, info};
use serde_json::Value;

use crate::chat::api::query_openai;
use crate::chat::config::{get_api_key, get_env_file_path, save_api_key};
use crate::chat::logging::setup_logging;
use crate::chat::parser::{parse_file, prepare_api_messages};
use crate::chat::_utils::handle_openai_response;

/// Handles the 'ask' subcommand, processing the file and querying OpenAI
pub async fn handle_ask_subcommand(matches: &ArgMatches) {
    let file_path = matches.get_one::<String>("file").unwrap();

    // Setup logging FIRST
    let _log_path = setup_logging(Some(file_path));
    info!("Starting processing for file: {}", file_path);

    // If API key is provided as argument, save it
    if let Some(api_key) = matches.get_one::<String>("api-key") {
        if let Err(e) = save_api_key(api_key) {
            eprintln!("Failed to save API key: {}", e);
            std::process::exit(1);
        }
    }

    // Load environment variables from the config directory
    if let Ok(env_path) = get_env_file_path() {
        let absolute_path = env_path.canonicalize().unwrap_or_else(|_| env_path.clone());
        debug!(
            "Loading environment from absolute path: {}",
            absolute_path.display()
        );
        dotenv::from_path(env_path).ok();
    }

    let api_key = get_api_key(matches.get_one::<String>("api-key"));

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
    let (system_prompt, model, api_endpoint, messages) = parse_file(file_path)?;
    let api_messages = prepare_api_messages(&system_prompt, &messages);

    debug!(
        "Prepared API messages:\n{}",
        serde_json::to_string_pretty(&api_messages)?
    );

    query_openai(api_key, &model, &api_endpoint, api_messages).await
}

/// Appends the answer and metadata to the specified file
fn append_answer_to_file(
    file_path: &str,
    answer: &str,
    response_body: &Value,
) -> std::io::Result<()> {
    handle_openai_response(file_path, None, answer, response_body)
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
