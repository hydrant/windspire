use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Country {
    pub id: Uuid,
    pub iso_name: String,
    pub iso_alpha_2: String,
    pub iso_alpha_3: String,
}
