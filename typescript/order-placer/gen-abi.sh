#!/usr/bin/env bash

set -euo pipefail

mkdir -p src/_abi
echo "export const abi =" > src/_abi/orderbook.ts
cat ../../contracts/out/OrderBook.sol/OrderBook.json | jq '.abi' >> src/_abi/orderbook.ts
truncate -s -1 src/_abi/orderbook.ts
echo -n "as const;" >> src/_abi/orderbook.ts