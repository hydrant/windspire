use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    domain::user_repository::UserRepository,
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};

pub async fn get_users_query(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.get_users(&pg_pool).await {
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

