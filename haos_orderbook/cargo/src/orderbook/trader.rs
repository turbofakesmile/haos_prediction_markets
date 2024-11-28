use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

use order_g::OrderSide;
use matching_engine::MatchingEngine;
use orderbook::OrderBook;

// Assuming these types are defined elsewhere or in separate modules
use order::Order;
use trade_data::TradeData;

pub struct Trader {
    id: u32,
    market: Arc<MatchingEngine>,
    on_order_execution_callback: Option<Rc<dyn Fn(TradeData)>>,
    on_public_trade_callback: Option<Rc<dyn Fn(TradeData)>>,
    on_order_book_callback: Option<Rc<dyn Fn(OrderBook)>>,
}

impl Trader {
    // ID generator, static to maintain uniqueness across instances
    static ID_GENERATOR: RefCell<u32> = RefCell::new(0);

    pub fn new(market: Arc<MatchingEngine>) -> Self {
        Trader {
            id: {
                let mut id_gen = ID_GENERATOR.borrow_mut();
                let id = *id_gen;
                *id_gen += 1;
                id
            },
            market,
            on_order_execution_callback: None,
            on_public_trade_callback: None,
            on_order_book_callback: None,
        }
    }

    pub fn on_order_book(&self, order_book: &OrderBook) {
        if let Some(callback) = &self.on_order_book_callback {
            callback(order_book.clone());
        }
    }

    pub fn on_public_trade(&self, trade_data: &TradeData) {
        if let Some(callback) = &self.on_public_trade_callback {
            callback(trade_data.clone());
        }
    }

    pub fn send_order(&self, contract_id: u32, volume: i32, price: f64, side: OrderSide) -> bool {
        self.market.add_order(Arc::new(Trader::new(Arc::clone(&self.market))), contract_id, volume, price, side)
    }

    pub fn on_order_execution(&self, trade_data: &TradeData) {
        if let Some(callback) = &self.on_order_execution_callback {
            callback(trade_data.clone());
        }
    }

    pub fn set_on_order_execution_callback<F>(&mut self, func: F)
    where
        F: Fn(TradeData) + 'static,
    {
        self.on_order_execution_callback = Some(Rc::new(func));
    }

    pub fn set_on_public_trade_callback<F>(&mut self, func: F)
    where
        F: Fn(TradeData) + 'static,
    {
        self.on_public_trade_callback = Some(Rc::new(func));
    }

    pub fn set_on_order_book_callback<F>(&mut self, func: F)
    where
        F: Fn(OrderBook) + 'static,
    {
        self.on_order_book_callback = Some(Rc::new(func));
    }
}
