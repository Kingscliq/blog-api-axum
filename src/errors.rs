use std::collections::HashMap;

use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalServer(String),
    UnAuthorized(String),
    ValidationError(validator::ValidationErrors),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    status_code: u16,
    timestamp: String,
    success: bool,
}

impl ErrorResponse {
    fn new(message: String, status_code: u16) -> Self {
        Self {
            message,
            status_code,
            timestamp: chrono::Utc::now().to_rfc3339(),
            success: false,
        }
    }
}

#[derive(Serialize)]
struct ValidationError {
    errors: HashMap<String, Vec<String>>,
    status_code: u16,
    timestamp: String,
    success: bool,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ResponseBody {
    Error(ErrorResponse),
    Validation(ValidationError),
}

impl ValidationError {
    fn new(errors: &validator::ValidationErrors) -> Self {
        let field_errors = errors
            .field_errors()
            .into_iter()
            .map(|(field, errs)| {
                let messages = errs
                    .iter()
                    .map(|e| e.message.clone().unwrap_or_default().to_string())
                    .collect();
                (field.to_string(), messages)
            })
            .collect();

        Self {
            errors: field_errors,
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            success: false,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, error_response) = match self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ResponseBody::Error(ErrorResponse::new(msg, StatusCode::BAD_REQUEST.as_u16())),
            ),
            AppError::InternalServer(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::Error(ErrorResponse::new(
                    msg,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                )),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ResponseBody::Error(ErrorResponse::new(msg, StatusCode::NOT_FOUND.as_u16())),
            ),
            AppError::UnAuthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ResponseBody::Error(ErrorResponse::new(msg, StatusCode::UNAUTHORIZED.as_u16())),
            ),
            AppError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                ResponseBody::Validation(ValidationError::new(&errors)),
            ),
        };

        (status, Json(error_response)).into_response()
    }
}
