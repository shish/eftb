#!/usr/bin/env python3

# Using https://github.com/frontier-reapers/frontier-static-data
# as a reference for the file formats

import argparse
import csv
import json
import logging
import os.path
import pickle
import sys
import typing as t
from pathlib import Path

log = logging.getLogger(__name__)


def read_index_file(root: Path, index_path: Path) -> t.Dict[str, Path]:
    """
    Reads a CSV index file and returns a dictionary mapping resource names to their paths.
    """
    log.info(f"Reading index file: {index_path}")
    resources: t.Dict[str, Path] = {}
    if index_path.is_file():
        with index_path.open("r", newline="") as csvfile:
            reader = csv.reader(csvfile, delimiter=",", quotechar='"')
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

    metaIndex = read_index_file(root, root / "index_stillness.txt")
    for fn, path in metaIndex.items():
        if fn.startswith("app:/resfileindex"):
            indexFiles.append(path)
            log.debug(f"Found index file: {path}")

    resources: t.Dict[str, Path] = {}
    for index in indexFiles:
        resources.update(read_index_file(root, index))
    return resources


def extract_resource(root: Path, resource_name: str, decode: bool = False) -> bytes:
    resources = list_resources(root)
    if resource_name not in resources:
        raise FileNotFoundError(f"Resource {resource_name} not found.")

    resource_path = resources[resource_name]
    if not resource_path.is_file():
        raise FileNotFoundError(f"Resource file {resource_path.absolute()} not found.")

    if decode and resource_name.endswith(".pickle"):
        data = resource_path.read_bytes()
        struct = pickle.loads(data)
        data = (json.dumps(struct, indent=4) + "\n").encode("utf-8")
    elif decode:
        raise ValueError("Decoding is only supported for .pickle files.")
    else:
        data = resource_path.read_bytes()
    return data


def get_ef_directory() -> Path:
    for path in ["./frontier", "C:/CCP/EVE Frontier"]:
        if Path(path).is_dir():
            return Path(path)
    raise FileNotFoundError("No valid root directory found. Please specify the --root argument.")


def base_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(sys.argv[0])
    parser.add_argument("--root", type=Path, help="the root directory containing ResFiles.", default=get_ef_directory())
    parser.add_argument("--debug", action="store_true", help="enable debug logging")
    return parser


if __name__ == "__main__":
    parser = base_parser()
    subparsers = parser.add_subparsers(dest="cmd")

    list_parser = subparsers.add_parser("list")

    ex_parser = subparsers.add_parser("extract")
    ex_parser.add_argument("resource", help="the relative name of the resource file.")
    ex_parser.add_argument(
        "--unpickle",
        "-u",
        action="store_true",
        default=False,
        help="unpickle the resource file.",
    )
    ex_parser.add_argument("--output", "-o", help="file to output to", default=None)

    args = parser.parse_args()

    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO, format="%(asctime)s %(message)s")

    if args.cmd == "list":
        for file in list_resources(args.root).keys():
            print(file)

    if args.cmd == "extract":
        if not args.resource.startswith("res:"):
            args.resource = "res:" + args.resource

        data = extract_resource(args.root, args.resource, decode=args.unpickle)

        if args.output is None:
            args.output = os.path.basename(args.resource)
        if args.output == "-":
            print(data.decode("utf-8"))
        else:
            with open(args.output, "wb") as output_file:
                output_file.write(data)
