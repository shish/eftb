.DEFAULT_GOAL := all
.PHONY: all

export UV_PROJECT=../tools

all: data/starmap.json
data/starmap.json: ../tools/src/gen_starmap.py ../tools/src/restool.py
	uv run gen_starmap \
	    -o data/starmap.json
