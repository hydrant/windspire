use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use dotenvy::dotenv;
use serde::Serialize;
use serde_json::json;
use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

pub mod domain;


#[tokio::main]
async fn main() -> () {

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
        .route("/users", get(get_users))
        //.route("/tasks", get(get_tasks).post(create_task))
        //.route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .with_state(db_pool);

    axum::serve(listener, app)
        .await
        .expect("Error serving application");

}


async fn get_users(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(
        domain::User, 
        r#"
        SELECT  
            users.id,
            users.first_name,
            users.last_name,
            users.email,
            users.phone,
            countries.iso_name as country
        FROM public.users 
        JOIN countries ON users.country_id = countries.id;
        "#
    )
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success" : false, "message" : e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success" : true, "data" : rows }).to_string(),
    ))
}
