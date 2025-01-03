use super::{jupiter, lulo};

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health() -> Json<Value> {
    Json(json!({"status":"healthy"}))
}

pub fn swap_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest(
            "/jup",
            jupiter::jupiter_router(),
        )
        .nest(
            "/lulo",
            lulo::lulo_router()
        )
        // .nest(
        //     "/drift",
        //     drift::drift_router(),
        // )

}
