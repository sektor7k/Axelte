use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
