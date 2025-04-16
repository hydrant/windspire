use application::approuter;
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

    let app = approuter::create_router(db_pool);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
