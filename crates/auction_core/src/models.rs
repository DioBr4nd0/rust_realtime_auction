use chrono::{DateTime, Utc};

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

impl Item {
    pub fn new(id:String, name:String, start_price:f64, status:ItemStatus, created_at:DateTime<Utc>, updated_at:DateTime<Utc>) -> Self {
        Item { id, name, start_price, status, created_at, updated_at }
    }
}