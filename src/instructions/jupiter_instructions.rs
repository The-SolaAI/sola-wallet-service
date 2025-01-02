use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig, JupiterSwapApiClient
};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use super::config::JUP_SWAP_API;

pub async fn swap(
    input_mint: String,
    output_mint: String,
    public_key: String,
    amount: u64,
) -> Vec<u8> {
    let jupiter_swap_api_client =
        JupiterSwapApiClient::new(JUP_SWAP_API.to_string());
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
