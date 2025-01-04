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
            orderbook_address: Address::from_str("0x19423a7c620FBa5057A10F2EA7359A350A615026")
                .unwrap(),
        },
    }
}
