use axum::{extract::{Json, State}, http::{header, StatusCode}, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{domain::{interface::country_repository::CountryRepository, models::country::CountryCreate}, infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository};


pub async fn insert_country_command(
    State(pg_pool): State<PgPool>,
    Json(country): Json<CountryCreate>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.insert_country(&pg_pool, country).await {
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
