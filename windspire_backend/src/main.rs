use application::{
    commands::insert_user_command::insert_user_command, queries::{get_countries_query::get_countries_query, get_users_query::get_users_query},
};
use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> () {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().expect("Unable to access .env file");

    // Read server address from .env/environment. Fallback to 127.0.0.1:3000
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url =
        env::var("DATABASE_URL").expect("Unable to read DATABASE_URL environment variable");
    println!("DATABASE_URL: {}", database_url);

    // Create database connections pool
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Run existing database migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations...");

    // Create Axum TCP listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create a TCP listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/users", get(get_users_query))
        .route("/users", post(insert_user_command))
        .route("/countries", get(get_countries_query))
        .with_state(db_pool);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
