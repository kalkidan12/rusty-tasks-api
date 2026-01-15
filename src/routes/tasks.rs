use axum::{
    Router, Json, routing::{get, post},
    extract::{State, Query},
};
use sqlx::PgPool;

use crate::{
    auth::middleware::AuthUser,
    models::task::{Task, CreateTaskRequest, TaskResponse},
};

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
) -> Json<Vec<TaskResponse>> {
    let page = q.page.unwrap_or(1);
    let limit = q.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let search = format!("%{}%", q.search.unwrap_or_default());

    let tasks = sqlx::query_as!(
        Task,
        r#"
        SELECT id, user_id, title, completed, created_at
        FROM tasks
        WHERE user_id = $1 AND title ILIKE $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        auth.user_id,
        search,
        limit,
        offset
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(
        tasks.into_iter().map(|t| TaskResponse {
            id: t.id,
            title: t.title,
            completed: t.completed,
            created_at: t.created_at,
        }).collect()
    )
}

async fn create_task(
    auth: AuthUser,
    State(db): State<PgPool>,
    Json(input): Json<CreateTaskRequest>,
) -> Json<&'static str> {
    sqlx::query!(
        "INSERT INTO tasks (user_id, title) VALUES ($1, $2)",
        auth.user_id,
        input.title
    )
    .execute(&db)
    .await
    .unwrap();

    Json("Task created")
}
