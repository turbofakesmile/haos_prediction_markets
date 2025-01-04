use alloy::{
    primitives::{Address, TxHash, U256},
    providers::Provider,
    transports::http::{Client, Http},
};
use anyhow::Result;
use serde::Deserialize;

use super::contract::IOrderBook;
use crate::{
    constants::MATCH_GAS_LIMIT,
    orderbook::{
        order::{Order, OrderSide},
        MatchedOrders,
    },
};

pub trait OrderMetadataReader {
    fn get_metadata(
        &self,
        order_id: U256,
    ) -> impl std::future::Future<Output = Result<Order>> + Send;
}

// pub struct MockedOrderMetadataReader<'a, P: Provider<Http<Client>>> {
//     provider: &'a P,
//     contract_address: Address,
// }

// impl<'a, P: Provider<Http<Client>>> MockedOrderMetadataReader<'a, P> {
//     pub fn new(provider: &'a P, contract_address: Address) -> Self {
//         Self {
//             provider,
//             contract_address,
//         }
//     }
// }

// impl<'a, P: Provider<Http<Client>>> OrderMetadataReader for MockedOrderMetadataReader<'a, P> {
//     async fn get_metadata(&self, order_id: U256) -> Result<Order> {
//         // pro
//         let contract = IOrderBook::new(self.contract_address, self.provider);
//         let order = contract.getOrder(order_id).call().await?;
//         Ok(Order::new(
//             order_id.try_into().unwrap(),
//             0,
//             order._1,
//             order._2,
//             if order._0 {
//                 OrderSide::Sell
//             } else {
//                 OrderSide::Buy
//             },
//         ))
//     }
// }

#[derive(Deserialize)]
struct OrderResponse {
    side: bool,
    amount: u64,
    price: u64,
}

pub struct FHEOrderMetadataReader {
    api_url: String,
}

impl FHEOrderMetadataReader {
    pub fn new() -> Self {
        Self {
            api_url: "http://localhost:3000".to_string(),
        }
    }
}

impl OrderMetadataReader for FHEOrderMetadataReader {
    async fn get_metadata(&self, order_id: U256) -> Result<Order> {
        let url = format!("{}/order/{}", self.api_url, order_id);

        let response = reqwest::get(&url)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch order: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch order. Status: {}",
                response.status()
            ));
        }

        let order_data: OrderResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse order response: {}", e))?;

        Ok(Order::new(
            order_id.try_into().unwrap(),
            0,
            order_data.amount.try_into().unwrap(),
            order_data.price.try_into().unwrap(),
            if order_data.side {
                OrderSide::Sell
            } else {
                OrderSide::Buy
            },
        ))
    }
}

pub async fn match_orders<P: Provider<Http<Client>>>(
    orders: MatchedOrders,
    wallet: &P,
    contract_address: Address,
) -> Result<TxHash> {
    let contract = IOrderBook::new(contract_address, wallet);
    let tx_receipt = contract
        .matchOrders(
            U256::from(orders.taker_order_id),
            U256::from(orders.maker_order_id),
        )
        .gas(MATCH_GAS_LIMIT)
        .send()
        .await?
        .get_receipt()
        .await?;
    if tx_receipt.status() {
        Ok(tx_receipt.transaction_hash)
    } else {
        Err(anyhow::anyhow!("Failed to settle orders"))
    }

    // if tx_receipt.status() {
    //     tx
    //     info!("Orders settled successfully");
    //     self.pending_matched_orders = Some(orders);
    // } else {
    //     warn!("Orders failed to settle");
    // }
}
