use crate::{
    application::common::http_reponse::json_response,
    domain::{interface::country_repository::CountryRepository, models::country::CountryUpdate},
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub async fn update_country_command(
    State(pg_pool): State<PgPool>,
    Path(country_id): Path<Uuid>,
    Json(country_update): Json<CountryUpdate>,
) -> impl IntoResponse {
    // Validate the country_update data
    match country_update.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };

    let repository = SqlxCountryRepository;
    match repository
        .update_country(&pg_pool, country_id, country_update)
        .await
    {
        Ok(country) => json_response(StatusCode::OK, json!({ "success": true, "data": country })),
        Err(sqlx::Error::RowNotFound) => json_response(
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": "Country not found" }),
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }),
        ),
    }
}
