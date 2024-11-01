use log::{debug, info};
use reqwest::Client;
use serde_json::{json, to_string_pretty, Value};

/// Queries the OpenAI API with the provided API key, model, and messages, returning the answer and response.
pub async fn query_openai(
    api_key: &str,
    model: &str,
    api_endpoint: &str,
    messages: Vec<Value>,
) -> Result<(String, Value), Box<dyn std::error::Error>> {
    let client = Client::new();

    info!("Sending request to OpenAI API using model: {}", model);
    debug!(
        "Request payload:\n{}",
        to_string_pretty(&json!({
            "model": model,
            "messages": messages
        }))?
    );

    let response = client
        .post(api_endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": messages
        }))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(format!(
            "OpenAI API error (status {}): {}",
            status, response_text
        ).into());
    }

    let response_body: Value = serde_json::from_str(&response_text)?;

    debug!("Received response:\n{}", to_string_pretty(&response_body)?);

    let answer = response_body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract answer from API response")?
        .to_string();

    info!("Successfully received and parsed answer from OpenAI API");

    Ok((answer, response_body))
}
