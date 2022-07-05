use crate::error::Result;
use crate::models::user::User;
use axum::{extract::Query, Json};
use tracing::info;
use validator::Validate;

#[tracing::instrument(skip(user),fields(user.name = %user.name(),user.email = %user.email()))]
pub async fn user_info(Query(user): Query<User>) -> Result<Json<User>> {
    info!(message = "req info", ?user);
    user.validate()?;
    Ok(Json(user))
}
