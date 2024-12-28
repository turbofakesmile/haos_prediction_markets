#!/usr/bin/env bash

set -euo pipefail

cat ../../contracts/out/MockedOrderBook.sol/MockedOrderBook.json | jq '.abi' >> orderbook.abi
# truncate -s -1 orderbook.abi