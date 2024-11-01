use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppResult,
    services::{AuthService, UserService},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

async fn login(
    State(auth_service): State<AuthService>,
    State(user_service): State<UserService>,
    Json(login_req): Json<LoginRequest>,
) -> AppResult<Json<TokenResponse>> {
    let user = user_service.get_user_by_username(&login_req.username).await?;
    user_service.verify_password(&user, &login_req.password).await?;
    
    let (access_token, refresh_token) = auth_service.create_token_pair(user.id)?;
    
    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}

async fn refresh(
    State(auth_service): State<AuthService>,
    Json(refresh_req): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let access_token = auth_service.refresh_token(&refresh_req.refresh_token).await?;
    
    Ok(Json(TokenResponse {
        access_token,
        refresh_token: refresh_req.refresh_token,
    }))
}

async fn logout(
    State(auth_service): State<AuthService>,
    Json(tokens): Json<TokenResponse>,
) -> AppResult<()> {
    auth_service.logout(&tokens.access_token, &tokens.refresh_token).await
}