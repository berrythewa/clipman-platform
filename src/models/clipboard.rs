use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClipboardData {
    pub id: Uuid,
    pub content: String,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub sent_at: u64,
    pub received_at: u64,
}

impl ClipboardData {
    pub fn new(content: String, device_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            device_id,
            user_id,
            sent_at: 0,  // set by client
            received_at: 0,  // set by server
        }
    }
}