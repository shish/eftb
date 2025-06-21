#!/usr/bin/env python3

# Using https://github.com/frontier-reapers/frontier-static-data
# as a reference for the file formats

import pickle
import argparse
import sys
import csv
import typing as t
import logging
import os.path
from pathlib import Path
import json

log = logging.getLogger(__name__)


def read_index_file(root: Path, index_path: Path) -> t.Dict[str, Path]:
    """
    Reads a CSV index file and returns a dictionary mapping resource names to their paths.
    """
    log.info(f"Reading index file: {index_path}")
    resources: t.Dict[str, Path] = {}
    if index_path.is_file():
        with index_path.open('r', newline='') as csvfile:
            reader = csv.reader(csvfile, delimiter=',', quotechar='"')
            for row in reader:
                if len(row) >= 2:
                    resources[row[0]] = Path(root / "ResFiles" / row[1])
                    log.debug(f"Added resource {row[0]} with path {row[1]}")
                else:
                    log.warning(f"Skipping malformed row: {row}")
    else:
        log.error(f"Index file {index_path.absolute()} not found.")
    return resources


def list_resources(root: Path) -> t.Dict[str, Path]:
    indexFiles: t.List[Path] = []

    metaIndex = read_index_file(root, root / 'index_stillness.txt')
    for fn, path in metaIndex.items():
        if fn.startswith('app:/resfileindex'):
            indexFiles.append(path)
            log.debug(f"Found index file: {path}")

    resources: t.Dict[str, Path] = {}
    for index in indexFiles:
        resources.update(read_index_file(root, index))
    return resources


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, format='%(asctime)s %(message)s')

    defpath = None
    for path in [
        "./frontier",
        "C:/CCP/EVE Frontier"
    ]:
        if Path(path).is_dir():
            defpath = Path(path)
            break
    if defpath is None:
        log.error("No valid root directory found. Please specify the --root argument.")
        sys.exit(1)

    parser = argparse.ArgumentParser(sys.argv[0])
    parser.add_argument('--root', type=Path, help='the root directory containing ResFiles.', default=defpath)
    subparsers = parser.add_subparsers(dest='cmd')

    list_parser = subparsers.add_parser('list')

    ex_parser = subparsers.add_parser('extract')
    ex_parser.add_argument('resource', help='the relative name of the resource file.')
    ex_parser.add_argument('--unpickle', '-u', action='store_true', default=False, help='unpickle the resource file.',)
    ex_parser.add_argument('--output', '-o', help='file to output to', default=None)

    args = parser.parse_args()

    if args.cmd == "list":
        for file in list_resources(args.root).keys():
            print(file)

    if args.cmd == "extract":
        files = list_resources(args.root)
        data = files[args.resource].read_bytes()

        if args.unpickle:
            struct = pickle.loads(data)
            data = (json.dumps(struct, indent=4) + "\n").encode('utf-8')

        if args.output is None:
            args.output = os.path.basename(args.resource)
        if args.output == "-":
            print(data.decode('utf-8'))
        else:
            with open(args.output, 'wb') as output_file:
                output_file.write(data)
