use serde::Deserialize;
use std::net::SocketAddr;
use dotenv::dotenv;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub app: AppConfig,
    pub websocket: WebSocketConfig,
    pub auth: AuthConfig,
    pub user: UserConfig,
    pub clipboard: ClipboardConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClipboardConfig {
    #[serde(default = "default_retention_period")]
    pub retention_period: u64,  // in seconds
    #[serde(default = "default_max_size")]
    pub max_size: usize,   // maximum size of clipboard content
    #[serde(default = "default_max_size")]
    pub broadcast_capacity: usize, // TODO research this
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebSocketConfig {
    #[serde(default = "default_channel_capacity")]
    pub channel_capacity: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    #[serde(default = "default_access_token_expiry")]
    pub access_token_expiry: u64,
    #[serde(default = "default_refresh_token_expiry")]
    pub refresh_token_expiry: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserConfig {
    #[serde(default = "default_min_password_length")]
    pub min_password_length: usize,
    #[serde(default = "default_max_username_length")]
    pub max_username_length: usize,
    #[serde(default = "default_password_rounds")]
    pub password_rounds: u32,
    #[serde(default = "default_memory_size")]
    pub memory_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_history_size")]
    pub history_size: usize,
    #[serde(default = "default_broadcast_capacity")]
    pub broadcast_capacity: usize,
}

// Default value functions
fn default_retention_period() -> u64 { 24 * 60 * 60 }  // 24 hours
fn default_max_size() -> usize { 1024 * 1024 }         // 1MB
fn default_channel_capacity() -> usize { 100 }
fn default_access_token_expiry() -> u64 { 3600 }       // 1 hour
fn default_refresh_token_expiry() -> u64 { 604800 }    // 7 days
fn default_host() -> String { "127.0.0.1".to_string() }
fn default_port() -> u16 { 3000 }
fn default_min_password_length() -> usize { 8 }
fn default_max_username_length() -> usize { 32 }
fn default_password_rounds() -> u32 { 3 }
fn default_memory_size() -> u32 { 65536 }
fn default_history_size() -> usize { 10 }
fn default_broadcast_capacity() -> usize { 100 }

// Implement Default for all configs
impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            retention_period: default_retention_period(),
            max_size: default_max_size(),

            //Personal use (2-3 devices): 100-500
            // Small team (5-10 devices): 1000-3000
            // Medium team (10-50 devices): 5000-10000
            // Large deployment (50+ devices): 10000+
            broadcast_capacity: default_broadcast_capacity(), //TODO make it configurable
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            channel_capacity: default_channel_capacity(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "your-secret-key".to_string(),
            access_token_expiry: default_access_token_expiry(),
            refresh_token_expiry: default_refresh_token_expiry(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            min_password_length: default_min_password_length(),
            max_username_length: default_max_username_length(),
            password_rounds: default_password_rounds(),
            memory_size: default_memory_size(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            history_size: default_history_size(),
            broadcast_capacity: default_broadcast_capacity(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            app: AppConfig::default(),
            websocket: WebSocketConfig::default(),
            auth: AuthConfig::default(),
            user: UserConfig::default(),
            clipboard: ClipboardConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        let config = Config {
            clipboard: ClipboardConfig {
                retention_period: std::env::var("RETENTION_PERIOD")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_retention_period()),
                max_size: std::env::var("MAX_CLIPBOARD_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_max_size()),
                broadcast_capacity: 3000,
            },
            user: UserConfig {
                min_password_length: std::env::var("MIN_PASSWORD_LENGTH")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_min_password_length()),
                max_username_length: std::env::var("MAX_USERNAME_LENGTH")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_max_username_length()),
                password_rounds: std::env::var("PASSWORD_ROUNDS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_password_rounds()),
                memory_size: std::env::var("PASSWORD_MEMORY_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_memory_size()),
            },
            auth: AuthConfig {
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "your-secret-key".to_string()),
                access_token_expiry: std::env::var("ACCESS_TOKEN_EXPIRY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_access_token_expiry()),
                refresh_token_expiry: std::env::var("REFRESH_TOKEN_EXPIRY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_refresh_token_expiry()),
            },
            websocket: WebSocketConfig {
                channel_capacity: std::env::var("WS_CHANNEL_CAPACITY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_channel_capacity()),
            },
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| default_host()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_port()),
            },
            app: AppConfig {
                history_size: std::env::var("HISTORY_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_history_size()),
                broadcast_capacity: std::env::var("BROADCAST_CAPACITY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default_broadcast_capacity()),
            },
        };

        config
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address configuration")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.user.min_password_length, 8);
        assert_eq!(config.user.max_username_length, 32);
        assert_eq!(config.clipboard.max_size, 1024 * 1024);
    }

    #[test]
    fn test_server_addr() {
        let config = Config::default();
        let addr = config.server_addr();
        assert_eq!(addr.port(), 3000);
    }
}