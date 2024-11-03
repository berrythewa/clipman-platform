use std::sync::Arc;
use uuid::Uuid;
use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::Device,
};

pub struct DeviceService {
    config: Arc<Config>,
}

impl DeviceService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn register_device(&self, user_id: Uuid, name: String) -> AppResult<Device> {
        // TODO add storage later
        let device = Device::new(name, user_id);
        Ok(device)
    }

    pub async fn get_user_devices(&self, user_id: Uuid) -> AppResult<Vec<Device>> {
        // TODO add storage later
        Ok(Vec::new())
    }

    pub async fn get_device(&self, id: Uuid) -> AppResult<Device> {
        Err(AppError::DeviceNotFound(id))
    }

    pub async fn update_device_status(&self, id: Uuid) -> AppResult<Device> {
        let mut device = self.get_device(id).await?;
        device.update_last_seen();
        Ok(device)
    }

    pub async fn verify_device(&self, device_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let device = self.get_device(device_id).await?;
        if device.user_id != user_id {
            return Err(AppError::DeviceUnauthorized(device_id));
        }
        Ok(true)
    }

    pub async fn remove_device(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        self.verify_device(id, user_id).await?;
        // TODO add deletion logic later
        Ok(())
    }
}