use crate::config::{APP_NAME, APP_VERSION};
use crate::models::log_level::LogLevel;
use chrono::Local;
use colored::*;

pub fn log(level: LogLevel, message: &str) {
    let cur_timestamp = current_timestamp();

    let level_str = match level {
        LogLevel::Info => "INFO".green().bold(),
        LogLevel::Debug => "DEBUG".blue(),
        LogLevel::Warn => "WARN".yellow().bold(),
        LogLevel::Error => "ERROR".red().bold(),
    };

    println!(
        "[{}:{} @ {}] {}: {}",
        APP_NAME.bright_magenta(),
        APP_VERSION.dimmed(),
        cur_timestamp.cyan(),
        level_str,
        message
    );
}

fn current_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
