use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use once_cell::sync::Lazy;

static REGEX_EMAIL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9_'^&+/=?`{|}~.-]+)@(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}$").unwrap()
});

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UserCreate {
    #[validate(length(min = 2, message = "First name must contain 2 at least characters"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "First name must contain 1 at least characters"))]
    pub last_name: String,
    // #[validate(email)] - does not work
    #[validate(regex(path = *REGEX_EMAIL, message = "Email has incorrect format"))]
    pub email: String,
    #[validate(length(min = 3, message = "Phone must contain 3 at least characters"))]
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
