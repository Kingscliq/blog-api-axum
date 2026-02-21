use std::sync::Arc;

use tokio::sync::Mutex;

use crate::BlogPosts;

#[derive(Clone, Debug)]
pub struct AppState {
    pub post_state: Arc<Mutex<BlogPosts>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            post_state: Arc::new(Mutex::new(BlogPosts::new())),
        }
    }
}
