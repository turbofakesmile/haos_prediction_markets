use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    transports::http::{Client, Http},
};
use anyhow::Result;

use super::contract::IOrderBook;
use crate::orderbook::order::{Order, OrderSide};

pub trait OrderMetadataReader {
    fn get_metadata(
        &self,
        order_id: U256,
    ) -> impl std::future::Future<Output = Result<Order>> + Send;
}

pub struct MockedOrderMetadataReader<'a, P: Provider<Http<Client>>> {
    provider: &'a P,
    contract_address: Address,
}

impl<'a, P: Provider<Http<Client>>> MockedOrderMetadataReader<'a, P> {
    pub fn new(provider: &'a P, contract_address: Address) -> Self {
        Self {
            provider,
            contract_address,
        }
    }
}

impl<'a, P: Provider<Http<Client>>> OrderMetadataReader for MockedOrderMetadataReader<'a, P> {
    async fn get_metadata(&self, order_id: U256) -> Result<Order> {
        // pro
        let contract = IOrderBook::new(self.contract_address, self.provider);
        let order = contract.getOrder(order_id).call().await?;
        Ok(Order::new(
            order_id.try_into().unwrap(),
            0,
            order._1,
            order._2,
            if order._0 {
                OrderSide::Sell
            } else {
                OrderSide::Buy
            },
        ))
    }
}

pub struct FHEOrderMetadataReader {
    contract_address: Address,
}

impl FHEOrderMetadataReader {
    pub fn new(contract_address: Address) -> Self {
        Self { contract_address }
    }
}

impl OrderMetadataReader for FHEOrderMetadataReader {
    async fn get_metadata(&self, order_id: U256) -> Result<Order> {
        // TODO: call node.js server to get order metadata
        // return not implemented error
        Err(anyhow::anyhow!("Not implemented"))
    }
}
