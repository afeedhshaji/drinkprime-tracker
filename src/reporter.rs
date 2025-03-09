use reqwest;
use rusqlite::{params, Connection};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::Config;

pub async fn report_usage(config: &Config) {
    if let Some(usage) = get_daily_usage(&config.db_file) {
        if let Err(e) = send_discord_message(&config.discord_webhook, usage).await {
            eprintln!("Error sending Discord message: {}", e);
        }
    }
}

fn get_daily_usage(db_file: &str) -> Option<f64> {
    let conn = Connection::open(db_file).ok()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let yesterday = now - 86400;

    let mut stmt = conn.prepare("SELECT timestamp, dispensed FROM usage WHERE timestamp >= ? ORDER BY timestamp ASC").ok()?;
    let data: Vec<(i64, i32)> = stmt.query_map(params![yesterday], |row| Ok((row.get(0)?, row.get(1)?))).ok()?.flatten().collect();
    
    if data.len() < 2 {
        return None;
    }
    let usage = (data.last()?.1 - data.first()?.1) as f64 / 1000.0;
    Some(usage)
}

async fn send_discord_message(webhook_url: &str, usage_liters: f64) -> Result<(), reqwest::Error> {
    let message = format!("🚰 **Daily Water Usage**: {:.2} L", usage_liters);
    let payload = serde_json::json!({"content": message});
    let client = reqwest::Client::new();
    let response = client.post(webhook_url).json(&payload).send().await?;
    if response.status().is_success() {
        println!("✅ Successfully sent daily usage to Discord.");
    } else {
        eprintln!("❌ Failed to send message: {}", response.status());
    }
    Ok(())
}
