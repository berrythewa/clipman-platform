use mock_config;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::mock_config;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_token_pair() {
        let config = Arc::new(mock_config());
        let auth_service = AuthService::new(config);

        let user_id = Uuid::new_v4();
        let (access_token, refresh_token) = auth_service.create_token_pair(user_id).unwrap();

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_verify_token() {
        let config = Arc::new(mock_config());
        let auth_service = AuthService::new(config);

        let user_id = Uuid::new_v4();
        let (access_token, _) = auth_service.create_token_pair(user_id).unwrap();

        let claims = auth_service.verify_token(&access_token).await.unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[tokio::test]
    async fn test_refresh_token() {
        let config = Arc::new(mock_config());
        let auth_service = AuthService::new(config);

        let user_id = Uuid::new_v4();
        let (_, refresh_token) = auth_service.create_token_pair(user_id).unwrap();

        let new_access_token = auth_service.refresh_token(&refresh_token).await.unwrap();
        assert!(!new_access_token.is_empty());
    }

    #[tokio::test]
    async fn test_invalidate_token() {
        let config = Arc::new(mock_config());
        let auth_service = AuthService::new(config);

        let user_id = Uuid::new_v4();
        let (access_token, _) = auth_service.create_token_pair(user_id).unwrap();

        auth_service.invalidate_token(&access_token).await.unwrap();
        let result = auth_service.verify_token(&access_token).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout() {
        let config = Arc::new(mock_config());
        let auth_service = AuthService::new(config);

        let user_id = Uuid::new_v4();
        let (access_token, refresh_token) = auth_service.create_token_pair(user_id).unwrap();

        auth_service.logout(&access_token, &refresh_token).await.unwrap();

        let access_result = auth_service.verify_token(&access_token).await;
        let refresh_result = auth_service.verify_token(&refresh_token).await;

        assert!(access_result.is_err());
        assert!(refresh_result.is_err());
    }
}
