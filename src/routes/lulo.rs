use std::ptr::null;

use axum::{extract, routing::post, Json, Router};
use chttp::{prelude::Request, Body, ResponseExt};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use solana_sdk::pubkey;

use crate::routes::lulo;

#[derive(Deserialize)]
struct DepositParams {
    owner: String,
    mintAddress: String,
    depositAmount: u64,
}

#[derive(Deserialize)]
struct WithdrawParams {
    owner: String,
    mintAddress: String,
    withdrawAmount: u64,
    withdrawAll: bool,
}

#[derive(Deserialize)]
struct UpdateParams {
    owner: String,
    allowedProtocols: String,
}

async fn deposit(
    owner: String,
    mintAddress: String,
    depositAmount: u64,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();

    let priority = "?priorityFee=50000";
    let url = format!("https://api.flexlend.fi/generate/account/deposit{priority}");

    let payload_json = json!({
        "owner": owner,
        "mintAddress": mintAddress,
        "depositAmount": depositAmount
    });
    let payload = to_string(&payload_json).unwrap();
    let request = Request::post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();

    let mut response = chttp::send(request).unwrap();
    let result = response.text().unwrap();

    let parsed_result: Value = serde_json::from_str(&result).unwrap();
    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }
    Json(json!({"transactions":transactions}))
}

async fn deposit_handler(extract::Json(params): extract::Json<DepositParams>) -> Json<Value> {
    deposit(params.owner, params.mintAddress, params.depositAmount).await
}

async fn withdraw(
    owner: String,
    mintAddress: String,
    withdrawAmount: u64,
    withdrawAll: bool,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();
    let priority = "?priorityFee=50000";
    let url = format!("https://api.flexlend.fi/generate/account/withdraw{priority}");
    let payload_json = json!({
        "owner": owner,
        "mintAddress": mintAddress,
        "withdrrawAmount": withdrawAmount,
        "withdrawAll": withdrawAll
    });
    let payload = to_string(&payload_json).unwrap();
    let request = Request::post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();

    let mut response = chttp::send(request).unwrap();
    let result = response.text().unwrap();

    let parsed_result: Value = serde_json::from_str(&result).unwrap();
    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }
    Json(json!({"transactions":transactions}))
}
async fn withdraw_handler(extract::Json(params): extract::Json<WithdrawParams>) -> Json<Value> {
    withdraw(
        params.owner,
        params.mintAddress,
        params.withdrawAmount,
        params.withdrawAll,
    )
    .await
}

async fn update(
    owner: String,
    allowedProtocols: String,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();
    let priority = "?priorityFee=50000";
    let url = format!("https://api.flexlend.fi/generate/account/update{priority}");
    let payload_json = json!({
        "owner": owner,
        "allowedProtocols": allowedProtocols,
        "minimumRate":0
    });
    let payload = to_string(&payload_json).unwrap();
    let request = Request::post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();

    let mut response = chttp::send(request).unwrap();
    let result = response.text().unwrap();

    let parsed_result: Value = serde_json::from_str(&result).unwrap();
    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }
    Json(json!({"transactions":transactions}))
}
async fn update_handler(extract::Json(params): extract::Json<UpdateParams>) -> Json<Value> {
    update(params.owner, params.allowedProtocols).await
}

pub fn lulo_router() -> Router {
    Router::new()
        .route("/deposit", post(deposit_handler))
        .route("/withdraw", post(withdraw_handler))
        .route("/update", post(update_handler))
}