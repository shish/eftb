#!/bin/sh
set -eu
docker run \
    --rm -ti \
    -v $(pwd)/data:/data \
	-p 3001:3001 \
	-e RPC_HTTP_URL=https://pyrope-external-sync-node-rpc.live.tech.evefrontier.com \
    -e STORE_ADDRESS=0xcdb380e0cd3949caf70c45c67079f2e27a77fc47 \
    -e SQLITE_FILENAME=/data/blockchain.db \
    ghcr.io/latticexyz/store-indexer:sha-6508c1d \
    node ./bin/sqlite-indexer.js
#    -e DEBUG="mud:*" \
