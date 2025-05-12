use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn signup(
    State(pool): State<MySqlPool>,
    Json(payload): Json<SignupPayload>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {

    let email_existing = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE email = ?",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Database error" }))));

    
    if email_existing.unwrap().count > 0 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "message": "This email has already been used" }))));
    }

    let hashed_password = bcrypt::hash(payload.password, 10)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Hashing error" })),
            )
        })
        .unwrap();

    let uuid = Uuid::new_v4().to_string();

    let result = sqlx::query!(
        "INSERT INTO users (id, username, email, password) VALUES (?, ?, ?, ?)",
        uuid,
        payload.username,
        payload.email,
        hashed_password
    )
    .execute(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    });

    Ok(StatusCode::CREATED)
}
