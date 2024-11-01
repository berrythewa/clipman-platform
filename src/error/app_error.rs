use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

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
    LockError(String),  // For Mutex lock failures
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
        match self {
            Self::Unauthorized(msg) => ErrorResponse {
                code: 401,
                message: "Unauthorized".to_string(),
                details: Some(msg.clone()),
            },
            Self::UserNotFound(id) => ErrorResponse {
                code: 404,
                message: "User not found".to_string(),
                details: Some(format!("User with ID {} not found", id)),
            },
            // Add other error mappings...
            _ => ErrorResponse {
                code: 500,
                message: "Internal server error".to_string(),
                details: None,
            },
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