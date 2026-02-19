use crate::BlogPosts;
use std::sync::Arc;
use tokio::sync::Mutex;

type SharedAppState = Arc<Mutex<BlogPosts>>;
struct AppState {
    pub post_state: SharedAppState,
}

type CustomString = String;
