.DEFAULT_GOAL := all
.PHONY: all

export UV_PROJECT=../tools

# data/starmap.json: frontier/index_stillness.txt
all: data/starmap.json
data/starmap.json: ../tools/src/restool.py
	uv run restool extract -u \
	    res:/staticdata/starmapcache.pickle \
		-o data/starmap.json

all: data/starmap.rkyv
data/starmap.rkyv: data/starmap.json data/smartgates.json
	cargo run --release -- build

#all: data/solarsystems.json
#data/solarsystems.json: ../tools/apitool.py
#	uv run --project ../tools ../tools/apitool.py solarsystems \
#	    -o data/solarsystems.json

#all: data/types.json
#data/types.json: ../tools/apitool.py
#	uv run --project ../tools ../tools/apitool.py types \
#	    -o data/types.json

#all: data/fuels.json
#data/fuels.json: ../tools/apitool.py
#	uv run --project ../tools ../tools/apitool.py fuels \
#	    -o data/fuels.json
