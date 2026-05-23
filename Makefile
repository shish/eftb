all: data/smartgates.json \
	data/starmap.rkyv \
	data/solarsystems.json data/types.json data/fuels.json \
	src/consts/fuels.ts src/consts/systemnames.json src/consts/bounds.json
.PHONY: all

data/starmap.json: frontier/index_stillness.txt tools/restool.py
	python3 tools/restool.py extract -u \
	    res:/staticdata/starmapcache.pickle \
		-o data/starmap.json

data/starmap.rkyv: data/starmap.json data/smartgates.json
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
	python3 tools/gen_fuels.py \
	    -i data/fuels.json \
	    -o src/consts/fuels.ts

src/consts/systemnames.json: data/solarsystems.json
	python3 tools/gen_systemnames.py \
	    -i data/solarsystems.json \
	    -o src/consts/systemnames.json

src/consts/bounds.json: data/solarsystems.json
	python3 tools/gen_bounds.py \
	    -i data/solarsystems.json \
	    -o src/consts/bounds.json
