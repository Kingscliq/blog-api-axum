use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Response, StatusCode, response},
    response::IntoResponse,
};
use chrono::{DateTime, Local};
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::broadcast::error;
use uuid::Uuid;

struct Post {
    id: u8,
    title: String,
    image_url: String,
    content: String,
    description: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    author_id: u8,
}

struct PostRequest {
    title: String,
    description: String,
    image_url: String,
}

struct PostResponse {}

struct BlogPosts {
    posts: HashMap<Uuid, Post>,
}

enum AppError {
    // #[error("Resourse not found")]
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
// async fn create_post(post: Post, author_id: u8) -> Result<&Post, String> {
//     let new_post = Post {
//         id: Uuid::new_v4(),
//         title: post.title,
//         description: post.description,
//         content: post.content,
//         created_at: Local::now(),
//         updated_at: Local::now(),
//         author_id,
//         image_url: String::from("https://placehold.net/400x400.png"),
//     };

//     self.posts.insert(new_post.id, new_post);

//     self.next_id += 1;
//     self.posts
//         .get(&(self.next_id - 1))
//         .ok_or(format!("unable to get newly created post"))
// }

// async fn create_post(state: State<BlogPosts>, Json<PostRequest>) -> Result<Json<PostResponse>, AppError> {
//     let new_post = Post {
//         id: Uuid::new_v4(),
//         title: post.title,
//         description: post.description,
//         content: post.content,
//         created_at: Local::now(),
//         updated_at: Local::now(),
//         author_id,
//         image_url: String::from("https://placehold.net/400x400.png"),
//     };

//     self.posts.insert(new_post.id, new_post);

//     self.next_id += 1;
//     self.posts
//         .get(&(self.next_id - 1))
//         .ok_or(format!("unable to get newly created post"))
// }

// fn get_all_posts(&self) -> Vec<&Post> {
//     self.posts.values().collect()
// }
// impl BlogPosts {
//     fn new() -> Self {
//         Self {
//             posts: HashMap::new(),
//         }
//     }
// }

fn main() {
    println!("Hello, world!");

    let id = Uuid::new_v4();

    println!("the generated ID is: {id}")
}

// // POST /posts - needs all post data
// struct CreatePostRequest {
//     title: String,
//     content: String,
//     description: String,
// }

// // PUT /posts/{id} - needs ID + updated data
// struct UpdatePostRequest {
//     title: Option<String>,  // Optional fields for partial updates
//     content: Option<String>,
//     description: Option<String>,
// }

// // GET /posts/search - needs search parameters
// struct SearchPostsRequest {
//     query: String,
//     limit: Option<u32>,
// }
