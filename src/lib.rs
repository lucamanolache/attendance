use mongodb::options::ClientOptions;
use mongodb::Client;
use std::env;

mod forms;
mod schema;
mod stats;

pub mod handlers;

pub const DATABASE: &str = "attendance";
pub const COLLECTION: &str = "people";
pub const ACCOUNTS: &str = "accounts";
pub const TIME_LIMIT: i64 = 43200; // 12 hours

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}

impl AppState {
    pub async fn new() -> AppState {
        let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
        let client_options = ClientOptions::parse(uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        AppState { client }
    }
}
