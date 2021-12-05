use serde::{Serialize, Deserialize};

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct LoginRequest {
    pub(crate) id: Option<u32>,
}

#[derive(Serialize, Clone)]
pub struct LoginResponse {
    pub(crate) leaving: bool,
    pub(crate) valid: bool,
    pub(crate) name: String,
}
