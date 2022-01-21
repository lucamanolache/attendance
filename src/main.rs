use actix_files as fs;

use actix_web::{App, HttpServer};
use log::*;
use std::env;
use actix_session::CookieSession;

use attendance_system::handlers::*;
use attendance_system::AppState;

extern crate pretty_env_logger;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    pretty_env_logger::init();
    trace!("Started logger");

    let data = AppState::new().await;

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(data.clone())
            .service(login_request)
            .service(get_leaderboard)
            .service(get_students)
            .service(get_stats)
            .service(echo)
            .service(slack_rtm)
            .service(get_corrections)
            .service(correction)
            .service(get_cookie)
            .service(fs::Files::new("/", "./static/build").index_file("index.html"))
    })
    .bind("0.0.0.0:".to_owned() + &env::var("PORT").unwrap_or("8080".to_owned()))?
    .run()
    .await?;

    Ok(())
}
