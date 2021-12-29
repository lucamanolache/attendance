use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddStudentRequest {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) subteam: String,
}
