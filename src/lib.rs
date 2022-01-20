use std::env;
use mongodb::Client;
use mongodb::options::ClientOptions;

mod forms;
mod schema;
mod stats;
pub mod handlers;

pub struct AppState {
    pub client: Client,
}

impl AppState {
    pub async fn new() -> AppState {
        let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        AppState {
            client
        }
    }
}
