.DEFAULT_GOAL := all
.PHONY: all

export UV_PROJECT=../tools

all: src/consts/bounds.ts
src/consts/bounds.ts: ../tools/src/gen_bounds.py
	uv run gen_bounds \
	    -o src/consts/bounds.ts

all: src/consts/engines_data.ts
src/consts/engines_data.ts: ../tools/src/gen_engines.py
	uv run gen_engines \
	    -o src/consts/engines_data.ts

all: src/consts/fuels_data.ts
src/consts/fuels_data.ts: ../tools/src/gen_fuels.py
	uv run gen_fuels \
	    -o src/consts/fuels_data.ts

all: src/consts/ships_data.ts
src/consts/ships_data.ts: ../tools/src/gen_ships.py
	uv run gen_ships \
	    -o src/consts/ships_data.ts

all: src/consts/structures_data.ts
src/consts/structures_data.ts: ../tools/src/gen_structures.py
	uv run gen_structures \
	    -o src/consts/structures_data.ts

all: src/consts/system_names.json
src/consts/system_names.json: ../tools/src/gen_system_names.py
	uv run gen_system_names \
	    -o src/consts/system_names.json
