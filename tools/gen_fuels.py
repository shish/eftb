#!/usr/bin/env python3
import json
import argparse
from pathlib import Path
import logging


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convert fuels from API-format to app-format.")
    parser.add_argument('--debug', action='store_true', help="Enable debug logging")
    parser.add_argument("--input", '-i', type=Path, default=Path('data/fuels.json'), help="Path to the API fuels")
    parser.add_argument('--output', '-o', type=Path, default=None, help="Where to write data")
    args = parser.parse_args()

    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO, format='%(asctime)s %(message)s')

    data = json.loads(args.input.read_text())
    fuels = {}

    groups = {}
    for d in data:
        fuels[d['type']['name'].replace(" Fuel", "")] = d['efficiency'] / 100
        groups[d['type']['name'].replace(" Fuel", "")] = d['type']['groupName']
    fuels = dict(sorted(fuels.items(), key=lambda item: item[1]))

    text = f"""
export type FuelName = {" | ".join([json.dumps(k) for k in fuels.keys()])};
export type Fuel = number;
export const fuels: {{ [key in FuelName]: Fuel }} = {json.dumps(fuels, indent=2)};
const groups = {json.dumps(groups, indent=2)};

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {{
  return groups[fuel1] === groups[fuel2];
}}
"""
    if args.output:
        args.output.write_text(text)
    else:
        print(text)
