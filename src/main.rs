use axum::Router;
use dotenvy::dotenv;

mod config;
mod db;
mod error;
mod auth;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::init().await;

    let app = Router::new()
        .merge(routes::auth::router())
        .merge(routes::tasks::router())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000")
    .await
    .unwrap();


    println!("🚀 API running on http://localhost:9000");

    axum::serve(listener, app).await.unwrap();
}
