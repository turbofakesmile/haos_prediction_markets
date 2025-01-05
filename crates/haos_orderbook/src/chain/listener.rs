use alloy::{
    primitives::Address,
    providers::Provider,
    pubsub::PubSubFrontend,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use anyhow::Result;
use futures_util::{future::join_all, StreamExt};
use tracing::info;

use super::{contract::IOrderBook, ContractEvent};
use crate::{orderbook::MatchedOrders, OrderHandler};

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
            start_block: 1,
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
        let mut latest_block = self.provider.get_block_number().await?;
        info!("Latest block: {}", latest_block);

        let orders = self
            .fetch_orders_in_range(self.start_block, latest_block)
            .await?;
        self.handle_orders(&orders).await?;

        let sub = self.provider.subscribe_blocks().await?;
        let mut stream = sub.into_stream();

        while let Some(block_header) = stream.next().await {
            // get logs from block
            let block_number = block_header.number;
            if latest_block + 1 > block_number {
                continue;
            }
            let orders = self
                .fetch_orders_in_range(latest_block + 1, block_number)
                .await?;
            latest_block = block_number;
            self.handle_orders(&orders).await?;
        }

        Ok(())
    }

    fn extract_order_from_log(&self, log: Log) -> Result<Option<ContractEvent>> {
        let block_number = log.block_number.unwrap_or(0);

        match log.topic0() {
            Some(&IOrderBook::OrderPlaced::SIGNATURE_HASH) => {
                let IOrderBook::OrderPlaced { id } = log.log_decode()?.inner.data;
                Ok(Some(ContractEvent::OrderUpdated(id, block_number)))
            }
            Some(&IOrderBook::OrderFilled::SIGNATURE_HASH) => {
                let IOrderBook::OrderFilled { id } = log.log_decode()?.inner.data;
                Ok(Some(ContractEvent::OrderUpdated(id, block_number)))
            }
            Some(&IOrderBook::OrdersMatched::SIGNATURE_HASH) => {
                let IOrderBook::OrdersMatched { takerId, makerId } = log.log_decode()?.inner.data;
                Ok(Some(ContractEvent::OrdersMatched(
                    takerId,
                    makerId,
                    block_number,
                )))
            }
            _ => Ok(None),
        }
    }

    async fn fetch_orders_in_range(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<ContractEvent>> {
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

    async fn handle_orders(&mut self, orders: &Vec<ContractEvent>) -> Result<()> {
        info!("Handling orders: {:?}", orders);
        for order in orders.iter() {
            match order {
                ContractEvent::OrdersMatched(taker_id, maker_id, _) => {
                    let matched_orders = MatchedOrders {
                        taker_order_id: taker_id.clone().try_into().unwrap(),
                        maker_order_id: maker_id.clone().try_into().unwrap(),
                    };
                    join_all(
                        self.handlers
                            .iter_mut()
                            .map(|handler| handler.match_orders(matched_orders.clone())),
                    )
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
                }
                _ => {}
            }
        }

        let orders = orders
            .iter()
            .filter_map(|order| match order {
                ContractEvent::OrderUpdated(id, block_number) => Some((*id, *block_number)),
                _ => None,
            })
            .collect::<Vec<_>>();

        for handler in self.handlers.iter_mut() {
            handler.handle_orders(orders.clone()).await?;
        }
        Ok(())
    }
}
