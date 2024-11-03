use crate::error::{AppError, AppResult};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::Config;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

pub struct AuthService {
    config: Arc<Config>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    blacklist: Arc<Mutex<HashSet<String>>>,
}


impl AuthService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.auth.jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.auth.jwt_secret.as_bytes()),
            blacklist: Arc::new(Mutex::new(HashSet::new())),
            config,
        }
    }

    pub fn create_token_pair(&self, user_id: Uuid) -> AppResult<(String, String)> {
        let access_token = self.create_access_token(user_id)?;
        let refresh_token = self.create_refresh_token(user_id)?;
        Ok((access_token, refresh_token))
    }

    fn create_access_token(&self, user_id: Uuid) -> AppResult<String> {
        let exp = jsonwebtoken::get_current_timestamp() as usize + 3600; // 1 hour
        let claims = Claims {
            sub: user_id,
            exp,
            token_type: TokenType::Access,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Token creation failed: {}", e)))
    }

    fn create_refresh_token(&self, user_id: Uuid) -> AppResult<String> {
        let exp = jsonwebtoken::get_current_timestamp() as usize + 7 * 24 * 3600; // 7 days
        let claims = Claims {
            sub: user_id,
            exp,
            token_type: TokenType::Refresh,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Refresh token creation failed: {}", e)))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<String> {
        let claims = self.verify_token(refresh_token).await?;
        
        if claims.token_type != TokenType::Refresh {
            return Err(AppError::InvalidToken);
        }
        
        self.create_access_token(claims.sub)
    }

    pub async fn verify_token(&self, token: &str) -> AppResult<Claims> {
        // Check blacklist first
        if self.blacklist.lock().await.contains(token) {
            return Err(AppError::InvalidToken);
        }

        let claims = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default()
        )
        .map(|token_data| token_data.claims)
        .map_err(|_| AppError::InvalidToken)?;

        // Check if token is expired
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
            
        if claims.exp < now {
            return Err(AppError::TokenExpired);
        }

        Ok(claims)
    }

    pub async fn invalidate_token(&self, token: &str) -> AppResult<()> {
        let mut blacklist = self.blacklist.lock().await;
        blacklist.insert(token.to_string());
        Ok(())
    }

    pub async fn logout(&self, access_token: &str, refresh_token: &str) -> AppResult<()> {
        let mut blacklist = self.blacklist.lock().await;
        blacklist.insert(access_token.to_string());
        blacklist.insert(refresh_token.to_string());
        Ok(())
    }
}