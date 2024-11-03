mod auth_handler;
mod user_handler;
mod device_handler;
mod websocket_handler;

pub use auth_handler::auth_routes;
pub use user_handler::user_routes;
pub use device_handler::device_routes;
pub use websocket_handler::websocket_handler;