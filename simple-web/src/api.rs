use crate::error::ApiError;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub addr1: String,
    pub addr2: String,
    pub addr3: String,
    pub status: i32, // 0: not dispatch, 1: working, 2: completed, 3: canceled
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTaskResponse {
    pub task_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTaskPayload {
    pub addr1: String,
    pub addr2: String,
    pub addr3: String,
}

pub async fn submit(
    State(state): State<Arc<AppState>>,
    Json(task): Json<SubmitTaskPayload>,
) -> Result<Json<SubmitTaskResponse>, ApiError> {
    if task.addr1.is_empty() || task.addr2.is_empty() || task.addr3.is_empty() {
        return Err(ApiError::BadRequest);
    };

    let result =
        sqlx::query("INSERT INTO tasks (prover, player, miner) VALUES ($1, $2, $3) returning id")
            .bind(task.addr1)
            .bind(task.addr2)
            .bind(task.addr3)
            .fetch_one(&state.db)
            .await;

    if let Err(e) = result {
        return Err(ApiError::DbError(format!("{:?}", e)));
    }

    let task_id = result.unwrap().get("id");

    Ok(Json(SubmitTaskResponse { task_id }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTaskParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    params: Query<ListTaskParams>,
) -> Result<Json<Vec<Task>>, ApiError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = sqlx::query_as!(
        Task,
        "SELECT id,addr1,addr2,addr3,status FROM tasks ORDER BY id DESC LIMIT $1 OFFSET $2",
        page_size,
        (page - 1) * page_size
    )
    .fetch_all(&state.db)
    .await;

    if let Err(e) = result {
        return Err(ApiError::DbError(format!("{:?}", e)));
    }

    Ok(Json(result.unwrap()))
}
