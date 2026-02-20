use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Response, StatusCode, response},
    response::IntoResponse,
    routing::{get, post},
    serve::Listener,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use thiserror::Error;
use tokio::{net::TcpListener, sync::Mutex};

use uuid::Uuid;

mod errors;
mod handlers;
mod logger;

use errors::AppError;
use handlers::create_post_handler;
use logger::AppLogger;

#[derive(Clone, Debug, Serialize)]
struct Post {
    id: Uuid,
    title: String,
    image_url: String,
    content: String,
    description: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
struct UpdatePostRequest {
    title: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiReponse<T> {
    data: T,
    status_code: u16,
    message: String,
}

struct _PaginationMeta {
    page: Option<u16>,
    per_page: Option<u16>,
    total: Option<u16>,
    total_pages: Option<u16>,
}

#[derive(Debug, Serialize)]
struct PostResponse {}
#[derive(Debug)]
struct BlogPosts {
    posts: HashMap<Uuid, Post>,
}

impl BlogPosts {
    fn new() -> Self {
        Self {
            posts: HashMap::new(),
        }
    }
    fn create_post(&mut self, post: Post) -> Result<(), AppError> {
        if self.posts.contains_key(&post.id) {
            Err(AppError::BadRequest(format!(
                "Post with ID: {} already Exists",
                post.id
            )))
        } else {
            self.posts.insert(Uuid::new_v4(), post);

            Ok(())
        }
    }
}
#[derive(Clone, Debug)]
struct AppState {
    post_state: Arc<Mutex<BlogPosts>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            post_state: Arc::new(Mutex::new(BlogPosts::new())),
        }
    }
}

// Handlers

async fn get_all_posts(
    State(app_state): State<AppState>,
) -> Result<Json<ApiReponse<Vec<Post>>>, AppError> {
    let response = app_state
        .post_state
        .lock()
        .await
        .posts
        .values()
        .cloned()
        .collect();

    let res = ApiReponse {
        data: response,
        status_code: StatusCode::OK.as_u16(),
        message: "Post retrieved successfully 🚀".to_string(),
    };

    Ok(Json(res))
}

async fn health_handler() -> Json<String> {
    Json("App sarted running successfully! 🚀🔥".to_string())
}

fn get_env_vars<T>(key: &str) -> Result<T, AppError>
where
    T: FromStr,
{
    let vars = std::env::var(key)
        .map_err(|_| AppError::NotFound(format!("Environment variable {key} not Found")))?;

    Ok(vars
        .parse::<T>()
        .map_err(|_| AppError::BadRequest("An error occured while parsing".to_string())))?
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    AppLogger::init();

    let app_state = AppState::new();
    let default_port = 8080;

    let app: Router = Router::new()
        .route("/", get(health_handler))
        .route("/posts", post(create_post_handler).get(get_all_posts))
        .with_state(app_state);

    let port = get_env_vars("PORT").unwrap_or(default_port);
    let socker_address: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

    AppLogger::info(&format!("Listening at {}", socker_address));
    let listener: TcpListener = TcpListener::bind(socker_address).await.unwrap();

    AppLogger::info(&format!(
        "Server listening  at {}",
        listener.local_addr().unwrap()
    ));
    axum::serve(listener, app).await.unwrap();
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
