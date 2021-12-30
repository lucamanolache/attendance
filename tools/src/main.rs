use std::env;
use chrono::{DateTime, Local};
use clap::{App, Arg};
use mongodb::Client;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

const DATABASE: &str = "attendance";
const COLLECTION: &str = "people";

#[derive(Serialize, Deserialize, Clone)]
struct CSVStudent {
    name: String,
    slack_id: String,
    subteam: String,
    student_id: u32
}

#[derive(Serialize, Deserialize, Clone)]
struct Student {
    pub id: u32,
    pub name: String,
    pub valid_time: i64,
    pub events: Vec<(DateTime<Local>, DateTime<Local>)>,
    pub login_status: Option<DateTime<Local>>,
    pub subteam: String,
    pub slack_id: String
}

impl Into<Student> for CSVStudent {
    fn into(self) -> Student {
        Student {
            id: self.student_id,
            name: self.name,
            valid_time: 0,
            events: vec![],
            login_status: None,
            subteam: self.subteam,
            slack_id: self.slack_id
        }
    }
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    let password = env::var("MONGO_PASSWD").expect("MONGO_PASSWD not set");
    let client_options = ClientOptions::parse(format!(
        "mongodb+srv://luca:{}@cluster0.sgpww.mongodb.net/attendance?retryWrites=true&w=majority",
        password
    ))
        .await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[tokio::main]
async fn main() {
    let matches = App::new("Attendance system tools")
        .version("0.1")
        .author("Luca Manolache")
        .about("Useful utils for the Paly Robotics attendance system")
        .arg(Arg::with_name("students")
            .short("s")
            .long("students")
            .value_name("FILE")
            .help("CSV file with student names, slack ids, subteams, and student ids")
            .takes_value(true))
        .get_matches();

    match matches.value_of("students") {
        None => {}
        Some(file) => {
            let mut reader = csv::Reader::from_path(file).unwrap();
            let mut students: Vec<Student> = Vec::new();
            for student in reader.deserialize() {
                let student: CSVStudent = student.unwrap();
                students.push(student.into());
            }

            let client = get_client().await.unwrap();
            let collection = client
                .database(DATABASE)
                .collection::<Student>(COLLECTION);
            collection.insert_many(students, None).await.unwrap();
        }
    }
}
