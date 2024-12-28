use std::{ops::Deref, str::FromStr};
use jupiter_swap_api_client::{
    quote::QuoteRequest,
    swap::SwapRequest,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner, transaction::VersionedTransaction};
use axum::{extract ,routing::{get, post}, Json, Router};
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
struct SwapParams {
    input_mint: String,
    output_mint: String,
    public_key: String,
    amount: u64
}


async fn health()->Json<Value>{
    Json(json!({"status":"healthy"}))
}

async fn swap_handler(extract::Json(payload): extract::Json<SwapParams>)-> Json<Value>{
    let txn = swap(payload.input_mint,payload.output_mint,payload.public_key,payload.amount).await;
    Json(json!({"transaction":&txn}))

}

async fn swap(input_mint:String, output_mint:String, public_key:String, amount:u64) -> VersionedTransaction{
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

    let versioned_transaction: VersionedTransaction = bincode::deserialize(&swap_response.swap_transaction).unwrap();
    let signer = NullSigner::new(&Pubkey::from_str(public_key.as_str()).unwrap());
    let tx = VersionedTransaction::try_new(versioned_transaction.message, &[&signer]).unwrap();

    tx
}

pub fn swap_router() ->Router{
    Router::new()
        .route("/swap", post(swap_handler))
        .route("/health",get(health))
}