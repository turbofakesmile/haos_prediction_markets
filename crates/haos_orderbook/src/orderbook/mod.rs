pub mod order;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Debug)]
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

    pub fn add_order(&mut self, order: order::Order) -> bool {
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

    pub fn match_orders(&mut self) -> Vec<order::Order> {
        let mut matches = Vec::new();
        while let Some(buy) = self.buy_orders.peek() {
            if let Some(sell) = self.sell_orders.peek() {
                if buy.price >= sell.price {
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
            // buy orders are sorted normally
            (order::OrderSide::Buy, order::OrderSide::Buy) => self.price.cmp(&other.price),
            // sell orders are sorted in reverse
            (order::OrderSide::Sell, order::OrderSide::Sell) => other.price.cmp(&self.price),
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

        let matches = book.match_orders();

        assert_eq!(matches.len(), 2);
    }
}
