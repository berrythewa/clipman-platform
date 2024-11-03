use axum::{
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
    extract::State,
};
use crate::state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        state.ws_service.handle_connection(socket).await
    })
}