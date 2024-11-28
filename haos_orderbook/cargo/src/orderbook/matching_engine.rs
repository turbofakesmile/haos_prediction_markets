use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use mdata::MarketData;
use order_g::OrderSide;
use order_manager::OrderManagement;
use trader::Trader;

pub struct MatchingEngine {
    order_management: OrderManagement,
    market_data: Arc<dyn MarketData>,
    live: AtomicBool,
    thread: Option<thread::JoinHandle<()>>,
    new_order: AtomicBool,
}

impl MatchingEngine {
    pub fn new() -> Self {
        let market_data: Arc<dyn MarketData> = Arc::new(MarketData::new()); // Assuming MarketData implements Default trait or has a similar initialization
        let order_management = OrderManagement::new(Arc::clone(&market_data));

        let live = AtomicBool::new(true);
        let new_order = AtomicBool::new(false);

        let thread = Some(thread::spawn({
            let live = Arc::new(live);
            let new_order = Arc::new(new_order);
            let order_management = Arc::new(order_management);

            let order_management_c = Arc::clone(&order_management);
            let live_c = Arc::clone(&live);
            let new_order_c = Arc::clone(&new_order);

            move || {
                while live_c.load(Ordering::SeqCst) {
                    if new_order_c.load(Ordering::SeqCst) {
                        new_order_c.store(false, Ordering::SeqCst);
                        order_management_c.match_orders();
                    }
                    // Sleep or yield here to reduce CPU usage, depending on your requirements
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }));

        MatchingEngine {
            order_management,
            market_data,
            live: AtomicBool::new(true),
            thread,
            new_order: AtomicBool::new(false),
        }
    }

    pub fn add_order(
        &self,
        trader: Arc<dyn OrderDelegate>,
        contract_id: u32,
        volume: i32,
        price: f64,
        side: OrderSide,
    ) -> bool {
        let result = self
            .order_management
            .add_order(trader, contract_id, volume, price, side);
        if result {
            self.new_order.store(true, Ordering::SeqCst);
        }
        result
    }

    pub fn add_trader(&self) -> Arc<dyn OrderDelegate> {
        let trader = Arc::new(Trader::new(Arc::clone(&self)));
        self.market_data.subscribe(trader.clone());
        trader
    }

    pub fn get_order_management(&self) -> &OrderManagement {
        &self.order_management
    }

    pub fn close(&mut self) {
        if self.live.load(Ordering::SeqCst) {
            self.live.store(false, Ordering::SeqCst);
            if let Some(thread) = self.thread.take() {
                thread
                    .join()
                    .expect("Failed to join matching engine thread");
            }
            self.order_management.match_orders();
        }
    }
}

impl Drop for MatchingEngine {
    fn drop(&mut self) {
        self.close();
    }
}
