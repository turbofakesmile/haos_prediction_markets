mod orderbook;
use orderbook::order_g::{Order, OrderSide};
use orderbook::OrderBook;
use std::sync::Arc;

fn main() {
    let mut book = OrderBook::new();
    let delegate = Arc::new(());

    let buy_order = Order::new(1, 1, 100, 10.50, OrderSide::Buy, Arc::clone(&delegate));
    let sell_order = Order::new(2, 1, 100, 10.00, OrderSide::Sell, delegate);

    book.add_order(buy_order);
    book.add_order(sell_order);

    let matches = book.match_orders();
    println!("Matched orders: {:?}", matches);
}
