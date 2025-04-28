use axum::{extract::State, response::IntoResponse};
use sqlx::PgPool;

use crate::{
    application::http_response::{internal_server_error_json_response, ok_json_response},
    domain::interface::user_repository::UserRepository,
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};

pub async fn get_users_query(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.get_users(&pg_pool).await {
        Ok(users) => ok_json_response(users),
        Err(err) => internal_server_error_json_response(err),
    }
}
