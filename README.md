# Prediction Markets

## Running with Docker

The project includes Docker support for running the order-scanner and haos_orderbook services together. Follow these
steps to get started:

1. Copy the example environment file:

```bash
cp .env.example .env
```

2. Edit the `.env` file and fill in your configuration:

- `PRIVATE_KEY`: Private key for the orderbook service
- `CONTRACT_ADDRESS`: Address of the deployed OrderBook contract
- `START_BLOCK`: Block number to start scanning from
- `ORDER_BOOK_URL`: Fhenix RPC URL

3. Build and start the services:

```bash
docker compose up --build
```

This will start both services:

- order-scanner on port 3000
- haos_orderbook connected to the order-scanner

To stop the services:

```bash
docker compose down
```

## Development

For local development without Docker, refer to the individual service directories:

- [order-scanner](typescript/order-scanner/README.md)
- [haos_orderbook](crates/haos_orderbook/README.md)
