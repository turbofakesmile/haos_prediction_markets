#!/usr/bin/env bash

set -euo pipefail

cat ../../contracts/out/OrderBook.sol/OrderBook.json | jq '.abi' > orderbook.abi
# truncate -s -1 orderbook.abi