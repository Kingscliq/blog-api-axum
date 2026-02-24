use axum::{
    Router,
    routing::{get, post},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

use uuid::Uuid;

mod app;
mod config;
mod db;
mod errors;
mod handlers;
mod logger;
mod services;

use errors::AppError;
use handlers::{create_post_handler, get_all_posts, health_handler};
use logger::AppLogger;

use app::AppState;

use config::get_env_vars;
use db::connect_db;
#[derive(Clone, Debug, Serialize)]
struct Post {
    id: Uuid,
    title: String,
    image_url: String,
    content: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// #[derive(Debug, Deserialize)]
// struct UpdatePostRequest {
//     title: Option<String>,
//     description: Option<String>,
//     image_url: Option<String>,
//     content: Option<String>,
// }
#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse<T> {
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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    AppLogger::init();

    let database_url: String = get_env_vars("DATABASE_URL")?;
    let pool = connect_db(&database_url).await.map_err(|err| {
        let msg = match &err {
            sqlx::Error::Database(db_err) => db_err.message().to_string(),
            _ => err.to_string(),
        };
        AppLogger::error(&format!("Database connection failed {}", msg));
        AppError::InternalServer(format!("Database connection failed"))
    })?;

    AppLogger::info(&format!("Database Connected Successfully 🔥🚀"));
    let app_state = AppState::new(pool);
    let default_port = 8080;

    let app: Router = Router::new()
        .route("/", get(health_handler))
        .route("/posts", post(create_post_handler).get(get_all_posts))
        .with_state(app_state);

    let port = get_env_vars("PORT").unwrap_or(default_port);
    let socket_address: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

    AppLogger::info(&format!("Listening at {}", socket_address));
    let listener: TcpListener = TcpListener::bind(socket_address).await.unwrap();

    AppLogger::info(&format!(
        "Server listening  at {}",
        listener.local_addr().unwrap()
    ));
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
