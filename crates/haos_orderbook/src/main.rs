use std::{env, io};

use alloy::providers::{ProviderBuilder, WsConnect};
use anyhow::Result;
use haos_orderbook::{
    chain::{listener::OrderListenerBuilder, order::MockedOrderMetadataReader},
    config::resolve_config,
    LoggingOrderHandler,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let config = resolve_config();
    init_tracing();
    // let orderbook = OrderBook::new();

    let ws_provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(config.chain.rpc_url_ws))
        .await?;

    let http_provider = ProviderBuilder::new().on_http(config.chain.rpc_url.parse()?);

    let mocked_order_metadata_reader =
        MockedOrderMetadataReader::new(&http_provider, config.chain.orderbook_address);

    let mut listener = OrderListenerBuilder::new(&ws_provider)
        .with_address(config.chain.orderbook_address)
        .with_handler(LoggingOrderHandler::new(mocked_order_metadata_reader))
        .build()?;

    listener.listen().await?;

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::new(env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    tracing_subscriber::fmt()
        .with_writer(io::stdout)
        .with_target(true)
        .with_env_filter(filter)
        .init();
}
