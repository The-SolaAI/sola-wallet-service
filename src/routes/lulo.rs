use crate::instructions::lulo_instructions::{deposit, update, user_assets, withdraw};
use axum::{
    extract, routing::{get, post}, Json, Router
};
use serde_json::Value;

use serde::Deserialize;

#[derive(Deserialize)]
struct UserAssetsParams {
    owner: String,
}


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

async fn user_assets_handler(
    extract::Query(params): extract::Query<UserAssetsParams>
) -> Json<Value> {
    user_assets(params.owner).await
}
async fn deposit_handler(extract::Json(params): extract::Json<DepositParams>) -> Json<Value> {
    deposit(
        params.owner,
        params.mintAddress,
        params.depositAmount,
    )
    .await
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
async fn update_handler(extract::Json(params): extract::Json<UpdateParams>) -> Json<Value> {
    update(
        params.owner,
        params.allowedProtocols,
    )
    .await
}

pub fn lulo_router() -> Router {
    Router::new()
        .route(
            "/assets",
            get(user_assets_handler),
        )
        .route(
            "/deposit",
            post(deposit_handler),
        )
        .route(
            "/withdraw",
            post(withdraw_handler),
        )
        .route(
            "/update",
            post(update_handler),
        )
}
