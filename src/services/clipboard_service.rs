use std::sync::Arc;
use crate::{
    error::{AppError, AppResult},
    models::ClipboardData,
    config::Config,
};
use tokio::sync::{broadcast, RwLock};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ClipboardService {
    config: Arc<Config>,
    clipboard_data: Arc<RwLock<HashMap<Uuid, ClipboardData>>>,
    tx: broadcast::Sender<ClipboardData>,
}

impl ClipboardService {
    pub fn new(config: Arc<Config>) -> Self {
        let (tx, _) = broadcast::channel(config.clipboard.broadcast_capacity);
        Self {
            config,
            clipboard_data: Arc::new(RwLock::new(HashMap::new())),
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ClipboardData> {
        self.tx.subscribe()
    }

    pub async fn save_clipboard(&self, mut data: ClipboardData) -> AppResult<ClipboardData> {
        // Validate content size
        if data.content.len() > self.config.clipboard.max_size {
            return Err(AppError::ValidationError("Content exceeds maximum size".to_string()));
        }

        data.received_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AppError::InternalError(format!("Time error: {}", e)))?
            .as_secs();

        let mut storage = self.clipboard_data.write().await;
        storage.insert(data.id, data.clone());

        // Broadcast update, ignore errors as receivers might have disconnected
        let _ = self.tx.send(data.clone());

        Ok(data)
    }

    pub async fn get_clipboard(&self, id: Uuid) -> AppResult<ClipboardData> {
        let storage = self.clipboard_data.read().await;
        storage
            .get(&id)
            .cloned()
            .ok_or(AppError::ClipboardNotFound(id))
    }

    pub async fn get_user_clipboard(&self, user_id: Uuid) -> AppResult<Vec<ClipboardData>> {
        let storage = self.clipboard_data.read().await;
        let mut user_data: Vec<ClipboardData> = storage
            .values()
            .filter(|data| data.user_id == user_id)
            .cloned()
            .collect();

        if user_data.is_empty() {
            return Err(AppError::ValidationError("No clipboard data found for user".to_string()));
        }

        // Sort by received_at in descending order
        user_data.sort_by(|a, b| b.received_at.cmp(&a.received_at));
        Ok(user_data)
    }

    pub async fn get_device_clipboard(&self, device_id: Uuid) -> AppResult<Vec<ClipboardData>> {
        let storage = self.clipboard_data.read().await;
        let mut device_data: Vec<ClipboardData> = storage
            .values()
            .filter(|data| data.device_id == device_id)
            .cloned()
            .collect();

        if device_data.is_empty() {
            return Err(AppError::ValidationError("No clipboard data found for device".to_string()));
        }

        // Sort by received_at in descending order
        device_data.sort_by(|a, b| b.received_at.cmp(&a.received_at));
        Ok(device_data)
    }

    pub async fn delete_clipboard(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut storage = self.clipboard_data.write().await;
        
        // Check if the clipboard belongs to the user
        if let Some(data) = storage.get(&id) {
            if data.user_id != user_id {
                return Err(AppError::DeviceUnauthorized(id));
            }
        }

        storage
            .remove(&id)
            .ok_or(AppError::ClipboardNotFound(id))?;
        Ok(())
    }

    pub async fn delete_user_clipboard(&self, user_id: Uuid) -> AppResult<()> {
        let mut storage = self.clipboard_data.write().await;
        let initial_len = storage.len();
        
        storage.retain(|_, data| data.user_id != user_id);
        
        if storage.len() == initial_len {
            return Err(AppError::ValidationError("No clipboard data found for user".to_string()));
        }
        
        Ok(())
    }

    pub async fn cleanup_old_data(&self) -> AppResult<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AppError::InternalError(format!("Time error: {}", e)))?
            .as_secs();
            
        let retention_period = self.config.clipboard.retention_period;
        let mut storage = self.clipboard_data.write().await;
        
        storage.retain(|_, data| {
            now - data.received_at < retention_period
        });
        
        Ok(())
    }

    pub async fn get_latest_clipboard(&self, user_id: Uuid) -> AppResult<ClipboardData> {
        let storage = self.clipboard_data.read().await;
        storage
            .values()
            .filter(|data| data.user_id == user_id)
            .max_by_key(|data| data.received_at)
            .cloned()
            .ok_or(AppError::ValidationError("No clipboard data found for user".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_config() -> Arc<Config> {
        let mut config = Config::default();
        // Override some config values for testing
        config.clipboard.retention_period = 1; // 1 second for testing
        config.clipboard.max_size = 100; // 100 bytes for testing
        Arc::new(config)
    }

    fn create_test_data(user_id: Uuid, device_id: Uuid) -> ClipboardData {
        ClipboardData::new(
            "test content".to_string(),
            device_id,
            user_id,
        )
    }

    #[tokio::test]
    async fn test_save_and_get_clipboard() {
        let service = ClipboardService::new(create_test_config());
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        
        let data = create_test_data(user_id, device_id);
        let saved_data = service.save_clipboard(data.clone()).await.unwrap();
        
        let retrieved_data = service.get_clipboard(saved_data.id).await.unwrap();
        assert_eq!(retrieved_data.content, "test content");
        assert_eq!(retrieved_data.user_id, user_id);
        assert_eq!(retrieved_data.device_id, device_id);
    }

    #[tokio::test]
    async fn test_get_user_clipboard() {
        let service = ClipboardService::new(create_test_config());
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        
        for i in 0..3 {
            let mut data = create_test_data(user_id, device_id);
            data.content = format!("content {}", i);
            service.save_clipboard(data).await.unwrap();
        }
        
        let user_data = service.get_user_clipboard(user_id).await.unwrap();
        assert_eq!(user_data.len(), 3);
    }

    #[tokio::test]
    async fn test_content_too_large() {
        let service = ClipboardService::new(create_test_config());
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        
        let mut data = create_test_data(user_id, device_id);
        data.content = "x".repeat(101); // Exceeds max_size of 100

        let result = service.save_clipboard(data).await;
        assert!(matches!(
            result,
            Err(AppError::ValidationError(msg)) if msg.contains("exceeds maximum size")
        ));
    }

    #[tokio::test]
    async fn test_delete_unauthorized() {
        let service = ClipboardService::new(create_test_config());
        let user_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        
        let data = create_test_data(user_id, device_id);
        let saved_data = service.save_clipboard(data).await.unwrap();
        
        let result = service.delete_clipboard(saved_data.id, other_user_id).await;
        assert!(matches!(result, Err(AppError::DeviceUnauthorized(_))));
    }

    #[tokio::test]
    async fn test_broadcast_updates() {
        let service = ClipboardService::new(create_test_config());
        let mut rx1 = service.subscribe();
        let mut rx2 = service.subscribe();

        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        let data = create_test_data(user_id, device_id);
        
        let saved_data = service.save_clipboard(data).await.unwrap();
        
        let received1 = rx1.try_recv().unwrap();
        let received2 = rx2.try_recv().unwrap();
        
        assert_eq!(received1.id, saved_data.id);
        assert_eq!(received2.id, saved_data.id);
    }

    #[tokio::test]
    async fn test_cleanup_old_data() {
        let service = ClipboardService::new(create_test_config());
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        
        let data = create_test_data(user_id, device_id);
        service.save_clipboard(data).await.unwrap();
        
        // Wait for data to expire (retention period is 1 second)
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        service.cleanup_old_data().await.unwrap();
        
        let result = service.get_user_clipboard(user_id).await;
        assert!(result.is_err());
    }
}