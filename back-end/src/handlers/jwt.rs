// src/auth.rs
use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    body::Body,
};
use tower_cookies::{CookieManagerLayer, Cookies};
use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use crate::models::user::User;

const SECRET_KEY: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str) -> String {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims { sub: user_id.to_string(), exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    ).unwrap()
}

pub fn verify_token(token: &str) -> jsonwebtoken::errors::Result<jsonwebtoken::TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::default(),
    )
}

/// Bu katmanı main.rs'de şöyle ekle:
///    .layer(CookieManagerLayer::new())
///    .layer(Extension(pool.clone()))
///    .route_layer(from_fn_with_state(pool, auth_middleware))
pub async fn auth_middleware(
    Extension(pool): Extension<MySqlPool>,
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {

    // 1. tower_cookies katmanının eklediği Cookies objesi
    let cookies = match req.extensions().get::<Cookies>() {
        Some(c) => c.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Cookie manager yok").into_response(),
    };

    // 2. axtoken çerezini al
    let cookie = match cookies.get("axtoken") {
        Some(c) => c,
        None => return (StatusCode::UNAUTHORIZED, "Token bulunamadı").into_response(),
    };
    let token = cookie.value().to_string();

    // 3. JWT doğrula
    let data = match verify_token(&token) {
        Ok(d) => d,
        Err(e) => {
            let msg = match *e.kind() {
                ErrorKind::ExpiredSignature => "Token süresi dolmuş",
                _ => "Token geçersiz",
            };
            return (StatusCode::UNAUTHORIZED, msg).into_response()
        }
    };

    // 4. user_id'yi parse et
    let user_id = match Uuid::parse_str(&data.claims.sub) {
        Ok(u) => u.to_string(),
        Err(_) => return (StatusCode::UNAUTHORIZED, "ID parse hatası").into_response(),
    };

    // 5. DB'den kullanıcıyı çek
    let row = match sqlx::query!(
        "SELECT id, username, email, avatar, role FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(r) => r,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Kullanıcı bulunamadı").into_response(),
    };

    // 6. User objesini request'e ekle
    req.extensions_mut().insert(User {
        id:       row.id,
        username: row.username,
        email:    row.email,
        avatar:   row.avatar,
        role:     row.role,
    });

    // 7. Handler zincirine devam et
    next.run(req).await
}
