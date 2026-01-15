use axum::{Router, Json, routing::post, extract::State};
use sqlx::PgPool;

use crate::{
    auth::{jwt, password},
    models::user::{RegisterRequest, LoginRequest, AuthResponse},
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(
    State(db): State<PgPool>,
    Json(input): Json<RegisterRequest>,
) -> Json<&'static str> {
    let hash = password::hash_password(&input.password);

    sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        input.email,
        hash
    )
    .execute(&db)
    .await
    .unwrap();

    Json("User registered")
}

async fn login(
    State(db): State<PgPool>,
    Json(input): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let user = sqlx::query!(
        "SELECT id, password_hash, role FROM users WHERE email = $1",
        input.email
    )
    .fetch_one(&db)
    .await
    .unwrap();

    password::verify_password(&user.password_hash, &input.password);

    Json(AuthResponse {
        access_token: jwt::create_token(user.id, &user.role),
    })
}
