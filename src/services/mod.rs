use axum::{routing::get, Router};

pub mod user;

pub fn router() -> Router {
    Router::new().route("/user", get(user::user_info))
}
