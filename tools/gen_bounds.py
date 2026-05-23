#!/usr/bin/env python3

import json
import logging
from pathlib import Path

import restool

if __name__ == "__main__":
    parser = restool.base_parser(description="Generate bounds JSON from solarsystem data.")
    parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
    args = parser.parse_args()
    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="%(asctime)s %(message)s",
    )

    data = restool.extract_resource(args.root, "res:/staticdata/starmapcache.pickle", decode=True)

    min_x = 0
    max_x = 0
    min_y = 0
    max_y = 0
    min_z = 0
    max_z = 0

    for star in data["solarSystems"].values():
        x, y, z = star["center"]
        min_x = min(min_x, int(x))
        max_x = max(max_x, int(x))
        min_y = min(min_y, int(y))
        max_y = max(max_y, int(y))
        min_z = min(min_z, int(z))
        max_z = max(max_z, int(z))

    bounds = {"x": [min_x, max_x], "y": [min_y, max_y], "z": [min_z, max_z]}

    if args.output:
        args.output.write_text(json.dumps(bounds, indent=2))
    else:
        print(json.dumps(bounds, indent=2))
