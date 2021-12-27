use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct AddStudentRequest {
    pub(crate) id: u32,
    pub(crate) name: String,
}
