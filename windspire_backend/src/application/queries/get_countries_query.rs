use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::common::http_reponse::json_response,
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_countries_query(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_countries(&pg_pool).await {
        Ok(countries) => json_response(
            StatusCode::OK,
            json!({ "success" : true, "data" : countries }),
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success" : false, "message" : e.to_string() }),
        ),
    }
}
