all: data/blockchain.db data/smartgates.json \
	data/starmap.bin \
	data/solarsystems.json data/types.json data/fuels.json \
	src/consts/fuels.ts src/consts/systemnames.json
.PHONY: all sync-blockchain

data/blockchain.db:
	make sync-blockchain

sync-blockchain:
	docker run \
        --init --rm -ti \
        -v $(pwd)/data:/data \
        -e RPC_HTTP_URL=https://pyrope-external-sync-node-rpc.live.tech.evefrontier.com \
        -e STORE_ADDRESS=0xcdb380e0cd3949caf70c45c67079f2e27a77fc47 \
        -e SQLITE_FILENAME=/data/blockchain.db \
        ghcr.io/latticexyz/store-indexer:sha-6508c1d \
        node ./bin/sqlite-indexer.js

data/smartgates.json: data/blockchain.db tools/smartgates.py
	python3 tools/smartgates.py \
		--db data/blockchain.db \
		-o data/smartgates.json

data/starmap.json: frontier/index_stillness.txt tools/restool.py
	python3 tools/restool.py extract -u \
	    res:/staticdata/starmapcache.pickle \
		-o data/starmap.json

data/starmap.bin: data/starmap.json data/smartgates.json
	cargo run --release -- build

data/solarsystems.json: tools/api_get.py
	python3 tools/api_get.py solarsystems \
	    -o data/solarsystems.json

data/types.json: tools/api_get.py
	python3 tools/api_get.py types \
	    -o data/types.json

data/fuels.json: tools/api_get.py
	python3 tools/api_get.py fuels \
	    -o data/fuels.json

src/consts/fuels.ts: data/fuels.json
	python3 tools/fuels.py \
	    -i data/fuels.json \
	    -o src/consts/fuels.ts

src/consts/systemnames.json: data/solarsystems.json
	python3 tools/systemnames.py \
	    -i data/solarsystems.json \
	    -o src/consts/systemnames.json
