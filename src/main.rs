use actix_files::NamedFile;
use simple_logger::SimpleLogger;
use log::*;
use actix_web::{App, HttpRequest, HttpServer, get};

#[get("/")]
async fn index(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("./index.html")?)
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    SimpleLogger::new().init().unwrap();
    trace!("Started logger");

    HttpServer::new(move || {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:3030")?
        .run()
        .await?;

    return Ok(());
}
