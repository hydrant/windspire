use axum::{extract::{Json, State}, http::{header, StatusCode}, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    domain::{models::user::UserCreate, user_repository::UserRepository},
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};


pub async fn insert_user_command(
    State(pg_pool): State<PgPool>,
    Json(user): Json<UserCreate>,
) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.insert_user(&pg_pool, user).await {
        Ok(users) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : true, "data" : users }).to_string(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            json!({ "success" : false, "message" : e.to_string() }).to_string(),
        ),
    }
}
