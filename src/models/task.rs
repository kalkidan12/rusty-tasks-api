use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}
