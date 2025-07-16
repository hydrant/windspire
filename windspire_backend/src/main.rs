use application::approuter;
use application::config::AppConfig;
use application::services::{jwt_service::JwtService, oauth_service::OAuthService};
use application::state::AppState;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> () {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().expect("Unable to access .env file");

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");
    println!("DATABASE_URL: {}", config.database_url);

    // Create database connections pool
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    // Run existing database migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations...");

    // Create JWT service
    let jwt_service = Arc::new(JwtService::new(config.jwt.clone()));

    // Create OAuth service
    let oauth_service =
        Arc::new(OAuthService::new(config.oauth.clone()).expect("Failed to create OAuth service"));

    // Create application state
    let app_state = AppState::new(db_pool, jwt_service, oauth_service, config.clone());

    // Create Axum TCP listener
    let listener = TcpListener::bind(&config.server_address)
        .await
        .expect("Could not create a TCP listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = approuter::create_router(app_state);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
