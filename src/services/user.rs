use axum::{extract::Query, Json};
use tracing::info;

use crate::models::user::User;

#[tracing::instrument(skip(user),fields(user.name = %user.name(),user.email = %user.email()))]
pub async fn user_info(Query(user): Query<User>) -> Json<User> {
    info!(message = "req info", ?user);
    Json(user)
}
