// Config tests

use crate::config::settings::{load_config, save_config};
use crate::config::types::Config;
use std::env;
use std::path::Path;
use std::sync::Mutex;

/// Env-var mutation and `src/config/config.toml` writes are process-global;
/// serialize every test that touches either so they can't race each other.
/// `into_inner` keeps a panicking test from poisoning the rest of the suite.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn test_load_config() {
    let _guard = lock_env();
    unsafe {
        env::remove_var("DATABASE_URL");
    }
    let config = load_config();
    #[cfg(feature = "postgres")]
    assert!(config.database_url.contains("postgres"));
    #[cfg(feature = "sqlite")]
    assert!(config.database_url.contains("sqlite"));
    // scan_paths will be loaded from src/config/config.toml which contains ["/srv"]
    assert!(!config.scan_paths.is_empty(), "scan_paths should not be empty");
    // retention_days is loaded from src/config/config.toml
    assert!(
        config.retention_days > 0,
        "retention_days should be greater than 0"
    );
}

#[test]
fn test_load_config_with_env() {
    let _guard = lock_env();
    let previous = env::var("DATABASE_URL").ok();
    unsafe {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("POSTGRES_USERNAME", "testuser");
    }
    let config = load_config();
    assert_eq!(config.database_url, "postgres://test:test@localhost/test");
    unsafe {
        match previous {
            Some(value) => env::set_var("DATABASE_URL", value),
            None => env::remove_var("DATABASE_URL"),
        }
    }
}

#[test]
fn test_load_config_from_src_config_toml() {
    let _guard = lock_env();
    // Test that config can be loaded from src/config/config.toml
    let config_path = "src/config/config.toml";
    assert!(
        Path::new(config_path).exists(),
        "config.toml should exist at {}",
        config_path
    );

    // Load the config
    let config = load_config();

    // Verify it has expected values from src/config/config.toml
    assert!(!config.scan_paths.is_empty(), "scan_paths should be loaded");
    assert!(
        config.retention_days > 0,
        "retention_days should be loaded"
    );
}

#[test]
fn test_save_and_load_config() {
    let _guard = lock_env();
    // save_config writes the tracked src/config/config.toml; snapshot it and
    // restore afterwards so the suite never dirties the working tree.
    let config_path = Path::new("src/config/config.toml");
    let backup = std::fs::read(config_path).ok();

    // Create a test config
    let test_config = Config {
        database_url: "postgres://test:test@localhost/testdb".to_string(),
        scan_paths: vec!["/srv".to_string()],
        retention_days: 30,
        debug_logs_enabled: false,
        excluded_paths: vec![],
    };

    // Save the config
    let result = save_config(&test_config);
    assert!(result.is_ok(), "save_config should succeed");

    // Verify the file exists at the new location
    assert!(
        config_path.exists(),
        "config.toml should exist at src/config/config.toml"
    );

    // Load and verify
    let loaded_config = load_config();
    assert_eq!(
        loaded_config.scan_paths, test_config.scan_paths,
        "scan_paths should match after save/load"
    );
    assert_eq!(
        loaded_config.retention_days, test_config.retention_days,
        "retention_days should match after save/load"
    );
    assert_eq!(
        loaded_config.excluded_paths, test_config.excluded_paths,
        "excluded_paths should match after save/load"
    );

    if let Some(bytes) = backup {
        std::fs::write(config_path, bytes).unwrap();
    }
}
