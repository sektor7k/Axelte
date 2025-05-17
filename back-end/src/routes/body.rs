use axum::routing::get;
use axum::{middleware::from_fn_with_state, routing::post, Router};
use sqlx::MySqlPool;
use crate::middleware::auth_middleware::auth_middleware;
use crate::handlers::body_handlers::{create_page, create_workspace, get_workspace_id, get_workspace_pages, get_workspaces};

pub fn body_routes(pool:MySqlPool) -> Router{
    Router::new()
    .route("/createworkspace", post(create_workspace))
    .route("/get-workspaces", get(get_workspaces))
    .route("/workspaces/{workspaceId}", get(get_workspace_id))
    .route("/workspaces/{workspaceId}/pages", get(get_workspace_pages))
    .route("/create-page", post(create_page))
    .layer(from_fn_with_state(pool.clone(), auth_middleware))
}