use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Country {
    pub id: Uuid,
    pub iso_name: String,
    pub iso_alpha_2: String,
    pub iso_alpha_3: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CountryCreate {
    pub iso_name: String,
    #[validate(length(equal = 2, message = "ISO Alpha-2 code must be 2 characters"))]
    pub iso_alpha_2: String,
    #[validate(length(equal = 3, message = "ISO Alpha-3 code must be 3 characters"))]
    pub iso_alpha_3: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountryUpdate {
    pub iso_name: String,
    pub iso_alpha_2: String,
    pub iso_alpha_3: String,
}
