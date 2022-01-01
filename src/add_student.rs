use serde::{Deserialize, Serialize};

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
