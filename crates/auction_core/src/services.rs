use crate::models::Item;
use crate::models::ItemId;
#[async_trait::async_trait]
pub trait AuctionService: Send + Sync {
    async fn get_item_by_id(&self, item_id:ItemId) -> Result<Option<Item>, anyhow::Error>;
    async fn check_db_connection(&self) -> Result<(),anyhow::Error>;
}