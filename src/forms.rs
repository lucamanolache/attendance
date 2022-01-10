use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct LoginRequest {
    pub(crate) id: u32,
}

#[derive(Serialize, Clone)]
pub struct LoginResponse {
    pub(crate) leaving: bool,
    pub(crate) name: String,
    pub(crate) time_spent: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SlackRequest {
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct StatsResponse {
    pub hours_time: Vec<Graph>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Graph {
    pub id: String,
    pub data: Vec<DataPoint>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DataPoint {
    pub x: NaiveDate,
    pub y: f64,
}

impl Default for DataPoint {
    fn default() -> Self {
        DataPoint {
            x: NaiveDate::from_ymd(1, 1, 1),
            y: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddStudentRequest {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) subteam: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StudentResponse {
    pub id: u32,
    pub total_time: i64,
    pub name: String,
    pub subteam: String,
}
