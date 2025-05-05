use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

static REGEX_SAIL_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z]{3}\d{1,5}$").unwrap());

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Boat {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub sail_number: Option<String>,
    pub country_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BoatCreate {
    #[validate(length(min = 2, message = "Boat name must contain 2 at least characters"))]
    pub name: String,
    #[validate(length(min = 1, message = "Brand must contain 1 at least characters"))]
    pub brand: Option<String>,
    #[validate(length(min = 1, message = "Model must contain 1 at least characters"))]
    pub model: Option<String>,
    #[validate(regex(path = *REGEX_SAIL_NUMBER, message = "Sail number has incorrect format"))]
    pub sail_number: Option<String>,
    pub country_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BoatUpdate {
    #[validate(length(min = 2, message = "Boat name must contain 2 at least characters"))]
    pub name: String,
    #[validate(length(min = 1, message = "Brand must contain 1 at least characters"))]
    pub brand: Option<String>,
    #[validate(length(min = 1, message = "Model must contain 1 at least characters"))]
    pub model: Option<String>,
    #[validate(regex(path = *REGEX_SAIL_NUMBER, message = "Sail number has incorrect format"))]
    pub sail_number: Option<String>,
    pub country_id: Uuid,
}
