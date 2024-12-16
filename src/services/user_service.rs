use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use crate::error::{AppError, AppResult};
use crate::config::Config;
use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordVerifier},
    Argon2, PasswordHash
};
use serde::{Serialize, Deserialize};
use crate::models::User;
use crate::models::UserResponse;
pub struct UserService {
    config: Arc<Config>,
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    usernames: Arc<RwLock<HashMap<String, Uuid>>>,  // For username lookups
}

impl UserService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            users: Arc::new(RwLock::new(HashMap::new())),
            usernames: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_user(&self, username: String, password: String) -> AppResult<User> {
        // Validate password length
        if password.len() < self.config.user.min_password_length {
            return Err(AppError::ValidationError("Password too short".to_string()));
        }

        // Check if username already exists
        {
            let usernames = self.usernames.read().await;
            if usernames.contains_key(&username) {
                return Err(AppError::UserAlreadyExists(username));
            }
        }

        // Hash password
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?
            .to_string();

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            username: username.clone(),
            password_hash,
            created_at: None,
            updated_at: None,
        };

        // Store user
        {
            let mut users = self.users.write().await;
            let mut usernames = self.usernames.write().await;
            users.insert(user.id, user.clone());
            usernames.insert(username, user.id);
        }

        Ok(user)
    }

    pub async fn list_users(&self) -> AppResult<Vec<UserResponse>> {
        let users = self.users.read().await;
        let mut user_list: Vec<UserResponse> = users
            .values()
            .cloned()
            .map(UserResponse::from)
            .collect();
        
        // Sort by username
        user_list.sort_by(|a, b| a.username.cmp(&b.username));
        Ok(user_list)
    }

    pub async fn list_users_paginated(&self, page: u32, limit: u32) -> Vec<UserResponse> {
        let users = self.users.read().await;

        // Convert HashMap values to a vector
        let mut user_list: Vec<UserResponse> = users
            .values()
            .cloned()
            .map(UserResponse::from)
            .collect();

        // Sort by username
        user_list.sort_by(|a, b| a.username.cmp(&b.username));

        // Apply pagination
        let start = ((page - 1) * limit) as usize;
        let end = (start + limit as usize).min(user_list.len());

        user_list[start..end].to_vec()
    }

   
    pub async fn user_count(&self) -> usize {
        let users = self.users.read().await;
        users.len()
    }
    

    pub async fn get_user_by_username(&self, username: &str) -> AppResult<User> {
        let usernames = self.usernames.read().await;
        let users = self.users.read().await;
        
        let user_id = usernames
            .get(username)
            .ok_or_else(|| AppError::ValidationError(format!("User {} not found", username)))?;
        
        users
            .get(user_id)
            .cloned()
            .ok_or_else(|| AppError::UserNotFound(*user_id))
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::InternalError(format!("Invalid hash format: {}", e)))?;
        
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<User> {
        // TODO add storage logic later
        Err(AppError::UserNotFound(id))
    }

    // pub async fn get_user_by_username(&self, username: &str) -> AppResult<User> {
    //     // TODO add storage logic later
    //     Err(AppError::ValidationError(format!("User {} not found", username)))
    // }

    // pub async fn update_user(&self, id: Uuid, new_username: Option<String>) -> AppResult<User> {
    pub async fn update_user(&self, id: Uuid) -> AppResult<User> {

        // TODO add storage logic 
        Err(AppError::UserNotFound(id))
    }

    // pub async fn update_password(&self, id: Uuid, old_password: &str, new_password: &str) -> AppResult<()> {
    pub async fn update_password(&self, id: Uuid, old_password: &str) -> AppResult<()> {

        let user = self.get_user_by_id(id).await?;
        
        // Verify old password
        self.verify_password(&user, old_password).await?;
        
        // Generate new hash
        // let salt = SaltString::generate(&mut rand::thread_rng());
        // let argon2 = Argon2::default();
        
        // let new_hash = argon2
        //     .hash_password(new_password.as_bytes(), &salt)
        //     .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?
        //     .to_string();

        // TODO add storage update logic
        Ok(())
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        // We'll add storage logic later
        Err(AppError::UserNotFound(id))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User::new(
            "testuser".to_string(),
            "hashedpassword123".to_string(),
        );

        let json = serde_json::to_string(&user).unwrap();
        println!("Serialized user: {}", json);
        
        // Should only contain id and username, not password_hash
        assert!(json.contains("username"));
        assert!(json.contains("id"));
        assert!(!json.contains("password_hash"));
    }
}