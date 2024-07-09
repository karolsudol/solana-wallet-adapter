

use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    wallet_address: Option<String>,
}


#[derive(Deserialize)]
struct WalletAddress {
    address: String,
}

#[derive(Deserialize)]
struct TransactionForm {
    from_address: String,
    to_address: String,
    amount: String,
}

#[derive(Serialize)]
struct WalletConnectionResponse {
    wallet_address: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/connect-wallet", post(connect_wallet))
        .route("/send-sol", post(send_sol))
        .layer(ServiceBuilder::new().into_inner());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let template = IndexTemplate { wallet_address: None };
    Html(template.render().unwrap())
}

async fn connect_wallet(Form(wallet_address): Form<WalletAddress>) -> impl IntoResponse {
    let template = IndexTemplate {
        wallet_address: Some(wallet_address.address),
    };
    Html(template.render().unwrap())
}

async fn send_sol(Form(transaction): Form<TransactionForm>) -> Html<String> {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let from_pubkey = Pubkey::from_str(&transaction.from_address).unwrap();
    let to_pubkey = Pubkey::from_str(&transaction.to_address).unwrap();
    let amount = transaction.amount.parse::<u64>().unwrap();

    // Load your keypair from a file or environment variable
    let from_keypair = Keypair::from_base58_string("YOUR_BASE58_KEYPAIR");

    let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, amount);
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap(); // Changed to get_latest_blockhash
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&from_pubkey),
        &[&from_keypair],
        recent_blockhash,
    );

    match rpc_client.send_and_confirm_transaction(&tx) {
        Ok(signature) => Html(format!("Transaction sent! Signature: {}", signature)),
        Err(err) => Html(format!("Failed to send transaction: {}", err)),
    }
}