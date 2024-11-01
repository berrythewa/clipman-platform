use argon2::{self, Config, password_hash::SaltString};
use crate::error::{AppError, AppResult};
use crate::models::User;
use rand::Rng;
use uuid::Uuid;

pub struct UserService {
    // We'll add storage later
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn register_user(&self, username: String, password: String) -> AppResult<User> {
        // Generate random salt
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let hash = argon2::hash_encoded(
            password.as_bytes(),
            &salt,
            &config
        ).map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

        Ok(User::new(username, hash))
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> AppResult<bool> {
        argon2::verify_encoded(&user.password_hash, password.as_bytes())
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<User> {
        // We'll add storage logic later
        Err(AppError::UserNotFound(id))
    }

    pub async fn get_user_by_username(&self, username: &str) -> AppResult<User> {
        // We'll add storage logic later
        Err(AppError::ValidationError(format!("User {} not found", username)))
    }

    pub async fn update_user(&self, id: Uuid, new_username: Option<String>) -> AppResult<User> {
        // We'll add storage logic later
        Err(AppError::UserNotFound(id))
    }

    pub async fn update_password(&self, id: Uuid, old_password: &str, new_password: &str) -> AppResult<()> {
        let user = self.get_user_by_id(id).await?;
        
        // Verify old password
        self.verify_password(&user, old_password).await?;
        
        // Generate new hash
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();
        
        let new_hash = argon2::hash_encoded(
            new_password.as_bytes(),
            &salt,
            &config
        ).map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

        // We'll add storage update logic later
        Ok(())
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        // We'll add storage logic later
        Err(AppError::UserNotFound(id))
    }
}