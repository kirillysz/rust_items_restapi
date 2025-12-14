mod routers;
mod app;
mod dto;
mod models;
mod services;
mod db;
mod config;
mod state;

use crate::config::Settings;
use crate::state::AppState;
use crate::routers::create_router;
use axum::{Router, routing::get};

async fn root() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let settings = Settings::from_env();
    let state = AppState::new(&settings).await;
    
    let app = Router::new()
        .route("/", get(root))
        .nest("/api", create_router())
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}