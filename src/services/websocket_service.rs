use axum::extract::ws::{WebSocket, Message};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::ClipboardData,
};

pub struct WebSocketService {
    config: Arc<Config>,
    tx: broadcast::Sender<ClipboardData>,
}

impl WebSocketService {
    pub fn new(config: Arc<Config>) -> Self {
        let (tx, _) = broadcast::channel(config.websocket.channel_capacity);
        Self { config, tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ClipboardData> {
        self.tx.subscribe()
    }

    pub fn broadcast(&self, data: ClipboardData) -> AppResult<()> {
        self.tx.send(data)
            .map_err(|e| AppError::BroadcastError(e.to_string()))?;
        Ok(())
    }

    pub async fn handle_connection(&self, mut socket: WebSocket) {//TODO make socket borrowable without mut
        let mut rx = self.subscribe();

        let send_task = tokio::spawn(async move {
            while let Ok(data) = rx.recv().await {
                if let Ok(msg) = serde_json::to_string(&data) {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
        });

        send_task.await.ok();
    }
}