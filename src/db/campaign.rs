use rusqlite::{Connection, Result};
use std::fmt::format;

pub fn create_campaign_table(database: &str) -> Result<()> {
    let conn = Connection::open(database)?;

    let sql = format!(
        "CREATE TABLE IF NOT EXISTS campaigns (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            in_game_days INTEGER NOT NULL
        )"
    );

    conn.execute(&sql, ());

    Ok(())
}

pub fn insert_campaign(database: &str) -> Result<()> {
    let conn = Connection::open(database)?;

    let sql = format!(
        "INSERT INTO campaigns(in_game_days)
        VALUES(1)"
    );

    match conn.execute(&sql, []) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    };

    Ok(())
}
