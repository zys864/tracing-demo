use axum::{
    routing::{delete, get, post},
    Router,
};

use super::short_link_service;

pub fn short_links() -> Router {
    Router::new()
        .route("/", post(short_link_service::create_shortlink))
        .route("/:id", delete(short_link_service::delete_shortlink))
        .route("/:id", get(short_link_service::get_shortlink))
}
