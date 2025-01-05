use std::{env, str::FromStr};

use alloy::primitives::Address;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub rpc_url_ws: String,
    pub orderbook_address: Address,
    pub private_key: String,
    pub orderbook_start_block: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FheDecryptionConfig {
    pub api_url: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Config {
    pub chain: ChainConfig,
    pub fhe_decryption: FheDecryptionConfig,
}

pub fn resolve_config() -> Config {
    Config {
        chain: ChainConfig {
            rpc_url: "https://api.nitrogen.fhenix.zone".to_string(),
            rpc_url_ws: "wss://api.nitrogen.fhenix.zone:8548".to_string(),
            orderbook_address: Address::from_str(
                env::var("CONTRACT_ADDRESS")
                    .expect("CONTRACT_ADDRESS env var not set")
                    .as_str(),
            )
            .unwrap(),
            private_key: env::var("PRIVATE_KEY").expect("PRIVATE_KEY env var not set"),
            orderbook_start_block: env::var("START_BLOCK")
                .expect("START_BLOCK env var not set")
                .parse()
                .expect("START_BLOCK env var not a number"),
        },
        fhe_decryption: FheDecryptionConfig {
            api_url: env::var("FHE_DECRYPTION_API_URL")
                .expect("FHE_DECRYPTION_API_URL env var not set"),
        },
    }
}
