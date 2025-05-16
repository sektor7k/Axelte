use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::MySqlPool;

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

    println!("user");

    println!("payload: {:?}", payload);
    
    let body = Json(json!({
        "message": "logged out successfully",
        "data": user.id
    }));
   
    (StatusCode::OK, body)
}
