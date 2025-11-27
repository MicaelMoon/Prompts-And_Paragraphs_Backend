use rusqlite::{Connection, Result};
use std::fmt::format;

pub fn create_locations_table(database: &str) -> Result<()> {
    let conn = Connection::open(database)?;

    let sql: String = format!(
        "CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTO INCREMENT,
            general_description TEXT,
            atmosphere TEXT
        )"
    );

    conn.execute(&sql, []);

    Ok(())
}
