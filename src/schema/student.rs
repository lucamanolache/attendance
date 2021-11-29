use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub valid_time: u32,
    pub events: Vec<[DateTime<Utc>; 2]>,
}
