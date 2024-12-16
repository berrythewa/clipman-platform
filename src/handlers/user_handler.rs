use axum::{
    routing::{post, get, put, delete},
    Router,
    Json,
    extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use uuid::Uuid;
use crate::{
    error::AppResult,
    models::User,
    state::AppState,
};
use crate::models::{UserResponse};
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

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: usize,
    pub page: u32,
    pub limit: u32,
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/register", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/users/:id/password", put(update_password))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    let user = state.user_service.register_user(req.username, req.password).await?;
    Ok(Json(user))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = state.user_service.get_user_by_id(id).await?;
    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let user = state.user_service.update_user(id).await?;

    // let user = state.user_service.update_user(id, req.username).await?;
    Ok(Json(user))
}

async fn update_password(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePasswordRequest>,
) -> AppResult<()> {
    state.user_service.update_password(id, &req.old_password).await

    // state.user_service.update_password(id, &req.old_password, &req.new_password).await
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    state.user_service.delete_user(id).await
}

async fn list_users(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<UserResponse>>> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let users = state.user_service.list_users_paginated(page, limit).await?;
    let total = state.user_service.user_count().await;
    let responses = users.into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
            created_at: None,
            updated_at: None,
        })
        .collect();
    // Ok(Json(responses))
    Json(PaginatedResponse {
        data: responses,
        meta: PaginationMeta {
            total,
            page,
            limit,
        },
    })
}