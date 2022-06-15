use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct Session {
    pub id: Vec<u8>,
    pub user_id: i64,
}