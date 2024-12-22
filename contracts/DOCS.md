# Haos Prediction Markets Technical Documentation

Haos Prediction Markets are the first private prediction markets that enable users to trade on the outcomes of events using encrypted tokens. Users can buy and sell "Yes" and "No" shares through a private Order Book. The accumulated cost of these shares represents the market's liquidity, and the price of a single share reflects the probability of a particular event occurring.

When an event is resolved, holders of the winning shares (e.g., "Yes" shares if the event outcome is positive) can claim the liquidity from the losing shares.

## Order Book

The **Order Book** is a smart contract where users can place orders to buy or sell "Yes" or "No" tokens (positive or negative shares, respectively). To place an order, the tokens being sold must be locked within the Order Book. When orders are matched and settled, they are filled at the maker's price, and both the maker and taker receive the bought tokens via transfer from the Order Book.

The price discovery mechanism is straightforward: after each match, the matching price is considered the current market price.


### Features
- ✅ Order Placement: Users can place orders.
- ✅ Order Settlement: The Order Matching Engine can settle matched orders.
- ✅ Token Management: The Order Book can lock and release tokens during order placement and fulfillment.
- ❌ Price Discovery Endpoint: (Work in Progress) An endpoint to retrieve current market prices.
- ❌ Order Book Factory: (Work in Progress) A factory contract for creating new Order Books.


> Note: Token locking and releasing are currently commented out due to testing library limitations with mocked Fully Homomorphic Encryption (FHE).


### Implementation
Source code: [src/OrderBook.sol](src/OrderBook.sol)

## Resolution of Market

**Market Resolution** is the process of determining the outcome of an event, i.e., whether it resolves to "Yes" or "No". This process involves a **Resolver** entity, which is currently a trusted party responsible for finalizing the event outcome.



## Encrypted Prediction Tokens
This component manages a pool of three types of encrypted tokens:

- Yes Tokens: Represent positive shares.
- No Tokens: Represent negative shares.
- USD Tokens: Used as the currency for trading shares.

The contract maintains allocations, balances, and lockups of tokens for the Order Book.

### Features
- ✅ Token Locking: The Order Book can lock tokens during the existence of an order.
- ✅ Token Transfer: The Order Book can transfer tokens after an order is filled.
- ❌ Token Minting: (Work in Progress) Market makers should be able to mint "Yes" and "No" shares in exchange for USD tokens.
    - Conversion: `1 USD ➔ 1 Yes Token + 1 No Token`
- ❌ Token Burning: (Work in Progress) "Yes" and "No" shares should be convertible back to USD tokens.
    - Conversion: `1 Yes Token + 1 No Token ➔ 1 USD`
- ❌ Market Finalization Endpoint: (Work in Progress) An endpoint for finalizing the market after resolution.

### Implementation
Source code: [src/FHERC20.sol](src/FHERC20.sol)

## Order Matching Engine

The Order Matching Engine is a trusted off-chain entity responsible for:

- Scanning the Order Book to find matching orders.
- Submitting matched orders to the blockchain for settlement.

> Order Matching Engine is currently WIP