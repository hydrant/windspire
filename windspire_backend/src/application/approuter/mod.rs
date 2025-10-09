use crate::application::handlers::boat_owner_handlers::{
    add_owner_to_boat, get_boats_for_user, get_owners_for_boat, remove_owner_from_boat,
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;

use crate::application::{
    commands::{
        delete_boat_command::delete_boat_command, delete_country_command::delete_country_command,
        delete_user_command::delete_user_command, insert_boat_command::insert_boat_command,
        insert_country_command::insert_country_command, insert_user_command::insert_user_command,
        update_boat_command::update_boat_command, update_country_command::update_country_command,
        update_user_command::update_user_command,
    },
    handlers::auth_handlers::{
        firebase_auth_handler, logout_handler, me_handler, refresh_token_handler,
    },
    middleware::{auth_middleware::jwt_auth_middleware, rbac_middleware::require_permission},
    queries::{
        get_boats_query::get_boats_query, get_countries_query::get_countries_query,
        get_country_by_code_query::get_country_by_code_query,
        get_country_by_id_query::get_country_by_id_query,
        get_user_by_id_query::get_user_by_id_query, get_users_query::get_users_query,
    },
};

use crate::application::state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    // CORS configuration from environment/config
    let cors_origins: Vec<axum::http::HeaderValue> = app_state
        .config
        .cors
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let cors_methods: Vec<axum::http::Method> = app_state
        .config
        .cors
        .allowed_methods
        .iter()
        .filter_map(|method| method.parse().ok())
        .collect();

    let cors_headers: Vec<axum::http::HeaderName> = app_state
        .config
        .cors
        .allowed_headers
        .iter()
        .filter_map(|header| header.parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods(cors_methods)
        .allow_headers(cors_headers)
        .allow_credentials(false);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(|| async { "Backend is running!" }))
        .route("/auth/firebase", post(firebase_auth_handler))
        .route("/auth/refresh", post(refresh_token_handler));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/auth/logout", post(logout_handler))
        .route("/auth/me", get(me_handler))
        .route("/users", get(get_users_query))
        .route("/users/{user_id}", get(get_user_by_id_query))
        .route("/boats", get(get_boats_query))
        .route("/countries", get(get_countries_query))
        .route("/countries/{country_id}", get(get_country_by_id_query))
        .route(
            "/countries/code/{country_code}",
            get(get_country_by_code_query),
        )
        // Boat-owner endpoints
        .route("/boats/{boat_id}/owners/{user_id}", post(add_owner_to_boat))
        .route(
            "/boats/{boat_id}/owners/{user_id}",
            delete(remove_owner_from_boat),
        )
        .route("/users/{user_id}/boats", get(get_boats_for_user))
        .route("/boats/{boat_id}/owners", get(get_owners_for_boat))
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
        .route("/boats/{boat_id}", put(update_boat_command))
        .route("/boats/{boat_id}", delete(delete_boat_command))
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

    // Combine all routes with /api prefix for consistency between cargo run and func start
    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes);

    Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(app_state)
}
