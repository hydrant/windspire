use crate::{
    application::http_response::{
        internal_server_error_json_response, json_response, row_not_found_error_json_response,
    },
    domain::repositories::user_repository::UserRepository,
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_user_command(
    State(pg_pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.delete_user(&pg_pool, user_id).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("User not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
