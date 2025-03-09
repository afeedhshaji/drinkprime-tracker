use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub db_file: String,
    pub filter_ip: String,
    pub discord_webhook: String,
}

pub fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}
