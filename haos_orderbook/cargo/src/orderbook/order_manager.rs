use crossbeam_queue::ArrayQueue;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

// Assume these types are defined elsewhere or in separate modules
use mdata::MarketData;
use order_delegate::OrderDelegate;
use orderbook::{Order, OrderBook};
use trader::Trader;

pub struct OrderManagement {
    delegate: Arc<dyn MarketData>,
    order_books: HashMap<u32, OrderBook>,
    queue: ArrayQueue<Order>,
    order_id: u32,
    total_volume: i32,
    total_traded_volume: i32,
}

impl OrderManagement {
    pub fn new(delegate: Arc<dyn MarketData>) -> Self {
        OrderManagement {
            delegate,
            order_books: HashMap::new(),
            queue: ArrayQueue::new(1000), // Adjust size as per requirement
            order_id: 0,
            total_volume: 0,
            total_traded_volume: 0,
        }
    }

    pub fn add_order(
        &mut self,
        trader: Arc<dyn OrderDelegate>,
        contract_id: u32,
        volume: i32,
        price: f64,
        side: OrderSide,
    ) -> bool {
        if volume < 1 {
            return false;
        }

        // Wait for queue availability with a timeout
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        while !self.queue.is_full() {
            if SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                - start
                > 1000
            {
                return false; // Timeout
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let order = Order::new(self.order_id, contract_id, volume, price, side, trader);
        self.order_id += 1;
        self.total_volume += volume;

        if self.queue.push(order).is_err() {
            return false; // Failed to add to queue
        }

        true
    }

    pub fn match_orders(&mut self) -> bool {
        // Early return if there are no orders to match
        if self.queue.is_empty() {
            return false;
        }

        let mut contract_ids = std::collections::HashSet::new();
        while let Ok(order) = self.queue.pop() {
            contract_ids.insert(order.contract_id);
            self.order_books
                .entry(order.contract_id)
                .or_insert_with(OrderBook::new)
                .add_order(order);
        }

        for &contract_id in &contract_ids {
            if let Some(order_book) = self.order_books.get_mut(&contract_id) {
                while let (Some(buy_order), Some(sell_order)) = (
                    order_book.m_buy_orders.peek(),
                    order_book.m_sell_orders.peek(),
                ) {
                    if buy_order.price >= sell_order.price {
                        let trade_volume = buy_order.volume.min(sell_order.volume);
                        if trade_volume == 0 {
                            break; // Avoid matching if volume is zero
                        }

                        // Execute trade
                        self.execute_trade(
                            contract_id,
                            trade_volume,
                            buy_order.price,
                            buy_order.clone(),
                            sell_order.clone(),
                        );

                        // Update volumes
                        let buy_order = order_book.m_buy_orders.pop().unwrap();
                        let sell_order = order_book.m_sell_orders.pop().unwrap();
                        if let Some(mut updated_buy) = buy_order {
                            updated_buy.volume -= trade_volume;
                            if updated_buy.volume > 0 {
                                order_book.m_buy_orders.push(updated_buy);
                            }
                        }
                        if let Some(mut updated_sell) = sell_order {
                            updated_sell.volume -= trade_volume;
                            if updated_sell.volume > 0 {
                                order_book.m_sell_orders.push(updated_sell);
                            }
                        }
                    } else {
                        break; // No more matches possible
                    }
                }
                // Publish the order book after matching
                self.delegate.publish_order_book(order_book);
            }
        }
        true
    }

    fn execute_trade(
        &mut self,
        contract_id: u32,
        trade_volume: i32,
        price: f64,
        buy_order: Order,
        sell_order: Order,
    ) {
        // Notify traders about the execution
        buy_order
            .owner
            .on_order_execution(contract_id, buy_order.id, trade_volume, price);
        sell_order
            .owner
            .on_order_execution(contract_id, sell_order.id, trade_volume, price);

        // Increment traded volume
        self.total_traded_volume += trade_volume;

        // Publish public trade
        self.delegate
            .publish_public_trade(contract_id, buy_order.id, trade_volume, price);
    }
}

#[derive(Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}
