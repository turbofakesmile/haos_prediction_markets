pub mod reboot;

use tokio::{io::AsyncWriteExt, net::TcpListener};

use crate::orderbook::OrderBook;

#[derive(Clone, Debug)]
pub struct RebootService {
    order_book: OrderBook,
}

impl RebootService {
    pub fn new() -> Self {
        RebootService {
            order_book: OrderBook::new(),
        }
    }

    pub fn process_events(&mut self, event: OrdersMatchedEvent) {
        for match_data in event.matches {
            self.order_book
                .modify_order_volume(match_data.order_id, match_data.matched_volume);
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrdersMatchedEvent {
    pub matches: Vec<OrderMatch>,
}

#[derive(Clone, Debug)]
pub struct OrderMatch {
    pub order_id: u32,
    pub matched_volume: u32,
}

// Function to start the server
pub async fn start_order_server(config: &crate::config::ServerConfig) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.address()).await?;
    println!("Server running on {}", config.address());

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = socket.write_all(b"Welcome to the Order Server!\n").await;
        });
    }
}
