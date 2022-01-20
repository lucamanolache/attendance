use std::collections::HashMap;
use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Local, NaiveDate};
use futures::stream::StreamExt;
use log::*;

use crate::{AppState, COLLECTION, DATABASE, TIME_LIMIT};
use mongodb::{bson::doc, options::ClientOptions, Client};

extern crate pretty_env_logger;

use crate::forms::{
    AddStudentRequest, AllCorrections, CorrectionRequest, DataPoint, Graph, LoginRequest,
    LoginResponse, SlackRequest, StatsResponse, StudentResponse,
};
use crate::schema::student::Student;

#[post("/api/echo")]
pub async fn echo(data: String) -> HttpResponse {
    /// Debugging request. Returns exactly what was sent.
    info!("Echo {}", data);
    HttpResponse::Ok().body(data)
}

#[get("/api/get_all")]
pub async fn get_leaderboard(state: web::Data<AppState>) -> HttpResponse {
    /// Returns all students in the database ordered by time at lab.
    info!("Getting leaderboard");
    // Get student collection
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    // Find all students
    let students = collection.find(doc! {}, None).await.unwrap();
    let mut students: Vec<StudentResponse> = students
        .map(|x| {
            let x = x.unwrap();
            // Turn students into a StudentResponse
            StudentResponse {
                id: x.id,
                subteam: x.subteam,
                name: x.name,
                total_time: x.valid_time / 3600,
            }
        })
        .collect()
        .await;
    // Sort by time at lab
    students.sort_by(|a, b| b.total_time.cmp(&a.total_time));

    // Return json array
    HttpResponse::Ok().body(serde_json::to_string(&students).unwrap())
}

#[get("/api/get_here")]
pub async fn get_students(state: web::Data<AppState>) -> HttpResponse {
    /// Returns all students in the database who are here today.
    info!("Getting students at lab");
    // Collection of students
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    // Get all students who are at lab
    let students = collection
        // Filters those who have login_status as non null
        .find(doc! {"login_status": {"$ne": Option::<String>::None}}, None)
        .await
        .unwrap();
    let students = students
        .map(|x| {
            let x = x.unwrap();
            // Turn students into a AddStudentRequest (because this is the format that needs to be returned)
            AddStudentRequest {
                id: x.id,
                subteam: x.subteam,
                name: x.name,
            }
        })
        .collect::<Vec<AddStudentRequest>>()
        .await;

    // Return json array
    HttpResponse::Ok().body(serde_json::to_string(&students).unwrap())
}

#[get("/api/get_stats")]
pub async fn get_stats(state: web::Data<AppState>) -> HttpResponse {
    /// Returns all students statistics.
    info!("Getting stats");
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    // Get all students from the database
    let students = collection.find(doc! {}, None).await.unwrap();
    let students: Vec<Student> = students.map(|x| x.unwrap()).collect().await;

    // Create a HashMap of subteam name to students
    let mut subteams: HashMap<String, (HashMap<NaiveDate, f64>, u32)> = HashMap::new();
    students.iter().for_each(|x| {
        // Reformat the order of the data in a scuffed way
        if !subteams.contains_key(&*x.subteam) {
            subteams.insert(x.subteam.clone(), (HashMap::new(), 0));
        }
        let (subteam_map, count) = subteams.get_mut(&*x.subteam).unwrap();
        *count += 1;

        x.events.iter().for_each(|e| {
            let time = match e.1 {
                None => chrono::Duration::seconds(0),
                Some(t) => t - e.0,
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

    // More scuffed reformatting
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
pub async fn login_request(
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
                time_spent = (event.1.unwrap() - event.0).num_seconds();
                if time_spent >= TIME_LIMIT {
                    time_spent = 0;
                    student.events.push((event.0, None));
                    warn!("Student {} has passed the time limit", form.id);
                } else {
                    student.events.push(event);
                }
                student.valid_time += time_spent;
                info!(
                    "Logging {} out at {} with {} seconds at lab",
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
pub async fn slack_rtm(body: web::Form<SlackRequest>, state: web::Data<AppState>) -> HttpResponse {
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
pub async fn get_corrections(state: web::Data<AppState>) -> HttpResponse {
    info!("Getting students who need corrections");
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);

    let students = collection.find(doc! {}, None).await.unwrap();
    let students: Vec<AllCorrections> = students
        .collect::<Vec<Result<Student, mongodb::error::Error>>>()
        .await
        .iter()
        .filter_map(|student| match student {
            Ok(s) => Some(s),
            Err(_) => None,
        })
        .filter_map(|student| {
            let student = student.clone();
            let misses = student
                .events
                .iter()
                .filter(|event| event.1.is_none())
                .map(|event| event.0)
                .collect::<Vec<DateTime<Local>>>();

            if misses.is_empty() {
                None
            } else {
                Some(
                    misses
                        .iter()
                        .map(|s| AllCorrections {
                            id: student.id,
                            name: student.name.clone(),
                            login_time: *s,
                        })
                        .collect::<Vec<AllCorrections>>(),
                )
            }
        })
        .flatten()
        .collect();

    HttpResponse::Ok().body(serde_json::to_string(&students).unwrap())
}

#[post("/api/correction")]
pub async fn correction(
    form: web::Json<CorrectionRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    info!("Requesting to correct {}", form.id);
    let collection = state
        .client
        .database(DATABASE)
        .collection::<Student>(COLLECTION);
    let student = collection
        .find_one(doc! {"id": form.id}, None)
        .await
        .unwrap();
    match student {
        None => {
            warn!("Student {} not found!", form.id);
            HttpResponse::NotFound().body("")
        }
        Some(mut student) => {
            let mut needed_time = 0;
            student.events.iter_mut().for_each(|mut event| {
                if event.0 == form.login_time {
                    event.1 = Some(form.logout_time);
                    needed_time += (form.logout_time - form.login_time).num_seconds() as i64;
                    info!("Corrected {} with {} new seconds", form.id, needed_time);
                }
            });
            if needed_time == 0 {
                warn!("Student {} has no time to correct", form.id);
            }
            student.valid_time += needed_time;

            collection
                .replace_one(doc! {"id": form.id}, student, None)
                .await
                .unwrap();
            HttpResponse::Ok().body("")
        }
    }
}
