#![allow(dead_code)]

use application::approuter;
use application::config::AppConfig;
use application::services::{firebase_service::FirebaseService, jwt_service::JwtService};
use application::state::AppState;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
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

    // Create database connections pool (optimized for serverless)
    let db_pool = PgPoolOptions::new()
        .max_connections(5) // Reduced for serverless environment
        .min_connections(1) // Ensure at least one connection
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Some(Duration::from_secs(10)))
        .max_lifetime(Some(Duration::from_secs(30)))
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

    // Create Firebase service
    let firebase_service = Arc::new(FirebaseService::new(config.firebase.project_id.clone()));

    // Create application state
    let app_state = AppState::new(db_pool, jwt_service, firebase_service, config.clone());

    // Determine port for Azure Functions or local development
    let port = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").unwrap_or_else(|_| {
        config
            .server_address
            .split(':')
            .next_back()
            .unwrap_or("8080")
            .to_string()
    });
    let bind_address = format!("127.0.0.1:{}", port);

    // Create Axum TCP listener
    let listener = TcpListener::bind(&bind_address)
        .await
        .expect("Could not create a TCP listener");

    println!(
        "Listening on {} (Azure Functions mode: {})",
        listener.local_addr().unwrap(),
        std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").is_ok()
    );

    let app = approuter::create_router(app_state);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
