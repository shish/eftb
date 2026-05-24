.DEFAULT_GOAL := all
.PHONY: all

# all: data/smartgates.json
# data/starmap.json: frontier/index_stillness.txt
data/starmap.json: tools/restool.py
	uv run tools/restool.py extract -u \
	    res:/staticdata/starmapcache.pickle \
		-o data/starmap.json

# all: data/solarsystems.json
data/starmap.rkyv: data/starmap.json data/smartgates.json
	cargo run --release -- build

# all: data/smartgates.json
data/solarsystems.json: tools/apitool.py
	uv run tools/apitool.py solarsystems \
	    -o data/solarsystems.json

# all: data/types.json
data/types.json: tools/apitool.py
	uv run tools/apitool.py types \
	    -o data/types.json

# all: data/fuels.json
data/fuels.json: tools/apitool.py
	uv run tools/apitool.py fuels \
	    -o data/fuels.json

all: src/consts/bounds.ts
src/consts/bounds.ts: tools/gen_bounds.py
	uv run tools/gen_bounds.py \
	    -o src/consts/bounds.ts

all: src/consts/engines_data.ts
src/consts/engines_data.ts: tools/gen_engines.py
	uv run tools/gen_engines.py \
	    -o src/consts/engines_data.ts

all: src/consts/fuels_data.ts
src/consts/fuels_data.ts: tools/gen_fuels.py
	uv run tools/gen_fuels.py \
	    -o src/consts/fuels_data.ts

all: src/consts/ships_data.ts
src/consts/ships_data.ts: tools/gen_ships.py
	uv run tools/gen_ships.py \
	    -o src/consts/ships_data.ts

all: src/consts/system_names.json
src/consts/system_names.json: tools/gen_system_names.py
	uv run tools/gen_system_names.py \
	    -o src/consts/system_names.json



.PHONY: check
check: lint format-check typecheck

lint:
	uv run ruff check tools/

format:
	uv run ruff format tools/

format-check:
	uv run ruff format --check tools/

typecheck:
	uv run mypy tools/
