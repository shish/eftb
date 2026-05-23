#!/usr/bin/env python3
import argparse
import json
import logging
from pathlib import Path

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate bounds JSON from solarsystem data.")
    parser.add_argument("--debug", action="store_true", help="Enable debug logging")
    parser.add_argument(
        "--input",
        "-i",
        type=Path,
        default=Path("data/solarsystems.json"),
        help="Path to downloaded solarsystem data",
    )
    parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
    args = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="%(asctime)s %(message)s",
    )

    data = json.loads(args.input.read_text())

    min_x = 0
    max_x = 0
    min_y = 0
    max_y = 0
    min_z = 0
    max_z = 0

    for star in data:
        loc = star["location"]
        min_x = min(min_x, loc["x"])
        max_x = max(max_x, loc["x"])
        min_y = min(min_y, loc["y"])
        max_y = max(max_y, loc["y"])
        min_z = min(min_z, loc["z"])
        max_z = max(max_z, loc["z"])

    bounds = {"x": [min_x, max_x], "y": [min_y, max_y], "z": [min_z, max_z]}

    if args.output:
        args.output.write_text(json.dumps(bounds, indent=2))
    else:
        print(json.dumps(bounds, indent=2))
