// main.rs
use std::{env, io};

use anyhow::Result;
use haos_orderbook::{config::resolve_config, server::start_order_server};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let server_config = resolve_config();
    start_order_server(&server_config).await?;
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
