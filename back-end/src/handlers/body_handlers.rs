use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Serialize, Deserialize};
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
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Database error" })))
    });

    //TODO: send email to all emails
    
    let body = Json(json!({
        "message": "workspace created successfully",
        "workspaceId": ws_id
    }));
   
    (StatusCode::OK, body)
}
