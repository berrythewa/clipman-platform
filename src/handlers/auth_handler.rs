use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
    // response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppResult,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

async fn login(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> AppResult<Json<TokenResponse>> {
    let user = state.user_service
        .get_user_by_username(&login_req.username).await?;
    
    state.user_service
        .verify_password(&user, &login_req.password).await?;
    
    let (access_token, refresh_token) = state.auth_service
        .create_token_pair(user.id)?;
    
    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(refresh_req): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let access_token = state.auth_service
        .refresh_token(&refresh_req.refresh_token).await?;
    
    Ok(Json(TokenResponse {
        access_token,
        refresh_token: refresh_req.refresh_token,
    }))
}

#[axum_macros::debug_handler]
async fn logout(
    State(state): State<AppState>,
    Json(tokens): Json<TokenResponse>,
) -> (StatusCode, Json<String>) {
    match state.auth_service.logout(&tokens.access_token, &tokens.refresh_token).await {
        Ok(()) => (StatusCode::OK, Json("Logged out successfully".to_string())),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(e.to_string()))
    }
}
