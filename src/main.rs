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
struct Player {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send_message("What did I ask you now again?").await;
    //get_player();

    //create_ollama_message_debug();

    send_prompt(
        "I look around in search for a path where I can sneak into the center building. Alternativly I look for cellar leading underground of somee kind.",
    ).await;

    Ok(())
}

async fn send_prompt(prompt: &str) -> Result<String, String> {
    let mut message: String = "".to_string();

    match generator_agent::send_message(prompt).await {
        Ok(message) => println!("Ollama: {}", message),
        Err(e) => eprintln!("Error sending message: {}", e),
    };

    Ok(message)
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

    let user_message = OllamaMessage {
        id: None,
        campaign_id: 1,
        role: "user".to_string(),
        content: "I follow the path untill I can see the fort in the distans. I then hide behind a bush to scout out the area".to_string(),
        time_stamp: Utc::now().to_string(),
    };

    ollama_message::insert_ollama_message(database, user_message);

    assistant_message = OllamaMessage {
        id: None,
        campaign_id: 1,
        role: "assistant".to_string(),
        content: "From your view you can see a massive fort the sice of a village. Headlights shooting out in every directions moving left and right.
        Towers rise dozens of meeters up with sharpshooters. There is no obviouse path for you to sneak your way in from this angle.".to_string(),
        time_stamp: Utc::now().to_string(),
    };

    ollama_message::insert_ollama_message(database, assistant_message);
}

fn get_player() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("config/player.json")?;

    let player: Player = serde_json::from_str(&data)?;

    println!("Player name: {}", player.name);

    Ok(())
}

async fn create_new_campaign() {
    character::create_characters_table("campaign", 1);
    println!("Table created")
}
