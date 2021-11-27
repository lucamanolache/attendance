use simple_logger::SimpleLogger;
use log::*;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    SimpleLogger::new().init().unwrap();
    trace!("Started logger");

    Ok(())
}
