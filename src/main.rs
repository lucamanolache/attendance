mod schema;
mod login;

use std::env;

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, get, post, web};
use log::*;
use mongodb::{options::ClientOptions, Client, Database};
use simple_logger::SimpleLogger;

struct AppState {
    db: Database,
}

#[post("/api/login")]
async fn login_request(form: web::Json<login::LoginRequest>, state: web::Data<AppState>) -> HttpResponse {
    info!("Login request {:?}", &form);
    HttpResponse::Ok().body("a")
}

#[get("/")]
async fn index(req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    info!("{:?}", req);
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
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    trace!("Started logger");

    let client = get_client().await.unwrap();
    let db = client.database("people");

    HttpServer::new(move || App::new().data(AppState { db: db.clone() }).service(index))
        .bind("127.0.0.1:3030")?
        .run()
        .await?;

    return Ok(());
}
