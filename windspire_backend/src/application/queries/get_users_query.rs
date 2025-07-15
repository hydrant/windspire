use axum::{extract::State, response::IntoResponse};

use crate::{
    application::{
        http_response::{internal_server_error_json_response, ok_json_response},
        state::AppState,
    },
    domain::repositories::user_repository::UserRepository,
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};

pub async fn get_users_query(State(app_state): State<AppState>) -> impl IntoResponse {
    let repository = SqlxUserRepository;
    match repository.get_users(&app_state.db_pool).await {
        Ok(users) => ok_json_response(users),
        Err(err) => internal_server_error_json_response(err),
    }
}
