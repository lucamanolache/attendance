use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub valid_time: i64,
    pub events: Vec<(DateTime<Local>, Option<DateTime<Local>>)>,
    pub login_status: Option<DateTime<Local>>,
    pub subteam: String,
    pub slack_id: String,
}
