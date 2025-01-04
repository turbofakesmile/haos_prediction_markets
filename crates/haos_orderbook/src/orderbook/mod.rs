pub mod order;
use std::{cmp::Ordering, collections::BinaryHeap};

use tracing::info;

#[derive(Clone, Debug)]
pub struct OrderBook {
    buy_orders: BinaryHeap<order::Order>,
    sell_orders: BinaryHeap<order::Order>,
}

#[derive(Clone, Debug)]
pub struct MatchedOrders {
    pub taker_order_id: u32,
    pub maker_order_id: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: BinaryHeap::new(),
            sell_orders: BinaryHeap::new(),
        }
    }

    pub fn add_order(&mut self, order: order::Order) -> bool {
        if order.volume == 0 {
            return false;
        }
        match order.side {
            order::OrderSide::Buy => {
                self.buy_orders.push(order);
            }
            order::OrderSide::Sell => {
                self.sell_orders.push(order);
            }
        }
        true
    }

    pub fn remove_order(&mut self, id: u32) -> bool {
        let mut found = false;

        // remove the order from the buy orders
        let mut buy_orders = Vec::new();
        while let Some(order) = self.buy_orders.pop() {
            if order.id == id {
                found = true;
            } else {
                buy_orders.push(order);
            }
        }
        self.buy_orders = BinaryHeap::from(buy_orders);

        // remove the order from the sell orders
        let mut sell_orders = Vec::new();
        while let Some(order) = self.sell_orders.pop() {
            if order.id == id {
                found = true;
            } else {
                sell_orders.push(order);
            }
        }
        self.sell_orders = BinaryHeap::from(sell_orders);

        found
    }

    // update an order in the orderbook
    pub fn update_order(&mut self, order: order::Order) -> bool {
        // if the order is not in the orderbook, return false
        if !self.remove_order(order.id) {
            info!("Order not found in orderbook, creating new order");
        } else {
            info!("Order found in orderbook, updating order");
        }

        if order.volume > 0 {
            self.add_order(order);
        }

        true
    }

    // find two orders that match
    pub fn find_matching_orders(&self) -> Option<MatchedOrders> {
        let buy = self.buy_orders.peek();
        let sell = self.sell_orders.peek();

        match (buy, sell) {
            (Some(buy), Some(sell)) => {
                if buy.price >= sell.price {
                    // the order with the lower id is the maker order
                    if buy.id < sell.id {
                        return Some(MatchedOrders {
                            taker_order_id: sell.id,
                            maker_order_id: buy.id,
                        });
                    }
                    return Some(MatchedOrders {
                        taker_order_id: buy.id,
                        maker_order_id: sell.id,
                    });
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// Implement Ord for Order to define the sorting behavior
impl Ord for order::Order {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.side, other.side) {
            // buy orders are sorted by price, then by id
            // first in the heap is the highest price, then the lowest id to ensure FIFO
            (order::OrderSide::Buy, order::OrderSide::Buy) => self
                .price
                .cmp(&other.price)
                .then_with(|| other.id.cmp(&self.id)),
            // sell orders are sorted in reverse
            // first in the heap is the lowest price, then the lowest id to ensure FIFO
            (order::OrderSide::Sell, order::OrderSide::Sell) => other
                .price
                .cmp(&self.price)
                .then_with(|| other.id.cmp(&self.id)),
            _ => panic!("Cannot compare buy and sell orders, should be separate heaps"),
        }
    }
}

// PartialOrd is required by Ord
impl PartialOrd for order::Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// PartialEq is required by BinaryHeap
impl PartialEq for order::Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

// Eq is required by BinaryHeap when using PartialOrd
impl Eq for order::Order {}

#[cfg(test)]
mod tests {
    use crate::orderbook::{
        order::{Order, OrderSide},
        OrderBook,
    };

    #[test]
    fn test_add_match() {
        let mut book = OrderBook::new();

        let buy_order = Order::new(1, 1, 100, 11, OrderSide::Buy);
        let sell_order = Order::new(2, 1, 100, 10, OrderSide::Sell);

        book.add_order(buy_order);
        book.add_order(sell_order);

        let matches = book.find_matching_orders();
        assert!(matches.is_some());
        let matches = matches.unwrap();
        assert_eq!(matches.maker_order_id, 1);
        assert_eq!(matches.taker_order_id, 2);
    }
}
