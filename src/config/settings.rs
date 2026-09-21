// Settings management

use crate::config::types::Config;
use std::fs;

#[cfg(feature = "postgres")]
fn get_database() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let user = std::env::var("POSTGRES_USERNAME").unwrap_or_else(|_| "ratifact".to_string());
        let password =
            std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
        format!("postgres://{}:{}@localhost:25851/ratifact", user, password)
    })
}

#[cfg(feature = "sqlite")]
fn get_database() -> String {
    std::env::var("DATABASE_URL").unwrap_or("sqlite://./ratifact.db?mode=rwc".to_string())
}

pub fn load_config() -> Config {
    dotenvy::dotenv().ok(); // Load .env file
    let database_url = get_database();

    let debug_logs_enabled = std::env::var("DEBUG_LOGS_ENABLED")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    // Try to load from src/config/config.toml
    if let Ok(content) = fs::read_to_string("src/config/config.toml")
        && let Ok(mut config) = toml::from_str::<Config>(&content)
    {
        // Override database_url and debug_logs_enabled from env
        config.database_url = database_url;
        config.debug_logs_enabled = debug_logs_enabled;
        return config;
    }

    Config {
        database_url,
        scan_paths: vec![".".to_string()],
        retention_days: 30,
        debug_logs_enabled,
        excluded_paths: vec![],
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Create a copy without database_url for saving
    let save_config = Config {
        database_url: "".to_string(), // Not saving db url
        scan_paths: config.scan_paths.clone(),
        retention_days: config.retention_days,
        debug_logs_enabled: config.debug_logs_enabled,
        excluded_paths: config.excluded_paths.clone(),
    };
    let toml_string = toml::to_string(&save_config)?;
    fs::write("src/config/config.toml", toml_string)?;
    Ok(())
}
