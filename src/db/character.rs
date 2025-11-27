use rusqlite::{Connection, Result};
use std::fmt::format;

pub fn create_characters_table(database: &str, campaign_index: i32) -> Result<()> {
    let db_name = ("{}_{}", database, campaign_index);
    let conn = Connection::open(database)?;

    let sql: String = format!(
        "CREATE TABLE IF NOT EXISTS characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_index INTEGER,
            name TEXT NOT NULL,
            personality TEXT
        )",
    );

    conn.execute(&sql, [])?;

    Ok(())
}

fn add_character() {}

fn update_character() {}
