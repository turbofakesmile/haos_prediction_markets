use alloy::primitives::U256;
use anyhow::Result;
use chain::order::OrderMetadataReader;
use orderbook::MatchedOrders;
use tracing::info;

pub mod chain;
pub mod config;
pub mod constants;
pub mod manager;
pub mod orderbook;

/// Handler trait for processing orders
pub trait OrderHandler: Send + Sync {
    fn handle_orders(
        &mut self,
        orders: Vec<(U256, u64)>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn match_orders(
        &mut self,
        orders: MatchedOrders,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub struct LoggingOrderHandler<T: OrderMetadataReader> {
    order_metadata_reader: T,
}

impl<T: OrderMetadataReader> LoggingOrderHandler<T> {
    pub fn new(order_metadata_reader: T) -> Self {
        Self {
            order_metadata_reader,
        }
    }
    async fn handle_order(&mut self, id: U256, block_number: u64) -> Result<()> {
        info!("Order {} at block {}", id, block_number);
        let metadata = self.order_metadata_reader.get_metadata(id).await?;
        info!("Order metadata: {:?}", metadata);
        Ok(())
    }
}

impl<T: OrderMetadataReader + Send + Sync> OrderHandler for LoggingOrderHandler<T> {
    async fn handle_orders(&mut self, orders: Vec<(U256, u64)>) -> Result<()> {
        for (id, block_number) in orders {
            self.handle_order(id, block_number).await?;
        }
        Ok(())
    }
    async fn match_orders(&mut self, orders: MatchedOrders) -> Result<()> {
        info!("Matched orders: {:?}", orders);
        Ok(())
    }
}
