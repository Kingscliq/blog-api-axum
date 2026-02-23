use crate::{ApiResponse, Post, app::AppState, errors::AppError, logger::AppLogger};
use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
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

    let new_post = Post {
        id: Uuid::new_v4(),
        title: post_request.title,
        content: post_request.content,
        description: post_request.description,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        image_url: post_request
            .image_url
            .unwrap_or_else(|| "https://placehold.net/400x400.png".to_string()),
    };

    let res: ApiResponse<Post> = ApiResponse {
        data: new_post.clone(),
        status_code: StatusCode::CREATED.as_u16(),
        message: "Post created successfully 🚀".to_string(),
    };

    let mut post_guard = app_state.post_state.lock().await;

    post_guard.create_post(new_post).map_err(|e| {
        AppLogger::error(&format!("An Error Occured while creating post {e:?}"));
        e
    })?;

    AppLogger::info("Post created successfully 🚀");
    Ok(Json(res))
}

pub async fn get_all_posts(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Post>>>, AppError> {
    let response = app_state
        .post_state
        .lock()
        .await
        .posts
        .values()
        .cloned()
        .collect();

    let res = ApiResponse {
        data: response,
        status_code: StatusCode::OK.as_u16(),
        message: "Post retrieved successfully 🚀".to_string(),
    };

    Ok(Json(res))
}

pub async fn health_handler() -> Json<String> {
    Json("App sarted running successfully! 🚀🔥".to_string())
}
