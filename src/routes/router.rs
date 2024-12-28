use super::swap;

use axum::{routing::{get, post}, Json, Router};
use serde_json::{Value,json};

async fn health()->Json<Value>{
    Json(json!({"status":"healthy"}))
}

pub fn swap_router() ->Router{
    Router::new()
        .route("/swap", post(swap::swap_handler))
        .route("/swap/health",get(health))
}