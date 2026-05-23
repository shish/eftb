#!/usr/bin/env python3

# Using https://github.com/frontier-reapers/frontier-static-data
# as a reference for the file formats

import argparse
import csv
import importlib
import json
import logging
import os.path
import pickle
import platform
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
    elif decode and resource_name.endswith(".fsdbinary") and platform.system() != "Windows":
        raise ValueError("Decoding .fsdbinary files is only supported on Windows due to the use of .pyd loaders.")
    elif decode and resource_name.endswith(".fsdbinary"):
        bin64_path = root / "stillness" / "bin64"

        # bin64/myFooLoader.pyd -> {myfoo: myFooLoader}
        loaders = {l.stem.replace("Loader", "").lower(): l.stem for l in bin64_path.glob("*Loader.pyd")}
        loader = loaders.get(Path(resource_name).stem.lower())
        if not loader:
            raise ValueError(f"No loader found for {resource_name}. Available loaders: {list(loaders.keys())}")

        l10n_file = resources["res:/localizationfsd/localization_fsd_en-us.pickle"]
        _locale, strings = pickle.loads(l10n_file.read_bytes())
        log.debug(f"Loaded {len(strings)} localization strings from {l10n_file}")

        bin64_in_path = str(bin64_path) in sys.path
        if not bin64_in_path:
            sys.path.insert(0, str(bin64_path))
        lib = importlib.import_module(loader)
        data = lib.load(str(resource_path))
        if not bin64_in_path:
            sys.path.pop(0)
        struct = decode_cfsd(None, data, strings)
        data = (json.dumps(struct, indent=4) + "\n").encode("utf-8")

    elif decode:
        raise ValueError("Decoding is only supported for .pickle and .fsdbinary files.")
    else:
        data = resource_path.read_bytes()
    return data


def decode_cfsd(key: str | None, data: t.Any, strings: dict[int, list[str]]) -> t.Any:
    """
    https://github.com/VULTUR-EveFrontier/eve-frontier-tools
    """
    data_type = type(data)

    if data_type.__module__ == "cfsd" and data_type.__name__ == "dict":
        return {k: decode_cfsd(k, v, strings) for k, v in data.items()}

    if data_type.__module__.endswith("Loader"):
        return {x: decode_cfsd(x, getattr(data, x), strings) for x in dir(data) if not x.startswith("__")}

    if data_type.__module__ == "cfsd" and data_type.__name__ == "list":
        return [decode_cfsd(None, v, strings) for v in data]

    if isinstance(data, tuple):
        return tuple([decode_cfsd(None, v, strings) for v in data])

    if data_type.__name__.endswith("_vector"):
        # TODO: Handle vector types
        return None

    if isinstance(data, int) or data_type.__name__ == "long":
        # In case it is a NameID, look up the name
        if key is not None and key.lower().endswith("nameid") and key != "dungeonNameID":
            return strings[data][0]
        return data

    if isinstance(data, float):
        return data

    if isinstance(data, str):
        return data

    raise ValueError(f"Unknown type: {type(data)}")


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
