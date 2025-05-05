use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::application::{
    commands::{
        delete_country_command::delete_country_command, delete_user_command::delete_user_command,
        insert_boat_command::insert_boat_command, insert_country_command::insert_country_command,
        insert_user_command::insert_user_command, update_country_command::update_country_command,
        update_user_command::update_user_command,
    },
    queries::{
        get_countries_query::get_countries_query,
        get_country_by_code_query::get_country_by_code_query,
        get_country_by_id_query::get_country_by_id_query,
        get_user_by_id_query::get_user_by_id_query, get_users_query::get_users_query,
    },
};

use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/users", get(get_users_query))
        .route("/users", post(insert_user_command))
        .route("/users/{user_id}", get(get_user_by_id_query))
        .route("/users/{user_id}", put(update_user_command))
        .route("/users/{user_id}", delete(delete_user_command))
        .route("/countries", get(get_countries_query))
        .route("/countries", post(insert_country_command))
        .route("/countries/{country_id}", get(get_country_by_id_query))
        .route(
            "/countries/code/{country_code}",
            get(get_country_by_code_query),
        )
        .route("/countries/{country_id}", put(update_country_command))
        .route("/countries/{country_id}", delete(delete_country_command))
        .route("/boats", post(insert_boat_command))
        .with_state(pool)
}
