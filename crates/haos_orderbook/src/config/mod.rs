use std::str::FromStr;

use alloy::primitives::Address;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub rpc_url_ws: String,
    pub orderbook_address: Address,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Config {
    pub chain: ChainConfig,
}

pub fn resolve_config() -> Config {
    Config {
        chain: ChainConfig {
            rpc_url: "https://api.nitrogen.fhenix.zone".to_string(),
            rpc_url_ws: "wss://api.nitrogen.fhenix.zone:8548".to_string(),
            orderbook_address: Address::from_str("0xE7DcCB9Ea2ff5FD40e9EB493Fc9dE29770d1B49A")
                .unwrap(),
        },
    }
}
