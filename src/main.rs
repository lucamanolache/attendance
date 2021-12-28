mod schema;
mod add_student;
mod login;

use std::{env, process::id};

use actix_files as fs;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, get, post, web};
use chrono::Utc;
use log::*;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use simple_logger::SimpleLogger;

use crate::schema::student::Student;

struct AppState {
    client: Client,
}

#[post("/api/echo")]
async fn echo(data: String) -> HttpResponse {
    info!("Echo {}", data);
    HttpResponse::Ok().body(data)
}

#[post("/api/login")]
async fn login_request(form: web::Json<login::LoginRequest>, state: web::Data<AppState>) -> HttpResponse {
    let mut session = state.client.start_session(None).await.unwrap();

    let collection = state.client.database("attendance").collection::<Student>("people");
    match collection.find_one_with_session(doc! {"id": form.id}, None, &mut session).await.unwrap() {
        Some(mut student) => {
            // Student has been found
            info!("Found student {}", form.id);
            let mut leaving = false;
            if student.login_status.is_some() {
                // We are currently at lab, therefore log out and add an event
                let event = (student.login_status.unwrap(), Utc::now());
                student.valid_time = (event.1 - event.0).num_seconds();
                info!("Logging {} out at {} with {} minutes at lab", student.name, Utc::now(), (event.1 - event.0).num_minutes());
                student.events.push(event);
                student.login_status = None;
                leaving = true;
            } else {
                // We are just signing into lab, therefore just log in and do not add an event
                student.login_status = Some(Utc::now());
                info!("Logging {} in at {}", &student.name, Utc::now());
            }
            let name = student.name.clone();
            collection.replace_one_with_session(doc! {"id": form.id}, student, None, &mut session).await.unwrap();

            HttpResponse::Ok().body(serde_json::to_string(&login::LoginResponse {
                leaving,
                name
            }).unwrap())
        },
        None => {
            // Student was not found in database
            warn!("Student {} not found", form.id);
            HttpResponse::NotFound().body("")
        }
    }

}

#[post("/api/add_students")]
async fn add_students(form: web::Json<add_student::AddStudentRequest>, state: web::Data<AppState>) -> HttpResponse {
    let collection = state.client.database("attendance").collection::<Student>("people");
    let student = Student {
        id: form.id,
        name: form.clone().name,
        valid_time: 0,
        events: Vec::new(),
        login_status: None,
    };

    match collection.insert_one(student, None).await {
        Ok(_) => {
            info!("Adding student {:?}", form);
            HttpResponse::Accepted().body("")
        },
        Err(e) => {
            warn!("Adding student {} failed with {:?}", form.id, e);
            HttpResponse::Conflict().body("")
        }
    }
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

    HttpServer::new(move || App::new().data(AppState { client: client.clone() })
        .service(add_students)
        .service(login_request)
        .service(echo)
        .service(fs::Files::new("/", "./static/build").index_file("index.html")))
        .bind("127.0.0.1:3030")?
        .run()
        .await?;

    return Ok(());
}
