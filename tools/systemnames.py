#!/usr/bin/env python3
import json
import argparse
import logging
from pathlib import Path

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Fetch smartgates from the blockchain database.")
    parser.add_argument('--debug', action='store_true', help="Enable debug logging")
    parser.add_argument("--input", '-i', type=Path, default=Path('data/solarsystems.json'), help="Path to downloaded solarsystem data")
    parser.add_argument('--output', '-o', type=Path, default=None, help="Where to write data")
    args = parser.parse_args()

    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO, format='%(asctime)s %(message)s')

    data = json.loads(args.input.read_text())
    names = [ss['name'] for ss in data]
    if args.output:
        args.output.write_text(json.dumps(names, indent=2))
    else:
        print(json.dumps(names, indent=2))
