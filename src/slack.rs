use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct SlackRequest {
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
}
