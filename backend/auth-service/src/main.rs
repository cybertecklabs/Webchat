mod handlers;
mod models;
mod auth;
mod db;

use axum::{
    routing::post,
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::get_database().await.expect("DB connection failed");

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB limit
                .layer(cors),
        )
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("🔐 Auth service running on {}", addr);
    
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
