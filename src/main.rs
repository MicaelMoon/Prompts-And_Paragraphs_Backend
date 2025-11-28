mod agents;
mod db;
mod ollama_types;

use agents::generator_agent;
use chrono::{DateTime, Utc};
use db::campaign;
use db::character;
use db::ollama_message::OllamaMessage;
use rusqlite::{Connection, Result, types::Null};
use serde::Deserialize;
use std::fs;

use crate::db::campaign::create_campaign_table;
use crate::db::campaign::insert_campaign;
use crate::db::ollama_message;

#[derive(Deserialize)]
struct PlayerStats {
    max_health: i32,
    current_heaelth: i32,
    max_attack: i32,
    current_attack: i32,
    max_defense: i32,
    current_defense: i32,
}

#[derive(Deserialize)]
struct Response {
    player_stats: PlayerStats,
    entities: serde_json::Value,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send_message("What did I ask you now again?").await;

    let prompt = "I follow the path in the woods";

    create_ollama_message_debug();

    send_prompt(prompt).await;

    Ok(())
}

async fn send_prompt(prompt: &str) -> Result<String, String> {
    let mut json_response: String = "".to_string();

    match generator_agent::send_message(prompt).await {
        Ok(response) => json_response = response,
        Err(e) => eprintln!("Error sending message: {}", e),
    };

    let parsed: Response = serde_json::from_str(&json_response).unwrap();

    println!("Ollama: {}", parsed.message);

    Ok(json_response)
}

fn create_ollama_message_debug() {
    //placeholder content
    let database = "app.db";
    create_campaign_table(database);
    insert_campaign(database);

    ollama_message::create_ollama_message_table(database);

    let mut assistant_message = OllamaMessage {
        id: None,
        campaign_id: 1,
        role: "assistant".to_string(),
        content: "You are on a path in the woods, and at the end of that path there is a fort and in the basement of that fort there is a princess.
        You are on your way to save her.".to_string(),
        time_stamp: Utc::now().to_string(),
    };

    ollama_message::insert_ollama_message(database, assistant_message);
}

async fn create_new_campaign() {
    character::create_characters_table("campaign", 1);
    println!("Table created")
}
