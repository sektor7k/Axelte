use axum::{middleware::from_fn_with_state, routing::post, Router};
use sqlx::MySqlPool;
use crate::middleware::auth_middleware::auth_middleware;
use crate::handlers::body_handlers::create_workspace;

pub fn body_routes(pool:MySqlPool) -> Router{
    Router::new()
    .route("/createworkspace", post(create_workspace))
    .layer(from_fn_with_state(pool.clone(), auth_middleware))
}