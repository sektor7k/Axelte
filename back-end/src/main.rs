mod routes;
mod db;
mod handlers;
mod models;
use std::env;

use axum::{
    http::{header, HeaderValue, Method}, routing::get, Router
};
use tokio::net::TcpListener;
use routes::auth::auth_routes;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok();
    let client_url = env::var("CLIENT_URL").expect("CLIENT_URL must be set");
    
    let pool = db::init_db().await.unwrap();

    let cors = CorsLayer::new()
    .allow_origin(client_url.parse::<HeaderValue>().unwrap()) 
    .allow_methods([Method::POST, Method::GET])
    .allow_headers([header::CONTENT_TYPE])
    .allow_credentials(true); 
    
    let app = Router::new()
    .nest("/auth", auth_routes(pool.clone()))
    .layer(cors);






    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


