use crate::{db::DbPool, models::short_link::ShortLink};
use sqlx::{mysql::MySqlQueryResult, Error};

pub async fn create_shortlink(pool: &DbPool, url: &str) -> Result<MySqlQueryResult, Error> {
    let res = sqlx::query!(
        r#"
            INSERT INTO short_links (`url`)
            VALUES(?)"#,
        url
    )
    .execute(pool)
    .await?;

    Ok(res)
}

pub async fn delete_shortlink(pool: &DbPool, id: u32) -> Result<MySqlQueryResult, Error> {
    let res = sqlx::query!(
        r#"
            DELETE FROM short_links
            WHERE id = ?
            "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(res)
}

pub async fn get_shortlink(pool: &DbPool, id: u32) -> Result<ShortLink, Error> {
    let res = sqlx::query_as!(
        ShortLink,
        r#"
            SELECT id as `id:u32`,url FROM short_links
            WHERE id = ?
            "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(res)
}
