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

    pub fn modify_order_volume(&mut self, order_id: u32, matched_volume: u32) -> bool {
        let mut temp_buy = BinaryHeap::new();
        let mut temp_sell = BinaryHeap::new();
        let mut found = false;

        while let Some(order) = self.buy_orders.pop() {
            if order.id == order_id {
                found = true;
                let remaining_volume = order.volume.saturating_sub(matched_volume);
                if remaining_volume > 0 {
                    temp_buy.push(order::Order {
                        volume: remaining_volume,
                        ..order
                    });
                }
            } else {
                temp_buy.push(order);
            }
        }

        while let Some(order) = self.sell_orders.pop() {
            if order.id == order_id {
                found = true;
                let remaining_volume = order.volume.saturating_sub(matched_volume);
                if remaining_volume > 0 {
                    temp_sell.push(order::Order {
                        volume: remaining_volume,
                        ..order
                    });
                }
            } else {
                temp_sell.push(order);
            }
        }

        self.buy_orders = temp_buy;
        self.sell_orders = temp_sell;

        found
    }
}

// Implement Ord for Order to define the sorting behavior
impl Ord for order::Order {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.side, other.side) {
            (order::OrderSide::Buy, order::OrderSide::Buy) => self.price.cmp(&other.price),
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
    use super::*;
    use crate::orderbook::order::{Order, OrderSide};

    #[test]
    fn test_add_order() {
        let mut order_book = OrderBook::new();
        let order = Order {
            id: 1,
            side: OrderSide::Buy,
            price: 100,
            volume: 10,
            contract_id: 100,
        };

        let added = order_book.add_order(order.clone());
        assert!(added);
        assert_eq!(order_book.buy_orders.peek(), Some(&order));
    }

    #[test]
    fn test_match_orders() {
        let mut order_book = OrderBook::new();

        let buy_order = Order {
            id: 1,
            side: OrderSide::Buy,
            price: 100,
            volume: 10,
            contract_id: 100,
        };
        let sell_order = Order {
            id: 2,
            side: OrderSide::Sell,
            price: 90,
            volume: 10,
            contract_id: 100,
        };

        order_book.add_order(buy_order.clone());
        order_book.add_order(sell_order.clone());

        let matches = order_book.match_orders();
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&buy_order));
        assert!(matches.contains(&sell_order));
        assert!(order_book.buy_orders.is_empty());
        assert!(order_book.sell_orders.is_empty());
    }

    #[test]
    fn test_no_match_orders() {
        let mut order_book = OrderBook::new();

        let buy_order = Order {
            id: 1,
            side: OrderSide::Buy,
            price: 90,
            volume: 10,
            contract_id: 100,
        };
        let sell_order = Order {
            id: 2,
            side: OrderSide::Sell,
            price: 100,
            volume: 10,
            contract_id: 100,
        };

        order_book.add_order(buy_order);
        order_book.add_order(sell_order);

        let matches = order_book.match_orders();
        assert!(matches.is_empty());
        assert_eq!(order_book.buy_orders.len(), 1);
        assert_eq!(order_book.sell_orders.len(), 1);
    }

    #[test]
    fn test_modify_order_volume() {
        let mut order_book = OrderBook::new();

        let buy_order = Order {
            id: 1,
            side: OrderSide::Buy,
            price: 100,
            volume: 10,
            contract_id: 100,
        };

        order_book.add_order(buy_order.clone());

        let modified = order_book.modify_order_volume(1, 5);
        assert!(modified);

        let modified_order = order_book.buy_orders.peek().unwrap();
        assert_eq!(modified_order.id, 1);
        assert_eq!(modified_order.volume, 5);
    }

    #[test]
    fn test_modify_order_volume_exhausted() {
        let mut order_book = OrderBook::new();

        let sell_order = Order {
            id: 2,
            side: OrderSide::Sell,
            price: 90,
            volume: 10,
            contract_id: 100,
        };

        order_book.add_order(sell_order.clone());

        let modified = order_book.modify_order_volume(2, 10);
        assert!(modified);

        assert!(order_book.sell_orders.is_empty());
    }

    #[test]
    fn test_modify_order_volume_nonexistent() {
        let mut order_book = OrderBook::new();

        let modified = order_book.modify_order_volume(999, 5);
        assert!(!modified);
    }
}
