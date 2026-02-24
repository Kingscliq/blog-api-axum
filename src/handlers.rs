use crate::{
    ApiResponse, Post,
    app::AppState,
    errors::AppError,
    logger::AppLogger,
    services::{create_post_service, get_posts_service},
};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 5, message = "Description must not be less than 6 characters"))]
    pub description: String,
    pub image_url: Option<String>,
    pub content: String,
}
pub async fn create_post_handler(
    State(app_state): State<AppState>,
    Json(post_request): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, AppError> {
    post_request.validate().map_err(|e| {
        AppLogger::error(&format!("❌ Validation error: {}", e));
        AppError::ValidationError(e)
    })?;

    let created_post: Post = create_post_service(&app_state.db, post_request).await?;

    AppLogger::info("Post created successfully 🚀");
    Ok(Json(ApiResponse {
        data: created_post,
        status_code: StatusCode::CREATED.as_u16(),
        message: "Post created successfully 🚀".to_string(),
    }))
}

pub async fn get_all_posts(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Post>>>, AppError> {
    let posts = get_posts_service(&app_state.db).await?;

    let res = ApiResponse {
        data: posts,
        status_code: StatusCode::OK.as_u16(),
        message: "Post retrieved successfully 🚀".to_string(),
    };

    Ok(Json(res))
}

pub async fn health_handler() -> Json<String> {
    Json("App sarted running successfully! 🚀🔥".to_string())
}
