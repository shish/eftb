.DEFAULT_GOAL := all
.PHONY: all

export UV_PROJECT=../tools

all: src/consts/bounds_data.ts
src/consts/bounds_data.ts: ../tools/src/gen_bounds.py ../tools/src/restool.py
	uv run gen_bounds \
	    -o src/consts/bounds_data.ts

all: src/consts/engines_data.ts
src/consts/engines_data.ts: ../tools/src/gen_engines.py ../tools/src/restool.py
	uv run gen_engines \
	    -o src/consts/engines_data.ts

all: src/consts/fuels_data.ts
src/consts/fuels_data.ts: ../tools/src/gen_fuels.py ../tools/src/restool.py
	uv run gen_fuels \
	    -o src/consts/fuels_data.ts

all: src/consts/ships_data.ts
src/consts/ships_data.ts: ../tools/src/gen_ships.py ../tools/src/restool.py
	uv run gen_ships \
	    -o src/consts/ships_data.ts

all: src/consts/structures_data.ts
src/consts/structures_data.ts: ../tools/src/gen_structures.py ../tools/src/restool.py
	uv run gen_structures \
	    -o src/consts/structures_data.ts

all: src/consts/system_names.json
src/consts/system_names.json: ../tools/src/gen_system_names.py ../tools/src/restool.py
	uv run gen_system_names \
	    -o src/consts/system_names.json

all: src/consts/items_data.ts
src/consts/items_data.ts: ../tools/src/gen_items.py ../tools/src/restool.py
	uv run gen_items \
	    -o src/consts/items_data.ts
