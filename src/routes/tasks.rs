use axum::{
    Router,
    Json,
    routing::get,
    extract::{State, Query},
};
use sqlx::{PgPool, FromRow};

use crate::{
    auth::middleware::AuthUser,
    models::task::{CreateTaskRequest, TaskResponse},
};

use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(FromRow)]
struct Task {
    id: Uuid,
    user_id: Uuid,
    title: String,
    completed: bool,
    created_at: NaiveDateTime,
}

#[derive(serde::Deserialize)]
pub struct TaskQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
}

async fn list_tasks(
    auth: AuthUser,
    State(db): State<PgPool>,
    Query(q): Query<TaskQuery>,
) -> Result<Json<Vec<TaskResponse>>, axum::http::StatusCode> {
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;
    let search = format!("%{}%", q.search.unwrap_or_default());

    let tasks: Vec<Task> = sqlx::query_as(
        r#"
        SELECT id, user_id, title, completed, created_at
        FROM tasks
        WHERE user_id = $1
          AND title ILIKE $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#
    )
    .bind(auth.user_id)
    .bind(search)
    .bind(limit)
    .bind(offset)
    .fetch_all(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = tasks.into_iter().map(|t| TaskResponse {
        id: t.id,
        title: t.title,
        completed: t.completed,
        created_at: t.created_at,
    }).collect();

    Ok(Json(response))
}

async fn create_task(
    auth: AuthUser,
    State(db): State<PgPool>,
    Json(input): Json<CreateTaskRequest>,
) -> Result<Json<&'static str>, axum::http::StatusCode> {
    sqlx::query(
        r#"INSERT INTO tasks (user_id, title) VALUES ($1, $2)"#
    )
    .bind(auth.user_id)
    .bind(input.title)
    .execute(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json("Task created"))
}
