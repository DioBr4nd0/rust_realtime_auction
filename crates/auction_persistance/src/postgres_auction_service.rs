use anyhow::Ok;
use auction_core::models::{ Item, ItemId };

pub struct PostgresAuctionService {
    pool: sqlx::PgPool,
}

impl PostgresAuctionService {
    fn new(pool: sqlx::PgPool) -> Self {
        PostgresAuctionService { pool: pool }
    }
}

#[async_trait::async_trait]
impl auction_core::services::AuctionService for PostgresAuctionService {
    async fn get_item_by_id(&self, item_id: ItemId) -> Result<Option<Item>, anyhow::Error> {
        let item = Item::new (String::from("value"), "name".to_string(),f64::default(),auction_core::models::ItemStatus::Closed,chrono::Utc::now(),chrono::Utc::now());
        Ok(Some(item))
    }

    async fn check_db_connection(&self) -> Result<(), anyhow::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
