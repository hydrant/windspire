use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country_id: Uuid,
}
