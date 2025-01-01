use super::jupiter;
use super::lulo;
use axum::{routing::get, Json, Router};
use serde_json::{Value,json};

async fn health()->Json<Value>{
    Json(json!({"status":"healthy"}))
}

pub fn swap_router() ->Router{
    Router::new()
        .route("/health", get(health))
        .nest("/jup", jupiter::swap_router())
        .nest("/lulo", lulo::lulo_router())
}