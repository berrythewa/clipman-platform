use std::path::PathBuf;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self},
    prelude::*,
};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::time::UtcTime;
// use tracing_subscriber::fmt::time::SystemTime;

pub fn setup_logger() {
    // Create log directory if it doesn't exist
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");
    }

    // Set up file appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "clipman.log",
    );

    // Create formatting layer for file
    let file_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(file_appender)
        .with_timer(UtcTime::new(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
        )));

    // Create formatting layer for console with colors
    let console_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true)
        .pretty();

    // Set up filter from RUST_LOG env var, defaulting to 'info'
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Combine layers and install subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!("Logger initialized successfully");
}