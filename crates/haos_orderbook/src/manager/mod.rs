use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    transports::http::{Client, Http},
};
use anyhow::Result;
use tracing::{error, info};

use crate::{
    chain::order::{match_orders, OrderMetadataReader},
    orderbook::{MatchedOrders, OrderBook},
    OrderHandler,
};

#[derive(Debug, Clone)]
pub struct OrderManager<T: OrderMetadataReader, P: Provider<Http<Client>>> {
    order_metadata_reader: T,
    wallet: P,
    contract_address: Address,
    orderbook: OrderBook,
    waiting_orders: Vec<(U256, u64)>,
    pending_matched_orders: Option<MatchedOrders>,
}

impl<T: OrderMetadataReader, P: Provider<Http<Client>>> OrderManager<T, P> {
    pub fn new(order_metadata_reader: T, wallet: P, contract_address: Address) -> Self {
        Self {
            order_metadata_reader,
            orderbook: OrderBook::new(),
            waiting_orders: Vec::new(),
            pending_matched_orders: None,
            wallet,
            contract_address,
        }
    }
    async fn add_orders(&mut self, orders: &[(U256, u64)]) -> Result<()> {
        // find unique order ids, as they may be duplicated
        let mut orders = orders.to_vec();
        orders.sort_by(|a, b| a.0.cmp(&b.0));
        orders.dedup_by(|a, b| a.0 == b.0);

        for (id, _) in orders.iter() {
            let order = self.order_metadata_reader.get_metadata(*id).await?;
            self.orderbook.update_order(order);
        }
        info!("Orderbook: {:?}", self.orderbook);
        Ok(())
    }

    async fn settle_orders(&mut self, orders: MatchedOrders) -> Result<()> {
        info!("Settling orders on chain: {:?}", orders);

        let result = match_orders(orders.clone(), &self.wallet, self.contract_address).await;
        if result.is_err() {
            error!("Failed to settle orders: {:?}", result);
        } else {
            self.pending_matched_orders = Some(orders);
        }
        Ok(())
    }
}

impl<T: OrderMetadataReader + Send + Sync, P: Provider<Http<Client>>> OrderHandler
    for OrderManager<T, P>
{
    async fn handle_orders(&mut self, orders: Vec<(U256, u64)>) -> Result<()> {
        self.waiting_orders.extend(orders);
        if self.pending_matched_orders.is_some() {
            return Ok(());
        }
        self.add_orders(&self.waiting_orders.clone()).await?;
        self.waiting_orders.clear();

        if let Some(matched_orders) = self.orderbook.find_matching_orders() {
            info!("Matched orders: {:?}", matched_orders);
            self.settle_orders(matched_orders).await?;
        }
        Ok(())
    }

    async fn match_orders(&mut self, orders: MatchedOrders) -> Result<()> {
        if self.pending_matched_orders.is_some() {
            self.pending_matched_orders = None;
            info!("Confirmed orders matching: {:?}", orders);
        }
        Ok(())
    }
}
