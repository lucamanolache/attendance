mod schema;
mod stats;
mod forms;

use std::collections::HashMap;
use std::env;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use chrono::{Local, NaiveDate};
use futures::stream::StreamExt;
use log::*;
use mongodb::{bson::doc, options::ClientOptions, Client};

extern crate pretty_env_logger;

use crate::forms::{AddStudentRequest, CorrectionRequest, DataPoint, Graph, LoginRequest, LoginResponse, SlackRequest, StatsResponse, StudentResponse};
use crate::schema::student::Student;

const DATABASE: &str = "attendance";
const COLLECTION: &str = "people";
const TIME_LIMIT: i64 = 43200; // 12 hours

struct AppState {
    client: Client,
}

#[post("/api/echo")]
async fn echo(data: String) -> HttpResponse {
    info!("Echo {}", data);
    HttpResponse::Ok().body(data)
}

#[get("/api/get_all")]
async fn get_leaderboard(state: web::Data<AppState>) -> HttpResponse {
    info!("Getting leaderboard");
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    let students = collection.find(doc! {}, None).await.unwrap();
    let mut students: Vec<StudentResponse> = students
        .map(|x| {
            let x = x.unwrap();
            StudentResponse {
                id: x.id,
                subteam: x.subteam,
                name: x.name,
                total_time: x.valid_time / 3600,
            }
        })
        .collect()
        .await;
    students.sort_by(|a, b| b.total_time.cmp(&a.total_time));

    HttpResponse::Ok().body(serde_json::to_string(&students).unwrap())
}

#[get("/api/get_here")]
async fn get_students(state: web::Data<AppState>) -> HttpResponse {
    info!("Getting students at lab");
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    let students = collection
        .find(doc! {"login_status": {"$ne": Option::<String>::None}}, None)
        .await
        .unwrap();
    let students = students
        .map(|x| {
            let x = x.unwrap();
            AddStudentRequest {
                id: x.id,
                subteam: x.subteam,
                name: x.name,
            }
        })
        .collect::<Vec<AddStudentRequest>>()
        .await;

    HttpResponse::Ok().body(serde_json::to_string(&students).unwrap())
}

#[get("/api/get_stats")]
async fn get_stats(state: web::Data<AppState>) -> HttpResponse {
    info!("Getting stats");
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    let students = collection.find(doc! {}, None).await.unwrap();
    let students: Vec<Student> = students.map(|x| x.unwrap()).collect().await;

    let mut subteams: HashMap<String, (HashMap<NaiveDate, f64>, u32)> = HashMap::new();
    students.iter().for_each(|x| {
        if !subteams.contains_key(&*x.subteam) {
            subteams.insert(x.subteam.clone(), (HashMap::new(), 0));
        }
        let (subteam_map, count) = subteams.get_mut(&*x.subteam).unwrap();
        *count += 1;

        x.events.iter().for_each(|e| {
            let time = match e.1 {
                None => chrono::Duration::seconds(0),
                Some(t) => t - e.0
            };
            match subteam_map.get_mut(&e.0.naive_local().date()) {
                None => {
                    subteam_map.insert(e.0.naive_local().date(), time.num_minutes() as f64 / 60.0);
                }
                Some(_) => {
                    *subteam_map
                        .get_mut(&e.0.naive_local().date().clone())
                        .unwrap() += time.num_minutes() as f64;
                }
            }
        })
    });

    let mut response = StatsResponse::default();
    subteams.iter().for_each(|x| {
        let mut graph = Graph::default();
        graph.id = x.0.clone();
        x.1 .0.iter().for_each(|e| {
            graph.data.push(DataPoint {
                x: *e.0,
                y: e.1 / x.1 .1 as f64,
            })
        });
        graph.data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        response.hours_time.push(graph);
    });

    HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
}

#[post("/api/login")]
async fn login_request(
    form: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let mut session = state.client.start_session(None).await.unwrap();

    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);
    match collection
        .find_one_with_session(doc! {"id": form.id}, None, &mut session)
        .await
        .unwrap()
    {
        Some(mut student) => {
            // Student has been found
            info!("Found student {}", form.id);
            let mut leaving = false;
            let mut time_spent = 0;
            if student.login_status.is_some() {
                // We are currently at lab, therefore log out and add an event
                let event = (student.login_status.unwrap(), Some(Local::now()));
                student.valid_time += time_spent;
                if time_spent >= TIME_LIMIT {
                    student.events.push((event.0, None));
                    warn!("Student {} has passed the time limit", form.id);
                } else {
                    time_spent = (event.1.unwrap() - event.0).num_seconds();
                    student.events.push(event);
                }
                info!(
                    "Logging {} out at {} with {} minutes at lab",
                    student.name,
                    Local::now(),
                    time_spent
                );
                student.login_status = None;
                leaving = true;
            } else {
                // We are just signing into lab, therefore just log in and do not add an event
                student.login_status = Some(Local::now());
                info!("Logging {} in at {}", &student.name, Local::now());
            }
            let name = student.name.clone();
            collection
                .replace_one_with_session(doc! {"id": form.id}, student, None, &mut session)
                .await
                .unwrap();

            HttpResponse::Ok().body(
                serde_json::to_string(&LoginResponse {
                    leaving,
                    time_spent,
                    name,
                })
                .unwrap(),
            )
        }
        None => {
            // Student was not found in database
            warn!("Student {} not found", form.id);
            HttpResponse::NotFound().body("")
        }
    }
}

#[post("/slack")]
async fn slack_rtm(body: web::Form<SlackRequest>, state: web::Data<AppState>) -> HttpResponse {
    info!(
        "Slack is requesting information on {} ({})",
        body.0.user_name, body.0.user_id
    );

    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);
    match collection
        .find_one(doc! {"slack_id": body.0.user_id}, None)
        .await
        .unwrap()
    {
        Some(student) => {
            // Student has been found
            info!("Found student information");
            HttpResponse::Ok().body(format!(
                "You have {:.2} hours",
                student.valid_time as f64 / 3600.0
            ))
        }
        None => {
            // Student was not found in database
            warn!("Student not found");
            HttpResponse::Ok()
                .body("Uh oh, an issue has been spotted. Please message @lmanolache for assistance")
        }
    }
}

#[get("/api/needs_corrections")]
async fn get_corrections(_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

#[post("/api/correction")]
async fn correction(_form: web::Json<CorrectionRequest>,
                    _state: web::Data<AppState>) -> HttpResponse {


    HttpResponse::Ok().body("")
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    pretty_env_logger::init();
    trace!("Started logger");

    let client = get_client().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                client: client.clone(),
            })
            .service(login_request)
            .service(get_leaderboard)
            .service(get_students)
            .service(get_stats)
            .service(echo)
            .service(slack_rtm)
            .service(fs::Files::new("/", "./static/build").index_file("index.html"))
    })
    .bind("0.0.0.0:".to_owned() + &env::var("PORT").unwrap_or("8080".to_owned()))?
    .run()
    .await?;

    Ok(())
}
