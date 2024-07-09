
use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tower::ServiceBuilder;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    wallet_address: Option<String>,
}

#[derive(Deserialize)]
struct WalletAddress {
    address: String,
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

async fn connect_wallet(Form(wallet_address): Form<WalletAddress>) -> Redirect {
    // can store the wallet address in a session or database here
    Redirect::to("/")
}

async fn send_sol(Form(transaction): Form<Transaction>) -> Html<String> {
    // Handle the Solana transaction logic here
    Html("Transaction sent!".to_string())
}

#[derive(Deserialize)]
struct Transaction {
    from_address: String,
    to_address: String,
    amount: String,
}
