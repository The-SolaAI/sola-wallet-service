use std::str::FromStr;
use jupiter_swap_api_client::{
    quote::QuoteRequest,
    swap::SwapRequest,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;
use axum::{extract, routing::post, Json, Router};
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SwapParams {
    input_mint: String,
    output_mint: String,
    public_key: String,
    amount: u64
}

async fn swap_handler(extract::Json(payload): extract::Json<SwapParams>)-> Json<Value>{
    let txn = swap(payload.input_mint,payload.output_mint,payload.public_key,payload.amount).await;
    Json(json!({"transaction":&txn}))
}

async fn swap(
    input_mint:String, 
    output_mint:String, 
    public_key:String, 
    amount:u64) -> Vec<u8>{
    let jupiter_swap_api_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string());
    let quote_request = QuoteRequest {
        amount: amount,
        input_mint: Pubkey::from_str(input_mint.as_str()).unwrap(),
        output_mint: Pubkey::from_str(output_mint.as_str()).unwrap(),
        slippage_bps: 50,
        ..QuoteRequest::default()
    };


    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: Pubkey::from_str(public_key.as_str()).unwrap(),
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();

    swap_response.swap_transaction
}

pub fn swap_router() -> Router {
    Router::new().route("/swap", post(swap_handler))
}


