use super::config::{
    LULO_ASSET_API, LULO_DEPOSIT_API, LULO_PRIORITY, LULO_UPDATE_API, LULO_WITHDRAW_API
};
use axum::Json;
use chttp::{prelude::Request, Body, ResponseExt};
use dotenv::dotenv;
use serde_json::{json, to_string, Value};

pub async fn user_assets(owner: String) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();

    let api_request = Request::get(LULO_ASSET_API.to_string())
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(())
        .unwrap();

    let mut response = chttp::send(api_request).unwrap();
    let result = response.text().unwrap();
    let parsed_result: Value = serde_json::from_str(&result).unwrap();

    let token_balance = parsed_result["data"]["tokenBalances"].as_array().unwrap();
    let total_value = parsed_result["data"]["totalValue"].as_f64().unwrap();
    let interest_earned = parsed_result["data"]["interestEarned"].as_f64().unwrap();
    let deposit_value = total_value - interest_earned;

    Json(json!({
        "tokenBalance": token_balance,
        "totalValue": total_value,
        "depositValue": deposit_value,
        "interestEarned": interest_earned,
    }))
}

pub async fn deposit(
    owner: String,
    mintAddress: String,
    depositAmount: u64,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();
    let payload_json = json!({
        "owner": owner,
        "mintAddress": mintAddress,
        "depositAmount": depositAmount
    });
    let payload = to_string(&payload_json).unwrap();

    let api_request = Request::post(LULO_DEPOSIT_API.to_string() + LULO_PRIORITY)
        .header("Accept", "application/json")
        .header(
            "Content-Type",
            "application/json",
        )
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();
    
    println!(
        "{}",
        LULO_DEPOSIT_API.to_string() + LULO_PRIORITY
    );
    let mut response = chttp::send(api_request).unwrap();
    let result = response.text().unwrap();
    let parsed_result: Value = serde_json::from_str(&result).unwrap();

    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }

    Json(json!({"transactions":transactions}))
}

pub async fn withdraw(
    owner: String,
    mintAddress: String,
    withdrawAmount: u64,
    withdrawAll: bool,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();
    let payload_json = json!({
        "owner": owner,
        "mintAddress": mintAddress,
        "withdrrawAmount": withdrawAmount,
        "withdrawAll": withdrawAll
    });
    let payload = to_string(&payload_json).unwrap();

    let api_request = Request::post(LULO_WITHDRAW_API.to_string() + LULO_PRIORITY)
        .header("Accept", "application/json")
        .header(
            "Content-Type",
            "application/json",
        )
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();

    let mut response = chttp::send(api_request).unwrap();
    let result = response.text().unwrap();
    let parsed_result: Value = serde_json::from_str(&result).unwrap();

    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }

    Json(json!({"transactions":transactions}))
}

pub async fn update(
    owner: String,
    allowedProtocols: String,
) -> Json<Value> {
    dotenv().ok();
    let lulo_api_key = std::env::var("LULO_API_KEY").unwrap();
    let payload_json = json!({
        "owner": owner,
        "allowedProtocols": allowedProtocols,
        "minimumRate":0
    });
    let payload = to_string(&payload_json).unwrap();

    let api_request = Request::post(LULO_UPDATE_API.to_string() + LULO_PRIORITY)
        .header("Accept", "application/json")
        .header(
            "Content-Type",
            "application/json",
        )
        .header("x-wallet-pubkey", owner)
        .header("x-api-key", lulo_api_key)
        .body(Body::from(payload))
        .unwrap();

    let mut response = chttp::send(api_request).unwrap();
    let result = response.text().unwrap();
    let parsed_result: Value = serde_json::from_str(&result).unwrap();

    let transaction_meta = parsed_result["data"]["transactionMeta"].as_array();
    let mut transactions = Vec::new();
    for transaction in transaction_meta {
        transactions.push(transaction)
    }

    Json(json!({"transactions":transactions}))
}
