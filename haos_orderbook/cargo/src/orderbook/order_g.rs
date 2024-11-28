use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u32,
    pub contract_id: u32,
    pub volume: i32,
    pub price: f64,
    pub side: OrderSide,
    pub owner: Arc<dyn OrderDelegate>,
}

impl Order {
    pub fn new(
        id: u32,
        contract_id: u32,
        volume: i32,
        price: f64,
        side: OrderSide,
        owner: Arc<dyn OrderDelegate>,
    ) -> Self {
        Order {
            id,
            contract_id,
            volume,
            price,
            side,
            owner,
        }
    }
}

// The trait for order delegates, which owners of orders implement
pub trait OrderDelegate: Send + Sync {
    fn on_order_execution(&self, contract_id: u32, order_id: u32, trade_volume: i32, price: f64);
}
