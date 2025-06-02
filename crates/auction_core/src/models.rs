pub type ItemId = String;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
pub enum ItemStatus {
    Pending,
    Open,
    Closed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Item {
    id: ItemId,
    name: String,
    start_price: f64,
    status: ItemStatus,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}