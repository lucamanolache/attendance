use serde::{Serialize, Deserialize};

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct LoginRequest {
    pub(crate) id: u32,
}

#[derive(Serialize, Clone)]
pub struct LoginResponse {
    pub(crate) leaving: bool,
    pub(crate) name: String,
}
