pub struct AppState {
    auction_service: std::sync::Arc<dyn auction_core::services::AuctionService>
    // notification_service:std::sync::Arc<dyn auction_core::services::NoticationService>
}