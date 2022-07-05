use axum::{
    debug_handler,
    extract::{Path, Query},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{curd, db::DbPool, models::short_link::ShortLink, Result};
#[derive(Deserialize)]
pub struct CreateShortLink {
    url: String,
}

#[debug_handler]
#[tracing::instrument(skip(pool))]
pub async fn create_shortlink(
    Query(CreateShortLink { url }): Query<CreateShortLink>,
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Value>> {
    let id = curd::short_link::create_shortlink(&pool, &url)
        .await?
        .last_insert_id();
    Ok(Json(serde_json::json!({
        "status":"ok",
        "id":id
    })))
}
#[debug_handler]
#[tracing::instrument(skip(pool))]
pub async fn delete_shortlink(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Value>> {
    let _row_affected = curd::short_link::delete_shortlink(&pool, id)
        .await?
        .rows_affected();
    Ok(Json(serde_json::json!({
        "status":"ok",
        "id":id
    })))
}
#[debug_handler]
#[tracing::instrument(skip(pool))]
pub async fn get_shortlink(
    Path(id): Path<u32>,
    Extension(pool): Extension<DbPool>,
) -> Result<Json<ShortLink>> {
    let short_link = curd::short_link::get_shortlink(&pool, id).await?;
    Ok(Json(short_link))
}
