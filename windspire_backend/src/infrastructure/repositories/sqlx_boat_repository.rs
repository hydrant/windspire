use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    interface::boat_repository::{BoatRepository, PaginatedResult, PaginationParams},
    models::boat::{Boat, BoatCreate, BoatUpdate},
    models::boat_owner::BoatWithOwners,
    models::user::UserWithCountry,
};

pub struct SqlxBoatRepository;

impl BoatRepository for SqlxBoatRepository {
    async fn get_by_id(&self, _pool: &PgPool, _id: Uuid) -> Result<Boat, Error> {
        todo!()
    }

    async fn get_all(&self, pool: &PgPool) -> Result<Vec<Boat>, Error> {
        let boats = sqlx::query_as!(
            Boat,
            r#"
            SELECT id, name, brand, model, sail_number, country_id
            FROM boats
            ORDER BY name ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(boats)
    }

    async fn get_paginated(
        &self,
        pool: &PgPool,
        params: PaginationParams,
    ) -> Result<PaginatedResult<Boat>, Error> {
        let offset = (params.page - 1) * params.limit;

        // Get total count
        let total_result = sqlx::query!("SELECT COUNT(*) as count FROM boats")
            .fetch_one(pool)
            .await?;

        let total = total_result.count.unwrap_or(0);

        // Get paginated boats
        let boats = sqlx::query_as!(
            Boat,
            r#"
            SELECT id, name, brand, model, sail_number, country_id
            FROM boats
            ORDER BY name ASC
            LIMIT $1 OFFSET $2
            "#,
            params.limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        let total_pages = ((total as f64) / (params.limit as f64)).ceil() as u32;

        Ok(PaginatedResult {
            data: boats,
            total,
            page: params.page,
            limit: params.limit,
            total_pages,
        })
    }

    async fn insert(&self, pool: &PgPool, data: BoatCreate) -> Result<Boat, Error> {
        // Generate UUID v7 id
        let ts = Timestamp::now(&NoContext);
        let id = Uuid::new_v7(ts);
        let user = sqlx::query_as!(
            Boat,
            r#"
            INSERT INTO boats (id, name, brand, model, sail_number, country_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, brand, model, sail_number, country_id
            "#,
            id,
            data.name,
            data.brand,
            data.model,
            data.sail_number,
            data.country_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn delete(&self, pool: &PgPool, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM boats WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn update(&self, pool: &PgPool, id: Uuid, data: BoatUpdate) -> Result<Boat, Error> {
        let boat = sqlx::query_as!(
            Boat,
            r#"
            UPDATE boats
            SET name = $2, brand = $3, model = $4, sail_number = $5, country_id = $6
            WHERE id = $1
            RETURNING id, name, brand, model, sail_number, country_id
            "#,
            id,
            data.name,
            data.brand,
            data.model,
            data.sail_number,
            data.country_id
        )
        .fetch_one(pool)
        .await?;

        Ok(boat)
    }

    async fn get_paginated_with_owners(
        &self,
        pool: &PgPool,
        params: PaginationParams,
    ) -> Result<PaginatedResult<BoatWithOwners>, Error> {
        let offset = (params.page - 1) * params.limit;

        // Get total count
        let total_result = sqlx::query!("SELECT COUNT(*) as count FROM boats")
            .fetch_one(pool)
            .await?;

        let total = total_result.count.unwrap_or(0);

        // Get paginated boats
        let boats = sqlx::query_as!(
            Boat,
            r#"
            SELECT id, name, brand, model, sail_number, country_id
            FROM boats
            ORDER BY name ASC
            LIMIT $1 OFFSET $2
            "#,
            params.limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        // For each boat, get its owners
        let mut boats_with_owners = Vec::new();
        for boat in boats {
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
                boat.id
            )
            .fetch_all(pool)
            .await?;

            boats_with_owners.push(BoatWithOwners { boat, owners });
        }

        let total_pages = ((total as f64) / (params.limit as f64)).ceil() as u32;

        Ok(PaginatedResult {
            data: boats_with_owners,
            total,
            page: params.page,
            limit: params.limit,
            total_pages,
        })
    }
}
