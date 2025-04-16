use axum::{
    Router,
    routing::{get, post},
};

use crate::application::{
    commands::insert_user_command::insert_user_command,
    queries::{get_countries_query::get_countries_query, get_users_query::get_users_query},
};

use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/users", get(get_users_query))
        .route("/users", post(insert_user_command))
        .route("/countries", get(get_countries_query))
        .with_state(pool)
}