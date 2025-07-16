use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};

use crate::application::{
    commands::{
        delete_country_command::delete_country_command, delete_user_command::delete_user_command,
        insert_boat_command::insert_boat_command, insert_country_command::insert_country_command,
        insert_user_command::insert_user_command, update_country_command::update_country_command,
        update_user_command::update_user_command,
    },
    handlers::auth_handlers::{
        login_handler, logout_handler, me_handler, oauth_callback_handler, refresh_token_handler,
    },
    middleware::{auth_middleware::jwt_auth_middleware, rbac_middleware::require_permission},
    queries::{
        get_countries_query::get_countries_query,
        get_country_by_code_query::get_country_by_code_query,
        get_country_by_id_query::get_country_by_id_query,
        get_user_by_id_query::get_user_by_id_query, get_users_query::get_users_query,
    },
};

use crate::application::state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    // CORS configuration for frontend integration
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(false);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(|| async { "Backend is running!" }))
        .route("/auth/login", get(login_handler))
        .route("/auth/callback", get(oauth_callback_handler))
        .route("/auth/refresh", post(refresh_token_handler));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/auth/logout", post(logout_handler))
        .route("/auth/me", get(me_handler))
        .route("/users", get(get_users_query))
        .route("/users/{user_id}", get(get_user_by_id_query))
        .route("/countries", get(get_countries_query))
        .route("/countries/{country_id}", get(get_country_by_id_query))
        .route(
            "/countries/code/{country_code}",
            get(get_country_by_code_query),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth_middleware,
        ));

    // Admin routes (authentication + admin permissions required)
    let admin_routes = Router::new()
        .route("/users", post(insert_user_command))
        .route("/users/{user_id}", put(update_user_command))
        .route("/users/{user_id}", delete(delete_user_command))
        .route("/countries", post(insert_country_command))
        .route("/countries/{country_id}", put(update_country_command))
        .route("/countries/{country_id}", delete(delete_country_command))
        .route("/boats", post(insert_boat_command))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_permission(
                crate::application::middleware::rbac_middleware::RequiredPermission::new(
                    "admin:write",
                ),
            ),
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth_middleware,
        ));

    // Combine all routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(cors)
        .with_state(app_state)
}
