use crate::{
    application::http_response::{internal_server_error_json_response, json_response},
    domain::{interface::user_repository::UserRepository, models::user::UserCreate},
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

pub async fn insert_user_command(
    State(pg_pool): State<PgPool>,
    Json(user_create): Json<UserCreate>,
) -> impl IntoResponse {
    // Validate the user_create data
    match user_create.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };

    let repository = SqlxUserRepository;
    match repository.insert_user(&pg_pool, user_create).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(e) => internal_server_error_json_response(e),
    }
}
