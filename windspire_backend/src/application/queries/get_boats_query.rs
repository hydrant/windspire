use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    application::{
        http_response::{internal_server_error_json_response, ok_json_response},
        state::AppState,
    },
    domain::interface::boat_repository::{BoatRepository, PaginationParams},
    infrastructure::repositories::sqlx_boat_repository::SqlxBoatRepository,
};

#[derive(Debug, Deserialize)]
pub struct BoatQueryParams {
    page: Option<u32>,
    limit: Option<u32>,
    include: Option<String>,
}

pub async fn get_boats_query(
    State(app_state): State<AppState>,
    Query(params): Query<BoatQueryParams>,
) -> impl IntoResponse {
    let pagination_params = PaginationParams {
        page: params.page.unwrap_or(1).max(1),
        limit: params.limit.unwrap_or(20).clamp(1, 100), // Limit between 1 and 100
    };

    let repository = SqlxBoatRepository;
    
    // Check if owners should be included
    let include_owners = params.include.as_deref() == Some("owners");
    
    if include_owners {
        match repository
            .get_paginated_with_owners(&app_state.db_pool, pagination_params)
            .await
        {
            Ok(result) => ok_json_response(result),
            Err(err) => internal_server_error_json_response(err),
        }
    } else {
        match repository
            .get_paginated(&app_state.db_pool, pagination_params)
            .await
        {
            Ok(result) => ok_json_response(result),
            Err(err) => internal_server_error_json_response(err),
        }
    }
}
