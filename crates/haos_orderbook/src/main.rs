use std::{env, io, str::FromStr};

use alloy::{
    network::EthereumWallet,
    providers::{ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
};
use anyhow::Result;
use haos_orderbook::{
    chain::{listener::OrderListener, order::FHEOrderMetadataReader},
    config::resolve_config,
    manager::OrderManager,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let config = resolve_config();
    init_tracing();

    let ws_provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(config.chain.rpc_url_ws))
        .await?;

    let mocked_order_metadata_reader: FHEOrderMetadataReader =
        FHEOrderMetadataReader::new(config.fhe_decryption.api_url.clone());

    let wallet = EthereumWallet::from(PrivateKeySigner::from_str(
        &config.chain.private_key.as_str(),
    )?);

    let wallet_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(config.chain.rpc_url.parse()?);

    let mut listener = OrderListener::builder(&ws_provider)
        .with_start_block(config.chain.orderbook_start_block)
        .with_address(config.chain.orderbook_address)
        .with_handler(OrderManager::new(
            mocked_order_metadata_reader,
            wallet_provider,
            config.chain.orderbook_address,
        ))
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
