pub fn mock_app_state() -> AppState {
    let config = Config::default();
    AppState::new(config).await
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_app_state_initialization() {
        // Mock configuration
        let config = Config {
            auth: crate::config::AuthConfig {
                jwt_secret: "testsecret".to_string(),
            },
            // Add other mock fields if required
        };

        // Initialize AppState
        let state = AppState::new(config).await;

        // Assert services are initialized
        assert!(state.user_service.is_some());
        assert!(state.auth_service.is_some());
        assert!(state.device_service.is_some());
        assert!(state.ws_service.is_some());
    }
}
