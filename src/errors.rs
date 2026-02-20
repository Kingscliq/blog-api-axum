use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalServer(String),
    UnAuthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::UnAuthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };

        let response_body = Json(json!(
            {
            "error": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status_code": 42,
            "success": false

        }));

        (status, response_body).into_response()
    }
}
