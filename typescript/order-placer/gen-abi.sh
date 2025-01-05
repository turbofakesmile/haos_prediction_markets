#!/usr/bin/env bash

set -euo pipefail

mkdir -p src/_abi
echo "export const abi =" > src/_abi/orderbook.ts
cat ../../contracts/out/OrderBook.sol/OrderBook.json | jq '.abi' >> src/_abi/orderbook.ts
truncate -s -1 src/_abi/orderbook.ts
echo -n "as const;" >> src/_abi/orderbook.ts

mkdir -p src/_abi
echo "export const abi =" > src/_abi/token.ts
cat ../../contracts/out/FHERC20.sol/EncryptedToken.json | jq '.abi' >> src/_abi/token.ts
truncate -s -1 src/_abi/token.ts
echo -n "as const;" >> src/_abi/token.ts