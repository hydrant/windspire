use crate::{
    application::{
        http_response::{
            internal_server_error_json_response, json_response, row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use uuid::Uuid;

pub async fn delete_country_command(
    State(app_state): State<AppState>,
    Path(country_id): Path<Uuid>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository
        .delete_country(&app_state.db_pool, country_id)
        .await
    {
        Ok(users) => json_response(StatusCode::OK, json!({ "success" : true, "data" : users })),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Country not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
