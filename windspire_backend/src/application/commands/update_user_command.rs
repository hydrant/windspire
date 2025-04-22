use axum::{extract::{Path, State}, http::{header, StatusCode}, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{domain::{interface::user_repository::UserRepository, models::user::UserUpdate}, infrastructure::repositories::sqlx_user_repository::SqlxUserRepository};


pub async fn update_user_command(
    State(pg_pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(user_update): Json<UserUpdate>,
) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.update_user(&pg_pool, user_id, user_update).await {
        Ok(users) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : true, "data" : users }).to_string(),
        ),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success": false, "message": "User not found" }).to_string(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : false, "message" : e.to_string() }).to_string(),
        ),
    }
}
