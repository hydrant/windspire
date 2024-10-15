use serde::Serialize;
use uuid::Uuid; // Import the Uuid type

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country_id: Uuid, 
}

#[derive(Serialize)]
pub struct Country {
    pub id: Uuid,
    pub iso_name: String,
    pub iso_alpha_2: String,
    pub iso_alpha_3: String,
}
