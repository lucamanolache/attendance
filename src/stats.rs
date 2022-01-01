use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct StatsResponse {
    pub hours_time: Vec<Graph>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Graph {
    pub label: String,
    pub data: Vec<DataPoint>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DataPoint {
    pub date: NaiveDate,
    pub value: f64,
}

impl Default for DataPoint {
    fn default() -> Self {
        DataPoint {
            date: NaiveDate::from_ymd(1, 1, 1),
            value: 0.0,
        }
    }
}
