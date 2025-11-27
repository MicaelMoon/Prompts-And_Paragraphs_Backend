use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use std::fmt::format;

pub struct OllamaMessage {
    pub id: Option<i32>,
    pub campaign_id: i32,
    pub role: String,
    pub content: String,
    pub time_stamp: String,
}

pub fn create_ollama_message_table(database: &str) -> Result<()> {
    let conn = Connection::open(database)?;

    let sql: String = format!(
        "CREATE TABLE IF NOT EXISTS ollama_messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_id INTEGER REFERENCES campaigns(id),
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            time_stamp TEXT
        )",
    );

    conn.execute(&sql, [])?;

    Ok(())
}

pub fn prepare_and_insert_message(database: &str, campaign_id: i32, role: &str, content: &str) {
    let new_db_message = OllamaMessage {
        id: None,
        campaign_id: campaign_id,
        role: role.to_string(),
        content: content.to_string(),
        time_stamp: Utc::now().to_rfc3339(),
    };

    insert_ollama_message(database, new_db_message);
}

pub fn insert_ollama_message(database: &str, ollama_message: OllamaMessage) -> Result<()> {
    let conn = Connection::open(database)?;
    let timestamp = Utc::now().to_rfc3339();

    let sql: String = format!(
        "INSERT INTO ollama_messages (campaign_id, role, content, time_stamp)
        VALUES({}, \'{}\', \'{}\',\'{}\')",
        ollama_message.campaign_id,
        ollama_message.role.to_string(),
        ollama_message.content.to_string().replace('\'', "''"),
        timestamp
    );

    match conn.execute(&sql, []) {
        Ok(_) => {}
        Err(e) => eprintln!("SQL Eerror: {}", e),
    };

    Ok(())
}

pub fn get_ollama_message_filter_by_campaign(
    database: &str,
    campaign_id: i32,
) -> Result<Vec<OllamaMessage>> {
    let conn = Connection::open(database)?;

    let sql: String = format!(
        "SELECT *
        FROM ollama_messages
        WHERE campaign_id = {}
        ORDER BY time_stamp ASC",
        campaign_id
    );

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        Ok(OllamaMessage {
            id: row.get(0)?,
            campaign_id: row.get(1)?,
            role: row.get(2)?,
            content: row.get(3)?,
            time_stamp: row.get(4)?,
        })
    })?;

    let results: Vec<OllamaMessage> = rows.collect::<Result<Vec<_>>>()?;

    Ok((results))
}

fn add_character() {}

fn update_character() {}
