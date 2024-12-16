use std::sync::Arc;
use crate::config::{Config, AuthConfig, ServerConfig, UserConfig, WebSocketConfig, ClipboardConfig, AppConfig};
use crate::state::AppState;
use crate::services::{AuthService, UserService, DeviceService, WebSocketService};

// Mock Config
pub fn mock_config() -> Config {
    Config {
        auth: AuthConfig {
            jwt_secret: "test_secret".to_string(),
            access_token_expiry: 3600,   // 1 hour
            refresh_token_expiry: 604800, // 7 days
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        user: UserConfig {
            min_password_length: 8,
            max_username_length: 32,
            password_rounds: 3,
            memory_size: 65536,
        },
        websocket: WebSocketConfig {
            channel_capacity: 100,
        },
        clipboard: ClipboardConfig {
            retention_period: 3600, // 1 hour
            max_size: 1024 * 1024,  // 1 MB
            broadcast_capacity: 100,
        },
        app: AppConfig {
            history_size: 10,
            broadcast_capacity: 100,
        },
    }
}

// Mock AppState
pub async fn mock_app_state() -> AppState {
    let config = Arc::new(mock_config());

    AppState {
        config: config.clone(),
        user_service: Arc::new(UserService::new(config.clone())),
        auth_service: Arc::new(AuthService::new(config.clone())),
        device_service: Arc::new(DeviceService::new(config.clone())),
        ws_service: Arc::new(WebSocketService::new(config.clone())),
    }
}
