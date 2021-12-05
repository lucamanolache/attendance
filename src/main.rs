mod schema;
mod login;

use std::env;

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, get, post, web};
use chrono::Utc;
use log::*;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use simple_logger::SimpleLogger;

use crate::schema::student::Student;

struct AppState {
    client: Client,
}

#[post("/api/login")]
async fn login_request(form: web::Json<login::LoginRequest>, state: web::Data<AppState>) -> HttpResponse {
    info!("Login request {:?}", &form);

    let mut session = state.client.start_session(None).await.unwrap();

    let collection = state.client.database("").collection::<Student>("people");
    let mut student = collection.find_one_with_session(doc! {"id": form.id}, None, &mut session).await.unwrap().unwrap();

    if student.login_status.is_some() {
        // We are currently at lab, therefore log out and add an event
        let event = (student.login_status.unwrap(), Utc::now());
        student.valid_time = (event.1 - event.0).num_seconds();
        student.events.push(event);
        student.login_status = None;
    } else {
        // We are just signing into lab, therefore just log in and do not add an event
        student.login_status = Some(Utc::now());
    }

    collection.replace_one_with_session(doc! {"id": form.id}, student, None, &mut session).await.unwrap();

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

    HttpServer::new(move || App::new().data(AppState { client: client.clone() }).service(index))
        .bind("127.0.0.1:3030")?
        .run()
        .await?;

    return Ok(());
}
