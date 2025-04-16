use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_country_by_id_query(State(pg_pool): State<PgPool>, Path(country_id): Path<Uuid>) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_country_by_id(&pg_pool, country_id).await {
        Ok(users) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : true, "data" : users }).to_string(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : false, "message" : e.to_string() }).to_string(),
        ),
    }
}
