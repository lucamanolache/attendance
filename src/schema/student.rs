use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub valid_time: i64,
    pub events: Vec<(DateTime<Utc>, DateTime<Utc>)>,
    pub login_status: Option<DateTime<Utc>>,
    pub subteam: String,
}
