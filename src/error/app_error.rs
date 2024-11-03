use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    // Auth errors
    Unauthorized(String),
    InvalidToken,
    TokenExpired,
    // User errors
    UserNotFound(Uuid),
    UserAlreadyExists(String),
    InvalidCredentials,
    // Device errors
    DeviceNotFound(Uuid),
    DeviceUnauthorized(Uuid),
    TooManyDevices,
    // Clipboard errors
    ClipboardNotFound(Uuid),
    InvalidClipboardData(String),
    // Database errors
    DatabaseError(String),
    // Validation errors
    ValidationError(String),
    // Generic errors
    InternalError(String),
    // Connection and data transmission errors
    WebSocketConnectionError(String),
    WebSocketMessageError(String),
    BroadcastError(String),
    LockError(String),
}

// Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::TokenExpired => write!(f, "Token expired"),
            Self::UserNotFound(id) => write!(f, "User not found: {}", id),
            Self::UserAlreadyExists(username) => write!(f, "User already exists: {}", username),
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
            Self::DeviceNotFound(id) => write!(f, "Device not found: {}", id),
            Self::DeviceUnauthorized(id) => write!(f, "Device unauthorized: {}", id),
            Self::TooManyDevices => write!(f, "Too many devices"),
            Self::ClipboardNotFound(id) => write!(f, "Clipboard not found: {}", id),
            Self::InvalidClipboardData(msg) => write!(f, "Invalid clipboard data: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Self::WebSocketConnectionError(msg) => write!(f, "WebSocket connection error: {}", msg),
            Self::WebSocketMessageError(msg) => write!(f, "WebSocket message error: {}", msg),
            Self::BroadcastError(msg) => write!(f, "Broadcast error: {}", msg),
            Self::LockError(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) | Self::InvalidToken | Self::TokenExpired => StatusCode::UNAUTHORIZED,
            Self::UserNotFound(_) | Self::DeviceNotFound(_) | Self::ClipboardNotFound(_) => StatusCode::NOT_FOUND,
            Self::UserAlreadyExists(_) => StatusCode::CONFLICT,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::DeviceUnauthorized(_) => StatusCode::FORBIDDEN,
            Self::TooManyDevices => StatusCode::TOO_MANY_REQUESTS,
            Self::ValidationError(_) | Self::InvalidClipboardData(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) | Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WebSocketConnectionError(_) => StatusCode::BAD_GATEWAY,
            Self::WebSocketMessageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BroadcastError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::LockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.status_code().as_u16(),
            message: self.to_string(),  // Now we can use to_string() because we implemented Display
            details: None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(self.error_response());
        (status, body).into_response()
    }
}

// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;