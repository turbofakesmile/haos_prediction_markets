use alloy::primitives::U256;

pub mod contract;
pub mod listener;
pub mod order;

#[derive(Debug)]
pub enum ContractEvent {
    OrderUpdated(U256, u64),
    OrdersMatched(U256, U256, u64),
}
