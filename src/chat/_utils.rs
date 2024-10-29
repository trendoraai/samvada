use chrono::{DateTime, Local, TimeZone};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;

/// Common metadata structure for OpenAI responses
pub struct ResponseMetadata {
    pub model: String,
    pub id: String,
    pub created_formatted: String,
    pub total_tokens: i64,
}

impl ResponseMetadata {
    pub fn from_response(response_body: &Value) -> Self {
        let created = response_body["created"].as_i64().unwrap_or_default();
        let created_datetime: DateTime<Local> = Local
            .timestamp_opt(created, 0)
            .single()
            .unwrap_or_else(|| Local::now());
        
        Self {
            model: response_body["model"].as_str().unwrap_or_default().to_string(),
            id: response_body["id"].as_str().unwrap_or_default().to_string(),
            created_formatted: created_datetime.format("%Y-%m-%d %H:%M:%S %:z").to_string(),
            total_tokens: response_body["usage"]["total_tokens"].as_i64().unwrap_or_default(),
        }
    }
}

/// Writes metadata comments to a file
pub fn write_metadata(file: &mut std::fs::File, metadata: &ResponseMetadata) -> std::io::Result<()> {
    writeln!(file, "<!-- model: {} -->", metadata.model)?;
    writeln!(file, "<!-- id: {} -->", metadata.id)?;
    writeln!(file, "<!-- created: {} -->", metadata.created_formatted)?;
    writeln!(file, "<!-- total_tokens: {} -->", metadata.total_tokens)?;
    Ok(())
}

/// Common function to handle OpenAI response and write to file
pub fn handle_openai_response(
    file_path: &str,
    question: Option<&str>,
    answer: &str,
    response_body: &Value,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    if let Some(q) = question {
        writeln!(file, "\nuser:\n{}\n", q)?;
    }
    
    writeln!(file, "assistant:\n{}\n", answer)?;

    let metadata = ResponseMetadata::from_response(response_body);
    write_metadata(&mut file, &metadata)?;

    if question.is_none() {
        writeln!(file, "\nuser:")?;
    }

    Ok(())
}