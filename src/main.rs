mod db;
mod errors;
mod handlers;
mod models;
mod query_builder;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::init_pool().await;

    let app = Router::new()
        .route("/employees", get(handlers::list_employees))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
