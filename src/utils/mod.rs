// Utility modules

pub mod helpers;
pub mod logger;

// Re-export commonly used functions
pub use helpers::{calculate_dir_size, detect_language_for_path};
