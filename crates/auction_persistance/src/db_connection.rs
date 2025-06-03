use sqlx;
pub async fn create_db_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error>{
    let pool = sqlx::postgres::PgPoolOptions::new().max_connections(5).connect(database_url).await?;
    Ok(pool)
}