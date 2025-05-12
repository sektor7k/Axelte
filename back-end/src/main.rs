mod routes;
mod db;
mod handlers;
mod models;
use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use routes::auth::auth_routes;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok();
    
    let pool = db::init_db().await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin(Any)        // Tüm domainlerden istek alır
        .allow_methods(Any)       // Tüm HTTP metodlarına izin verir (GET, POST, vs.)
        .allow_headers(Any); 
    
    let app = Router::new()
    .nest("/auth", auth_routes(pool.clone()))
    .layer(cors);






    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


