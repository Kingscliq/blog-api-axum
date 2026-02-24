use sqlx::PgPool;

use crate::{Post, errors::AppError, handlers::CreatePostRequest, logger::AppLogger};

pub async fn create_post_service(db: &PgPool, post: CreatePostRequest) -> Result<Post, AppError> {
    let image_url = post
        .image_url
        .unwrap_or("https://placehold.net/400x400.png".to_string());
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (title, description, image_url, content)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, image_url, content, description, created_at, updated_at
        "#,
        post.title.trim(),
        image_url,
        post.content.trim(),
        post.description.trim()
    )
    .fetch_one(db)
    .await
    .map_err(|err| {
        AppLogger::error(&format!("Error creating post: {err:?}"));
        AppError::InternalServer(format!("Error creating post: {err:?}"))
    });

    post
}

pub async fn get_posts_service(db: &PgPool) -> Result<Vec<Post>, AppError> {
    let posts = sqlx::query_as!(
        Post,
        r#"
  SELECT id, title, image_url, content, description, created_at, updated_at
  FROM posts
  ORDER BY created_at DESC

  "#
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        AppLogger::error(&format!("An Error occured fetching post: {e}"));
        AppError::InternalServer(format!("An Error occured fetching post: {e}"))
    })?;

    Ok(posts)
}
