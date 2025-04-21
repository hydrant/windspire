use axum::{extract::{Path, State}, http::{header, StatusCode}, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{domain::{interface::country_repository::CountryRepository, models::country::CountryUpdate}, infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository};


pub async fn update_country_command(
    State(pg_pool): State<PgPool>,
    Path(country_id): Path<Uuid>,
    Json(country_update): Json<CountryUpdate>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.update_country(&pg_pool, country_id, country_update).await {
        Ok(users) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : true, "data" : users }).to_string(),
        ),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success": false, "message": "User not found" }).to_string(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : false, "message" : e.to_string() }).to_string(),
        ),
    }
}
