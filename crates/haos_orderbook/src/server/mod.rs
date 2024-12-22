use std::sync::{Arc, RwLock};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::add_extension::AddExtensionLayer;
use tracing::info;

use crate::{
    config::ServerConfig,
    orderbook::{
        order::{Order, OrderSide},
        OrderBook,
    },
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub orderbook: OrderBook,
}

pub async fn start_order_server(config: &ServerConfig) -> Result<()> {
    info!("Listening on {}", config.address());

    let app_state = Arc::new(RwLock::new(AppState {
        orderbook: OrderBook::new(),
    }));

    let app = Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route("/new_order", post(new_order_handler))
        .layer(AddExtensionLayer::new(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.address()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewOrderQuery {
    pub side: String,
    pub price: u32,
    pub quantity: u32,
}

async fn new_order_handler(
    state: Extension<Arc<RwLock<AppState>>>,
    Json(query): Json<NewOrderQuery>,
) -> String {
    info!("Received new order: {:?}", query);

    let mut app_state = state.write().unwrap();
    app_state.orderbook.add_order(Order {
        id: 0,
        contract_id: 0,
        volume: query.quantity,
        price: query.price,
        side: match query.side.as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => panic!("Invalid side"),
        },
    });

    info!("OrderBook: {:?}", app_state.orderbook);

    "Order received".to_string()
}
