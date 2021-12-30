use serde::{Deserialize, Serialize};

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
