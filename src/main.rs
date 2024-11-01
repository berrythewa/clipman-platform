use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}}, 
    response::IntoResponse,
    http::StatusCode,
    response::Response,
};
use axum_server;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use tokio::sync::broadcast;
// use std::time::UNIX_EPOCH;
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, header::CONTENT_TYPE};
use clipman_platform::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ClipboardData {
    content: String,
    device_id: String,
    sent_at: u64,
    received_at: u64,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

struct AppStateInner {
    history: VecDeque<ClipboardData>,
    tx: broadcast::Sender<ClipboardData>,
}

// custom error response
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

enum AppError {
    InvalidInput(String),
    // Add other error types here
}

// Implement IntoResponse for our error type
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InvalidInput(msg) => {
                let error_response = ErrorResponse {
                    error: msg,
                    code: StatusCode::BAD_REQUEST.as_u16(),
                };
                
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            }
        }
    }
}

type AppState = Arc<Mutex<AppStateInner>>;
// type ApiResult<T> = Result<Json<T>, (StatusCode, Json<ErrorResponse>)>;


// Helper functions
// fn create_test_state() -> AppState {
//     let (tx, _) = broadcast::channel(100);
//     Arc::new(Mutex::new(AppStateInner {
//         history: VecDeque::with_capacity(10),
//         tx,
//     }))
// }

// fn current_timestamp() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as u64
// }

// handlers

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// Handle individual WebSocket connection
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    
    // Get broadcast receiver
    let mut rx = {
        let state = state.lock().unwrap();
        state.tx.subscribe()
    };

    // Spawn task to forward broadcast messages to WebSocket
    let send_task = tokio::spawn(async move {
        while let Ok(data) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&data) {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Wait for either task to finish
    send_task.await.ok();
}

async fn receive_clipboard(
    State(state): State<AppState>,
    Json(mut data): Json<ClipboardData>,
) -> Result<Json<ClipboardData>, AppError> {
    
   // Validate input
    if data.content.is_empty() {
        return Err(AppError::InvalidInput("Content cannot be empty".to_string()));
    }

    // Add server-side timestamp
    data.received_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // sent_at should come from the client
    // If not provided, default to received_at
    if data.sent_at == 0 {
        data.sent_at = data.received_at;
    }

    let mut state = state.lock().unwrap();
    state.history.push_front(data.clone());
    while state.history.len() > 10 {
        state.history.pop_back();
    }
    
    // broadcast to /ws route
    let _ = state.tx.send(data.clone());

    Ok(Json(data))
}

async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, Clipboard Sync!".to_string(),
    })
}

async fn get_history(
    State(state): State<AppState>,
) -> Json<Vec<ClipboardData>> {
    let state = state.lock().unwrap();
    Json(state.history.iter().cloned().collect())
}

#[tokio::main]
async fn main() {

    //loading config
    let config = Config::load();

    // Create broadcast channel
    let (tx, _rx) = broadcast::channel::<ClipboardData>(100);  // specify type and channel size

    // Initialize application state
    let state = Arc::new(Mutex::new(AppStateInner {
        history: VecDeque::new(),
        tx,
    }));

    // cors configs
    // let cors = CorsLayer::new()
    // .allow_origin([
    //     "http://localhost:3000".parse::<HeaderValue>().unwrap(),
    //     "http://localhost:8080".parse::<HeaderValue>().unwrap(),
    // ])
    // .allow_methods([Method::GET, Method::POST])
    // .allow_headers([CONTENT_TYPE]);
    let cors = CorsLayer::new()
        // allow all origins
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/clipboard", post(receive_clipboard))
        .route("/history", get(get_history))
        .route("/ws", get(websocket_handler))
        .layer(cors)
        .with_state(state); 
    
    // config addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    
    println!("🚀 Server starting on {}", addr);
    println!("📝 POST /clipboard - Send clipboard data");
    println!("📋 GET /history - Get clipboard history");
    println!("🔌 GET /ws - WebSocket connection for real-time updates");

    if let Err(e) = axum_server::bind(config.server_addr())
        .serve(app.into_make_service())
        .await 
    {
        eprintln!("Server error: {}", e);
    }
    
}