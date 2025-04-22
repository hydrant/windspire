use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{application::common::http_reponse::json_response, domain::{interface::user_repository::UserRepository, models::user::UserUpdate}, infrastructure::repositories::sqlx_user_repository::SqlxUserRepository};


pub async fn update_user_command(
    State(pg_pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(user_update): Json<UserUpdate>,
) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.update_user(&pg_pool, user_id, user_update).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(sqlx::Error::RowNotFound) => json_response(StatusCode::NOT_FOUND, json!({ "success": false, "message": "User not found" })),
        Err(e) => json_response(StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string() })),
    }
}
