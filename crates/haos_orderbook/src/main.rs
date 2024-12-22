mod orderbook;
use orderbook::order::{Order, OrderSide};
use orderbook::OrderBook;

fn main() {
    let mut book = OrderBook::new();

    let buy_order = Order::new(1, 1, 100, 11, OrderSide::Buy);
    let sell_order = Order::new(2, 1, 100, 10, OrderSide::Sell);

    book.add_order(buy_order);
    book.add_order(sell_order);

    let matches = book.match_orders();
    println!("Matched orders: {:?}", matches);
}
