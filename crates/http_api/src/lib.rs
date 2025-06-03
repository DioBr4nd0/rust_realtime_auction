use std::env;
pub mod app_state;
pub mod routes;
use dotenvy::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("failed to load the env file");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Database url not found");
    let server_addr = env::var("SERVER_ADDR").expect("server addr not found");
    let pool = auction_persistance::db_connection::create_db_pool(&db_url).await;
    let auction_service_impl = auction_persistance::postgres_auction_service::
    Ok(())
}