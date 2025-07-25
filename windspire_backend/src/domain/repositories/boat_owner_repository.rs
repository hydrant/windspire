use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::boat::Boat;
use crate::domain::models::boat_owner::{BoatWithOwners, UserWithBoats};
use crate::domain::models::user::UserWithCountry;

pub struct BoatOwnerRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> BoatOwnerRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_owner_to_boat(&self, boat_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO boat_owners (boat_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            boat_id,
            user_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_owner_from_boat(
        &self,
        boat_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM boat_owners WHERE boat_id = $1 AND user_id = $2",
            boat_id,
            user_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_boats_for_user(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT boat_id FROM boat_owners WHERE user_id = $1",
            user_id
        )
        .fetch_all(self.pool)
        .await?;
        Ok(records.into_iter().map(|r| r.boat_id).collect())
    }

    pub async fn get_owners_for_boat(&self, boat_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT user_id FROM boat_owners WHERE boat_id = $1",
            boat_id
        )
        .fetch_all(self.pool)
        .await?;
        Ok(records.into_iter().map(|r| r.user_id).collect())
    }

    pub async fn get_boats_with_details_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Boat>, sqlx::Error> {
        let boats = sqlx::query_as!(
            Boat,
            r#"
            SELECT b.id, b.name, b.brand, b.model, b.sail_number, b.country_id
            FROM boats b
            INNER JOIN boat_owners bo ON b.id = bo.boat_id
            WHERE bo.user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await?;
        Ok(boats)
    }

    pub async fn get_owners_with_details_for_boat(
        &self,
        boat_id: Uuid,
    ) -> Result<Vec<UserWithCountry>, sqlx::Error> {
        let owners = sqlx::query_as!(
            UserWithCountry,
            r#"
            SELECT u.id, u.first_name, u.last_name, u.email, u.phone, u.country_id,
                   c.iso_name, u.provider_id, u.provider_name, u.avatar_url
            FROM users u
            INNER JOIN boat_owners bo ON u.id = bo.user_id
            LEFT JOIN countries c ON u.country_id = c.id
            WHERE bo.boat_id = $1
            "#,
            boat_id
        )
        .fetch_all(self.pool)
        .await?;
        Ok(owners)
    }

    pub async fn get_boat_with_owners(
        &self,
        boat_id: Uuid,
    ) -> Result<Option<BoatWithOwners>, sqlx::Error> {
        // First get the boat
        let boat = sqlx::query_as!(
            Boat,
            "SELECT id, name, brand, model, sail_number, country_id FROM boats WHERE id = $1",
            boat_id
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(boat) = boat {
            // Then get the owners
            let owners = self.get_owners_with_details_for_boat(boat_id).await?;
            Ok(Some(BoatWithOwners { boat, owners }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_with_boats(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserWithBoats>, sqlx::Error> {
        // First get the user
        let user = sqlx::query_as!(
            UserWithCountry,
            r#"
            SELECT u.id, u.first_name, u.last_name, u.email, u.phone, u.country_id,
                   c.iso_name, u.provider_id, u.provider_name, u.avatar_url
            FROM users u
            LEFT JOIN countries c ON u.country_id = c.id
            WHERE u.id = $1
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(user) = user {
            // Then get the boats
            let boats = self.get_boats_with_details_for_user(user_id).await?;
            Ok(Some(UserWithBoats { user, boats }))
        } else {
            Ok(None)
        }
    }
}
