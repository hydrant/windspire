use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    interface::boat_repository::BoatRepository,
    models::boat::{Boat, BoatCreate, BoatUpdate},
};

pub struct SqlxBoatRepository;

impl BoatRepository for SqlxBoatRepository {
    async fn get_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Boat, Error> {
        todo!()
    }

    async fn get_all(&self, pool: &PgPool) -> Result<Vec<Boat>, Error> {
        todo!()
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
        todo!()
    }

    async fn update(&self, pool: &PgPool, id: Uuid, data: BoatUpdate) -> Result<Boat, Error> {
        todo!()
    }
}
