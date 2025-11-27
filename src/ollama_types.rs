use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub message: Option<OllamaMessage>,
}
