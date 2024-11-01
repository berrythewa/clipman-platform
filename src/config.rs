use serde::Deserialize;
use std::net::SocketAddr;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub app: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub history_size: usize,
    pub broadcast_capacity: usize,
}

impl Config {
    pub fn load() -> Self {
        // Load .env file
        dotenv().ok();
        // You could load from environment variables
        Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            app: AppConfig {
                history_size: std::env::var("HISTORY_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                broadcast_capacity: std::env::var("BROADCAST_CAPACITY")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
            },
        }
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .unwrap()
    }
}