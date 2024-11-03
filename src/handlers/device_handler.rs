use axum::{
    routing::{post, get, delete},
    Router,
    Json,
    extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    error::AppResult,
    models::Device,
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    name: String,
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateDeviceRequest {
    name: Option<String>,
}

pub fn device_routes() -> Router<AppState> {
    Router::new()
        .route("/devices", post(register_device))
        .route("/devices/:id", get(get_device))
        .route("/devices/:id/status", post(update_device_status))
        .route("/devices/:id", delete(remove_device))
        .route("/users/:user_id/devices", get(get_user_devices))
}

async fn register_device(
    State(state): State<AppState>,
    Json(req): Json<RegisterDeviceRequest>,
) -> AppResult<Json<Device>> {
    let device = state.device_service
        .register_device(req.user_id, req.name)
        .await?;
    Ok(Json(device))
}

async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Device>> {
    let device = state.device_service.get_device(id).await?;
    Ok(Json(device))
}

async fn update_device_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Device>> {
    let device = state.device_service.update_device_status(id).await?;
    Ok(Json(device))
}

async fn remove_device(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    // TODO: Get user_id from auth token
    Path(user_id): Path<Uuid>,
) -> AppResult<()> {
    state.device_service.remove_device(id, user_id).await
}

async fn get_user_devices(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<Vec<Device>>> {
    let devices = state.device_service.get_user_devices(user_id).await?;
    Ok(Json(devices))
}