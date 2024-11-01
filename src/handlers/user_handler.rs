use axum::{
    routing::{post, get, put, delete},
    Router,
    Json,
    extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    error::AppResult,
    models::User,
    services::UserService,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    username: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    old_password: String,
    new_password: String,
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/users/:id/password", put(update_password))
}

async fn create_user(
    State(service): State<UserService>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    let user = service.register_user(req.username, req.password).await?;
    Ok(Json(user))
}

async fn get_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = service.get_user_by_id(id).await?;
    Ok(Json(user))
}

async fn update_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let user = service.update_user(id, req.username).await?;
    Ok(Json(user))
}

async fn update_password(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePasswordRequest>,
) -> AppResult<()> {
    service.update_password(id, &req.old_password, &req.new_password).await
}

async fn delete_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    service.delete_user(id).await
}