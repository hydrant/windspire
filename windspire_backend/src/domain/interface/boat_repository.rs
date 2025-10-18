use anyhow::Result;
use serde::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::models::boat::{Boat, BoatCreate, BoatUpdate};
use crate::domain::models::boat_owner::BoatWithOwners;

#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: u32,
    pub limit: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { page: 1, limit: 20 }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

pub(crate) trait BoatRepository {
    async fn get_paginated(
        &self,
        pool: &PgPool,
        params: PaginationParams,
    ) -> Result<PaginatedResult<Boat>, Error>;

    async fn get_paginated_with_owners(
        &self,
        pool: &PgPool,
        params: PaginationParams,
    ) -> Result<PaginatedResult<BoatWithOwners>, Error>;

    async fn insert(&self, pool: &PgPool, data: BoatCreate) -> Result<Boat, Error>;
    async fn delete(&self, pool: &PgPool, id: Uuid) -> Result<(), Error>;
    async fn update(&self, pool: &PgPool, id: Uuid, data: BoatUpdate) -> Result<Boat, Error>;
}
