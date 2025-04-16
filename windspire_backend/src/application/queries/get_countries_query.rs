use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_countries_query(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_countries(&pg_pool).await {
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
