use chrono::NaiveDate;
use serde::Serialize;

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
