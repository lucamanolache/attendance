use actix_files as fs;

use actix_web::{App, HttpServer};
use log::*;
use std::env;

use attendance_system::handlers::*;
use attendance_system::AppState;

extern crate pretty_env_logger;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    pretty_env_logger::init();
    trace!("Started logger");

    HttpServer::new(move || {
        App::new()
            .data(AppState::new())
            .service(login_request)
            .service(get_leaderboard)
            .service(get_students)
            .service(get_stats)
            .service(echo)
            .service(slack_rtm)
            .service(get_corrections)
            .service(correction)
            .service(fs::Files::new("/", "./static/build").index_file("index.html"))
    })
    .bind("0.0.0.0:".to_owned() + &env::var("PORT").unwrap_or("8080".to_owned()))?
    .run()
    .await?;

    Ok(())
}
