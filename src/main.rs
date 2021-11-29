use std::env;

use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpServer};
use log::*;
use mongodb::{options::ClientOptions, Client, Database};
use simple_logger::SimpleLogger;

struct AppState {
    db: Database,
}

#[get("/")]
async fn index(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("./index.html")?)
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    let password = env::var("MONGO_PASSWD").unwrap();
    let client_options = ClientOptions::parse(format!(
        "mongodb+srv://luca:{}@cluster0.sgpww.mongodb.net/attendance?retryWrites=true&w=majority",
        password
    ))
    .await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    SimpleLogger::new().init().unwrap();
    trace!("Started logger");

    let client = get_client().await.unwrap();
    let db = client.database("people");

    HttpServer::new(move || App::new()
                    .data(AppState { db: db.clone() })
                    .service(index))
        .bind("127.0.0.1:3030")?
        .run()
        .await?;

    return Ok(());
}
