use thiserror::Error;
#[derive(Error, Debug)]
pub enum AuctionError {
    #[error("failed to do something on databse")]
    DatabaseError,
    #[error("Auction Object not found in Database")]
    ItemNotFound,
    #[error("invalid input for Auction Item")]
    InvalidInput,
}

