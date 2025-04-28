use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::http_response::{
        internal_server_error_json_response, ok_json_response, row_not_found_error_json_response,
    },
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_country_by_id_query(
    State(pg_pool): State<PgPool>,
    Path(country_id): Path<Uuid>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_country_by_id(&pg_pool, country_id).await {
        Ok(country) => ok_json_response(country),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Country not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
