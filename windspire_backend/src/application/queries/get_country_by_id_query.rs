use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::common::http_reponse::json_response,
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_country_by_id_query(
    State(pg_pool): State<PgPool>,
    Path(country_id): Path<Uuid>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_country_by_id(&pg_pool, country_id).await {
        Ok(country) => json_response(
            StatusCode::OK,
            json!({ "success" : true, "data" : country }),
        ),
        Err(sqlx::Error::RowNotFound) => json_response(
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": "Country not found" }),
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success" : false, "message" : e.to_string() }),
        ),
    }
}
