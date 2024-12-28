use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    pubsub::PubSubFrontend,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use anyhow::Result;
use futures_util::StreamExt;
use tracing::info;

use super::contract::IOrderBook;
use crate::OrderHandler;

pub struct OrderListenerBuilder<'a, P: Provider<PubSubFrontend>, H: OrderHandler> {
    provider: &'a P,
    address: Option<Address>,
    handlers: Vec<H>,
    start_block: u64,
}

impl<'a, P: Provider<PubSubFrontend>, H: OrderHandler> OrderListenerBuilder<'a, P, H> {
    pub fn new(provider: &'a P) -> Self {
        Self {
            provider,
            address: None,
            handlers: Vec::new(),
            start_block: 14799,
        }
    }

    pub fn with_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_start_block(mut self, block: u64) -> Self {
        self.start_block = block;
        self
    }

    pub fn with_handler(mut self, handler: H) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn build(self) -> Result<OrderListener<'a, P, H>> {
        let address = self
            .address
            .ok_or_else(|| anyhow::anyhow!("Address is required for OrderListener"))?;

        Ok(OrderListener {
            provider: self.provider,
            address,
            handlers: self.handlers,
            start_block: self.start_block,
        })
    }
}

pub struct OrderListener<'a, P: Provider<PubSubFrontend>, H: OrderHandler> {
    provider: &'a P,
    address: Address,
    handlers: Vec<H>,
    start_block: u64,
}

impl<'a, P: Provider<PubSubFrontend>, H: OrderHandler> OrderListener<'a, P, H> {
    pub fn builder(provider: &'a P) -> OrderListenerBuilder<'a, P, H> {
        OrderListenerBuilder::new(provider)
    }

    pub async fn listen(&mut self) -> Result<()> {
        let latest_block = self.provider.get_block_number().await?;
        info!("Latest block: {}", latest_block);

        let orders = self
            .fetch_orders_in_range(self.start_block, latest_block)
            .await?;
        self.handle_orders(&orders).await?;

        let sub = self
            .provider
            .subscribe_logs(&Filter::new().address(self.address))
            .await?;
        let mut stream = sub.into_stream();

        let mut first_read = true;

        while let Some(log) = stream.next().await {
            if first_read {
                match log.block_number {
                    Some(block_number) => {
                        if block_number > latest_block + 1 {
                            info!(
                                "Handling missed blocks: {} - {}",
                                latest_block + 1,
                                block_number
                            );
                            let missed_orders = self
                                .fetch_orders_in_range(latest_block + 1, block_number)
                                .await?;
                            self.handle_orders(&missed_orders).await?;
                        }
                    }
                    _ => {}
                }
                first_read = false;
            }
            let order = self.extract_order_from_log(log)?;
            if let Some((id, block_number)) = order {
                self.handle_orders(&vec![(id, block_number)]).await?;
            }
        }

        Ok(())
    }

    fn extract_order_from_log(&self, log: Log) -> Result<Option<(U256, u64)>> {
        let block_number = log.block_number.unwrap_or(0);

        match log.topic0() {
            Some(&IOrderBook::OrderPlaced::SIGNATURE_HASH) => {
                let IOrderBook::OrderPlaced { id } = log.log_decode()?.inner.data;
                Ok(Some((id, block_number)))
            }
            Some(&IOrderBook::OrderFilled::SIGNATURE_HASH) => {
                let IOrderBook::OrderFilled { id } = log.log_decode()?.inner.data;
                Ok(Some((id, block_number)))
            }
            _ => Ok(None),
        }
    }

    async fn fetch_orders_in_range(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<(U256, u64)>> {
        let filter = Filter::new()
            .address(self.address)
            .from_block(from_block)
            .to_block(to_block);

        let logs = self.provider.get_logs(&filter).await?;

        let mut orders = Vec::new();
        for log in logs {
            if let Some(order) = self.extract_order_from_log(log)? {
                orders.push(order);
            }
        }

        Ok(orders)
    }

    async fn handle_orders(&mut self, orders: &Vec<(U256, u64)>) -> Result<()> {
        for handler in self.handlers.iter_mut() {
            handler.handle_orders(orders.clone()).await?;
        }
        Ok(())
    }
}
