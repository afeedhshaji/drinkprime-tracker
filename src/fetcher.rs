use reqwest;
use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::Config;

pub async fn fetch_and_store(config: &Config) {
    if let Some(dispensed) = fetch_data(&config.filter_ip).await {
        if let Err(e) = store_data(&config.db_file, dispensed) {
            eprintln!("Error storing data: {}", e);
        }
    }
}

async fn fetch_data(filter_ip: &str) -> Option<i32> {
    let url = format!("http://{}/getValidity", filter_ip);
    match reqwest::get(&url).await {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                json["dispensed"].as_i64().map(|v| v as i32)
            } else {
                eprintln!("Error parsing JSON response");
                None
            }
        }
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
            None
        }
    }
}

fn store_data(db_file: &str, dispensed: i32) -> Result<()> {
    let conn = Connection::open(db_file)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usage (timestamp INTEGER, dispensed INTEGER)",
        [],
    )?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    conn.execute(
        "INSERT INTO usage (timestamp, dispensed) VALUES (?, ?)",
        params![timestamp, dispensed],
    )?;
    Ok(())
}
