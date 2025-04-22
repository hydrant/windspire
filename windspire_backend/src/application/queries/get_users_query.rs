use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::common::http_reponse::json_response, domain::interface::user_repository::UserRepository, infrastructure::repositories::sqlx_user_repository::SqlxUserRepository
};


pub async fn get_users_query(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.get_users(&pg_pool).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(e) => json_response(StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string() })),
    }
}
