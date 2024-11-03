use axum::{routing::get, Router};
use tower_http::{
    trace::TraceLayer,
    cors::{CorsLayer, Any},
};
use axum::http::{Method, header::CONTENT_TYPE};
use clipman_platform::{
    state::AppState,
    config::Config,
    handlers::{auth_routes, user_routes, device_routes, websocket_handler},
    utils::logger::setup_logger,
};
use std::sync::Arc;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() {
    // Initialize logger
    setup_logger();
    info!("Starting Clipman Platform");

    // Load configuration
    let config = Config::load();
    info!("Configuration loaded successfully");

    // Create application state
    let state = AppState::new(Arc::new(config)).await;
    let addr = state.config.server_addr();
    info!("Application state initialized");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([CONTENT_TYPE]);
    info!("CORS configuration set up");

    // Build our application with our routes
    let app = Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .merge(device_routes())
        .route("/ws", get(websocket_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())  // Add request tracing
        .with_state(state);
    info!("Router configured with all routes");

    // Print startup information
    info!("🚀 Server starting on {}", addr);
    info!("👤 User endpoints enabled");
    info!("🔒 Auth endpoints enabled");
    info!("📱 Device endpoints enabled");
    info!("🔌 WebSocket endpoint enabled");

    // Start the server
    info!("Starting HTTP server");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("🌐 Listening on http://{}", addr);

    match axum::serve(listener, app).await {
        Ok(_) => info!("Server shutdown gracefully"),
        Err(e) => error!("Server error: {}", e),
    }
}