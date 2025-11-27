use crate::ollama_types::{OllamaChatRequest, OllamaMessage, OllamaResponse};
use reqwest::Client;

pub async fn send_message(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://localhost:11434/api/chat";

    let mut messages = vec![
        OllamaMessage {
            role: "system".into(),
            content: "You are a helpful assistant".into(),
        },
        OllamaMessage {
            role: "user".into(),
            content: "What time is it?".into(),
        },
        OllamaMessage {
            role: "assistant".into(),
            content: "It's 9:00pm GMT".into(),
        },
    ];

    let new_message = OllamaMessage {
        role: "user".into(),
        content: prompt.into(),
    };

    messages.push(new_message);

    let request_body = OllamaChatRequest {
        model: "mistral".into(),
        messages: messages,
        stream: false,
    };

    let res = client
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    let answer = res
        .message
        .map(|m| m.content)
        .unwrap_or("EMPTY RESPONSE".into());

    Ok(answer)
}
