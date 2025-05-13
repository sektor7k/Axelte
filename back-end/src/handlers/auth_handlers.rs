use axum::{
    extract::{Json, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response}, Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use tower_cookies::{cookie::time::{macros::time, Duration}, Cookie, Cookies};
use uuid::Uuid;
use crate::models::user::User;

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

// pub async fn login(
//     State(pool): State<MySqlPool>,
//     Json(payload): Json<LoginPayload>,
//     Extension(cookies): Extension<Cookies>,
// ) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
//     // 1) Kullanıcıyı çek ve şifreyi doğrula
//     let user = sqlx::query!(
//         "SELECT id, email, password FROM users WHERE email = ?",
//         payload.email
//     )
//     .fetch_one(&pool)
//     .await
//     .map_err(|_| {
//         (
//             StatusCode::UNAUTHORIZED,
//             Json(json!({ "message": "Invalid email" })),
//         )
//     })?;

//     let is_valid = bcrypt::verify(payload.password, &user.password).map_err(|_| {
//         (
//             StatusCode::UNAUTHORIZED,
//             Json(json!({ "message": "Invalid password" })),
//         )
//     })?;
//     if !is_valid {
//         return Err((
//             StatusCode::UNAUTHORIZED,
//             Json(json!({ "message": "Invalid password" })),
//         ));
//     }

//     // 2) JWT’yi üret
//     let token = generate_token(&user.id);

//     // 3) tower-cookies ile çerez ekle
//     cookies.add(
//         Cookie::build(("axtoken", token))
//             .path("/")                            // login'daki ile birebir
//             .max_age(Duration::hours(24))        // 86400 saniye
//             .http_only(true)
//             // .secure(true)                     // prod’da HTTPS ise açın
//             // .same_site(SameSite::None)        // gerekiyorsa ekleyin
//             .finish(),
//     );

//     // 4) JSON yanıtını bir Response’e çevirip dön
//     let response = Json(json!({ "message": "Login successful" })).into_response();
//     Ok(response)
// }

#[derive(Serialize)]
pub struct Profile {
    pub id:       String,
    pub username: String,
    pub email:    String,
}

pub async fn me(Extension(user): Extension<User>) -> Json<Profile> {
    Json(Profile {
        id:       user.id.clone(),
        username: user.username.clone(),
        email:    user.email.clone(),
    })
}

pub async fn logout() -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    println!("logouttt");

    let mut headers = HeaderMap::new();

    headers.insert(
        header::SET_COOKIE,
        "axtoken=; HttpOnly; Path=/; Max-Age=0; ".to_string()
            .parse()
            .unwrap(),
    );

    let body = Json(json!({
        "message": "logged out successfully"
    }));

    Ok((headers, body).into_response())
}

// pub async fn logout(
//     cookies: Cookies,
// ) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {

//     println!("logouttt");
//     // 1️⃣ “axtoken” isimli cookie’yi sil
//     cookies.remove(Cookie::build("axtoken").domain("localhost")
//     .path("/")
//     .http_only(true).into());

//     // (Opsiyonel) Yeni bir test cookie’si eklemek istersen:
//     cookies.add(
//         Cookie::build(("deney_cookie", "deney_degeri"))
//             .path("/")
//             .max_age(time::Duration::hours(1))
//             .http_only(true)
//             .finish()
//     );

//     // 2️⃣ JSON body
//     let body = Json(json!({ "message": "Logout successful" }));

//     // 3️⃣ Response’a çevir ve dön
//     Ok(body.into_response())
// }

