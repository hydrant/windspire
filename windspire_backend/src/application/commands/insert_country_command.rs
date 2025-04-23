use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    application::common::http_reponse::json_response,
    domain::{interface::country_repository::CountryRepository, models::country::CountryCreate},
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn insert_country_command(
    State(pg_pool): State<PgPool>,
    Json(country_create): Json<CountryCreate>,
) -> impl IntoResponse {
    // Validate the country_create data
    match country_create.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };
    let repository = SqlxCountryRepository;
    match repository.insert_country(&pg_pool, country_create).await {
        Ok(country) => json_response(StatusCode::OK, json!({ "success": true, "data": country })),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }),
        ),
    }
}
