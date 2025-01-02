use axum::{extract, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::instructions::jupiter_instructions;

#[derive(Deserialize)]
struct SwapParams {
    input_mint: String,
    output_mint: String,
    public_key: String,
    amount: u64,
}

async fn swap_handler(extract::Json(payload): extract::Json<SwapParams>) -> Json<Value> {
    let txn = jupiter_instructions::swap(
        payload.input_mint,
        payload.output_mint,
        payload.public_key,
        payload.amount,
    )
    .await;
    Json(json!({"transaction":&txn}))
}

pub fn jupiter_router() -> Router {
    Router::new().route("/swap", post(swap_handler))
}
