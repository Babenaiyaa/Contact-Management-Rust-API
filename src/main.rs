use axum::Router;
use dotenvy::dotenv;
use sea_orm::Database;
use std::env;

mod app;
mod entities;

use app::route::routes;
use app::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Connected to PostgreSQL");

    let state = AppState { db };

    let app = Router::new()
        .merge(routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}