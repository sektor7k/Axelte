use axum::{
    extract::{Json, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::MySqlPool;
use uuid::Uuid;

use super::jwt::generate_token;

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
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Database error" })),
        )
    });

    if email_existing.unwrap().count > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "This email has already been used" })),
        ));
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

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {

    let user = sqlx::query!(
        "SELECT id, email, password FROM users WHERE email = ?",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid email" })),
        )
    })?;

    let is_valid_password = bcrypt::verify(payload.password, &user.password).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid password" })),
        )
    })?;

    if !is_valid_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid password" })),
        ));
    }

    // jwt token adjustment

    let token = generate_token(&user.id);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        format!("axtoken={}; HttpOnly; Path=/; Max-Age=86400",token)
            .parse()
            .unwrap()
    );

    let body = Json(json!({ "message": "Login successful" }));

    Ok((headers, body).into_response())
}
