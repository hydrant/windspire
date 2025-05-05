use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::models::boat::{Boat, BoatCreate, BoatUpdate};

pub(crate) trait BoatRepository {
    async fn get_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Boat, Error>;

    async fn get_all(&self, pool: &PgPool) -> Result<Vec<Boat>, Error>;

    async fn insert(&self, pool: &PgPool, data: BoatCreate) -> Result<Boat, Error>;
    async fn delete(&self, pool: &PgPool, id: Uuid) -> Result<(), Error>;
    async fn update(&self, pool: &PgPool, id: Uuid, data: BoatUpdate) -> Result<Boat, Error>;
}
