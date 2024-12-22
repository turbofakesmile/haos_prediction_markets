use std::collections::HashSet;
use std::sync::Arc;

use orderbook::OrderBook;

// Define the types we're going to use
pub type TradeData = (u32, u32, i32, f64); // (contract_id, order_id, volume, price)

pub struct MarketData {
    traders: HashSet<Arc<dyn MarketDataDelegate>>,
}

impl MarketData {
    pub fn new() -> Self {
        MarketData {
            traders: HashSet::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Arc<dyn MarketDataDelegate>) {
        self.traders.insert(subscriber);
    }

    pub fn publish_public_trade(&self, contract_id: u32, order_id: u32, volume: i32, price: f64) {
        let trade_data = (contract_id, order_id, volume, price);
        for trader in &self.traders {
            trader.on_public_trade(trade_data);
        }
    }

    pub fn publish_order_book(&self, contract_id: u32, order_book: &OrderBook) {
        for trader in &self.traders {
            trader.on_order_book(contract_id, order_book);
        }
    }
}

// The trait for market data delegates, which subscribers implement
pub trait MarketDataDelegate {
    fn on_public_trade(&self, trade_data: TradeData);
    fn on_order_book(&self, contract_id: u32, order_book: &OrderBook);
}
