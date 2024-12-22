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
    pub price: u32,
    pub side: OrderSide,
}

impl Order {
    pub fn new(
        id: u32,
        contract_id: u32,
        volume: i32,
        price: u32,
        side: OrderSide,
    ) -> Self {
        Order {
            id,
            contract_id,
            volume,
            price,
            side,
        }
    }
}

