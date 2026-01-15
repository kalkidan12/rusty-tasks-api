use axum::Router;
use dotenvy::dotenv;

mod config;
mod db;
mod error;
mod auth;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::init().await;

    let app = Router::new()
        .merge(routes::auth::router())
        .merge(routes::tasks::router())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("🚀 API running on http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}
