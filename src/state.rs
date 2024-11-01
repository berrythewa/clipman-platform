use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::{UserService, AuthService};
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub async fn new(config: Config) -> Self {
        let config = Arc::new(config);
        
        // Initialize services
        let user_service = Arc::new(UserService::new(config.clone()));
        let auth_service = Arc::new(AuthService::new(config.clone()));

        Self {
            config,
            user_service,
            auth_service,
        }
    }
}


// In main.rs
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    
    let app = Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .with_state(state);
}

// // In handlers, we now extract specific services:
// async fn login(
//     State(state): State<AppState>,
//     Json(login_req): Json<LoginRequest>,
// ) -> AppResult<Json<TokenResponse>> {
//     let user = state.user_service.get_user_by_username(&login_req.username).await?;
//     state.user_service.verify_password(&user, &login_req.password).await?;
    
//     let (access_token, refresh_token) = state.auth_service.create_token_pair(user.id)?;
//     // ...
// }