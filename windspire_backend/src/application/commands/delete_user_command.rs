use axum::{extract::{Path, State}, http:: StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{application::common::http_reponse::json_response, domain::interface::user_repository::UserRepository, infrastructure::repositories::sqlx_user_repository::SqlxUserRepository};


pub async fn delete_user_command(
    State(pg_pool): State<PgPool>,
    Path(user_id): Path<Uuid>
) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.delete_user(&pg_pool, user_id).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(sqlx::Error::RowNotFound) => json_response(StatusCode::NOT_FOUND, json!({ "success": false, "message": "User not found" })),
        Err(e) => json_response(StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string() })),
    }
}
