use crate::{ApiReponse, AppState, Post, errors::AppError, logger::AppLogger};
use axum::{Json, extract::State, http::StatusCode};
use chrono::Local;
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
) -> Result<Json<ApiReponse<Post>>, AppError> {
    let mut post_guard = app_state.post_state.lock().await;

    post_request.validate().map_err(|e| {
        AppLogger::error(&format!("❌ Validation error: {}", e));
        AppError::BadRequest(format!("❌ Validation error: {}", e))
    })?;

    let new_post = Post {
        id: Uuid::new_v4(),
        title: post_request.title,
        content: post_request.content,
        description: post_request.description,
        created_at: Local::now(),
        updated_at: Local::now(),
        image_url: post_request
            .image_url
            .unwrap_or_else(|| "https://placehold.net/400x400.png".to_string()),
    };

    let res: ApiReponse<Post> = ApiReponse {
        data: new_post.clone(),
        status_code: StatusCode::CREATED.as_u16(),
        message: "Post created successfully 🚀".to_string(),
    };

    if let Ok(_) = post_guard.create_post(new_post) {
        AppLogger::info("Post created successfully 🚀");
        Ok(Json(res))
    } else {
        AppLogger::error("An Error Occured while creating post");
        Err(AppError::BadRequest(
            "An Error Occured while creating post".to_string(),
        ))
    }
}
