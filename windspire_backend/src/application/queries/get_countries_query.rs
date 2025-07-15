use axum::{extract::State, response::IntoResponse};

use crate::{
    application::http_response::{internal_server_error_json_response, ok_json_response},
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_countries_query(
    State(app_state): State<crate::application::state::AppState>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_countries(app_state.pool()).await {
        Ok(countries) => ok_json_response(countries),
        Err(e) => internal_server_error_json_response(e),
    }
}
