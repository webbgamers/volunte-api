use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Error {
    pub error: String,
}
