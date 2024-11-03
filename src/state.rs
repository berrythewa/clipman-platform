use std::sync::Arc;

use crate::services::{UserService, AuthService, WebSocketService, DeviceService};
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub device_service: Arc<DeviceService>,
    pub ws_service: Arc<WebSocketService>, 
}

impl AppState {
    pub async fn new(config: Config) -> Self {
        let config = Arc::new(config);
        
        // Initialize services
        let user_service = Arc::new(UserService::new(config.clone()));
        let auth_service = Arc::new(AuthService::new(config.clone()));
        let device_service = Arc::new(DeviceService::new(config.clone()));
        let ws_service = Arc::new(WebSocketService::new(config.clone()));

        Self {
            config,
            user_service,
            auth_service,
            device_service,
            ws_service,
        }
    }
}