use crate::db::ollama_message;
use crate::ollama_types::{OllamaChatRequest, OllamaMessage, OllamaResponse};
use chrono::{DateTime, Utc};
use reqwest::Client;
use std::fs;
use std::ptr::null;

pub async fn send_message(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://localhost:11434/api/chat";
    let system_content = fs::read_to_string("generator_model_system_content.txt").expect("");
    let time_stamp = Utc::now().to_rfc3339();

    let new_message = OllamaMessage {
        role: "user".into(),
        content: prompt.into(),
    };

    ollama_message::prepare_and_insert_message("app.db", 1, "user", prompt);

    let mut messages: Vec<OllamaMessage> = vec![OllamaMessage {
        role: "system".into(),
        content: system_content.into(),
    }];

    messages.push(new_message);

    let db_ollama_messages =
        match ollama_message::get_ollama_message_filter_by_campaign("app.db", 1) {
            Ok(vec) => vec,
            Err(e) => {
                eprintln!("Error getting messege by campaign: {}", e);
                vec![]
            }
        };

    for message in db_ollama_messages {
        let new_ollama_message = OllamaMessage {
            role: message.role,
            content: message.content,
        };

        if new_ollama_message.role == "user" {
            println!("User: {}", new_ollama_message.content)
        } else if new_ollama_message.role == "assistant" {
            println!("Ollama: {}", new_ollama_message.content)
        }

        messages.push(new_ollama_message);
    }

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

    ollama_message::prepare_and_insert_message("app.db", 1, "assistant", &answer);

    Ok(answer)
}
