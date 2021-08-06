use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: String,
    pub guid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserRequest {
    pub guid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginRequest {
    pub username: String,
}