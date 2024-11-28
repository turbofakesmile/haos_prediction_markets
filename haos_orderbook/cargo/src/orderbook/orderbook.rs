mod order_g;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Arc;

#[derive(Debug)]
pub struct OrderBook {
    buy_orders: BinaryHeap<order::Order>,
    sell_orders: BinaryHeap<order::Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: BinaryHeap::new(),
            sell_orders: BinaryHeap::new(),
        }
    }

    pub fn add_order(&mut self, mut order: order::Order) -> bool {
        match order.side {
            order::OrderSide::Buy => {
                self.buy_orders.push(order);
            }
            order::OrderSide::Sell => {
                // Invert price for sell orders to use BinaryHeap (max-heap) as min-heap for price
                order.price = -order.price;
                self.sell_orders.push(order);
            }
        }
        true
    }

    pub fn match_orders(&mut self) -> Vec<order::Order> {
        let mut matches = Vec::new();
        while let Some(buy) = self.buy_orders.peek() {
            if let Some(sell) = self.sell_orders.peek() {
                if buy.price >= -sell.price {
                    matches.push(self.buy_orders.pop().unwrap());
                    matches.push(self.sell_orders.pop().unwrap());
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        matches
    }
}

// Implement Ord for Order to define the sorting behavior
impl Ord for order::Order {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.side, other.side) {
            (order::OrderSide::Buy, order::OrderSide::Buy) => self.price.cmp(&other.price),
            (order::OrderSide::Sell, order::OrderSide::Sell) => other.price.cmp(&self.price), // Remember price is inverted for sell orders
            _ => Ordering::Equal, // This case should not occur as orders are segregated
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
