use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspacePayload {
    pub name: String,
    pub description: String,
    pub emails: Vec<String>,
}

pub async fn create_workspace(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateWorkspacePayload>,
) -> impl IntoResponse {
    let ws_id = Uuid::new_v4().to_string();

    let result = sqlx::query!(
        "INSERT INTO workspaces (id,name, description, owner_id) VALUES (?, ?, ?, ?)",
        ws_id,
        payload.name,
        payload.description,
        user.id
    )
    .execute(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    });

    //TODO: send email to all emails

    let body = Json(json!({
        "message": "workspace created successfully",
        "workspaceId": ws_id
    }));

    (StatusCode::OK, body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
}

pub async fn get_workspaces(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // 1) SQLx ile çek
    let result = sqlx::query!(
        "
        SELECT id, name, description, owner_id
        FROM workspaces
        WHERE owner_id = ?
        ",
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    })?;

    let workspaces: Vec<Workspace> = result
        .into_iter()
        .map(|row| Workspace {
            id: row.id,
            name: row.name,
            description: row.description.unwrap_or_default(),
            owner_id: row.owner_id,
        })
        .collect();

    let body = Json(json!({
        "workspaces": workspaces
    }));

    Ok((StatusCode::OK, body))
}

pub async fn get_workspace_id(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
    Path(workspace_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query!(
        "SELECT id, name, description, owner_id FROM workspaces WHERE id = ? AND owner_id = ?",
        workspace_id,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    })?;
    let workspace: Vec<Workspace> = result
        .into_iter()
        .map(|row| Workspace {
            id: row.id,
            name: row.name,
            description: row.description.unwrap_or_default(),
            owner_id: row.owner_id,
        })
        .collect();

    let body = Json(json!({
        "workspace": workspace
    }));

    Ok((StatusCode::OK, body))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub workspace_id: String,
}

pub async fn get_workspace_pages(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
    Path(workspace_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Önce workspace'in kullanıcıya ait olduğunu kontrol et
    let workspace_exists = sqlx::query!(
        "SELECT id FROM workspaces WHERE id = ? AND owner_id = ?",
        workspace_id,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    })?;

    if workspace_exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Workspace not found" })),
        ));
    }

    let pages = sqlx::query_as!(
        Page,
        "SELECT id, title, workspace_id FROM pages WHERE workspace_id = ? AND created_by = ?",
        workspace_id,
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    })?;

    Ok((StatusCode::OK, Json(pages)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePagePayload {
    pub title: String,
    pub workspace_id: String,
}

pub async fn create_page(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreatePagePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let page_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(
        "INSERT INTO pages (id, title, workspace_id, created_by) VALUES (?, ?, ?, ?)",
        page_id,
        payload.title,
        payload.workspace_id,
        user.id
    )
    .execute(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error"})),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Page created successfully",
        "page_id": page_id
         })),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse {
    pub id: String,
    pub title: String,
    pub workspace_id: String,
    pub created_by: String,
    pub content: Option<String>,
}

pub async fn get_page(
    Extension(pool): Extension<MySqlPool>,
    Extension(user): Extension<User>,
    Path(page_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let page = sqlx::query_as!(
        PageResponse,
        "SELECT id, title, workspace_id, created_by,content FROM pages WHERE id = ? AND created_by = ?",
        page_id,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    })?;

    Ok((StatusCode::OK, Json(page)))
}
