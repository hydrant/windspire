use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::boat::Boat;
use crate::domain::models::user::UserWithCountry;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BoatOwner {
    pub boat_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoatWithOwners {
    pub boat: Boat,
    pub owners: Vec<UserWithCountry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithBoats {
    pub user: UserWithCountry,
    pub boats: Vec<Boat>,
}
