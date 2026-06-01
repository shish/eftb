#!/usr/bin/env python3

# Using https://github.com/frontier-reapers/frontier-static-data
# as a reference for the file formats

import abc
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
from typing import overload

log = logging.getLogger(__name__)

Namespace = argparse.Namespace


class ResToolBase:
    ADD_OUTPUT = True

    def __init__(self) -> None:
        # figure out defaults
        default_root = None
        for path in ["/home/shish/tmp/frontier", "C:/CCP/EVE Frontier"]:
            if Path(path).is_dir():
                default_root = Path(path)
                break

        # parse args
        parser = argparse.ArgumentParser(description=self.__doc__)
        parser.add_argument("--root", type=Path, help="the directory containing ResFiles", default=default_root)
        parser.add_argument("--debug", action="store_true", help="enable debug logging")
        if self.ADD_OUTPUT:
            parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
        self.custom_args(parser)
        args = parser.parse_args()

        # handle args
        level = logging.DEBUG if args.debug else logging.INFO
        logging.basicConfig(level=level, format="%(asctime)s %(message)s")

        if not args.root:
            raise ValueError("Root directory is required. Please specify with --root or ensure a default path exists.")
        if not (args.root / "ResFiles").is_dir():
            raise FileNotFoundError(f"Root directory {args.root} does not contain a ResFiles directory.")

        self.args = args
        self.root = args.root
        self.output_file: Path | None = args.output if self.ADD_OUTPUT else None
        self.resources = self._list_resources()
        self._strings: dict[int, str] | None = None

        self.main(self.args)

    def custom_args(self, parser: argparse.ArgumentParser) -> None: ...

    @abc.abstractmethod
    def main(self, args: argparse.Namespace) -> None: ...

    @property
    def strings(self) -> dict[int, str]:
        if self._strings is None:
            l10n_file = self.resources["res:/localizationfsd/localization_fsd_en-us.pickle"]
            _locale, strings = pickle.loads(l10n_file.read_bytes())
            log.debug(f"Loaded {len(strings)} localization strings from {l10n_file}")
            self._strings = {messageID: messages[0] for messageID, messages in strings.items()}
        return self._strings

    def _read_index_file(self, index_path: Path) -> t.Dict[str, Path]:
        """
        Reads a CSV index file and returns a dictionary mapping resource names to their paths.
        """
        log.debug(f"Reading index file: {index_path}")
        resources: t.Dict[str, Path] = {}
        if index_path.is_file():
            with index_path.open("r", newline="") as csvfile:
                reader = csv.reader(csvfile, delimiter=",", quotechar='"')
                for row in reader:
                    if len(row) >= 2:
                        resources[row[0]] = Path(self.root / "ResFiles" / row[1])
                        # log.debug(f"Added resource {row[0]} with path {row[1]}")
                    else:
                        log.warning(f"Skipping malformed row: {row}")
        else:
            log.error(f"Index file {index_path.absolute()} not found.")
        return resources

    def _list_resources(self) -> t.Dict[str, Path]:
        indexFiles: t.List[Path] = []

        metaIndex = self._read_index_file(self.root / "index_stillness.txt")
        for fn, path in metaIndex.items():
            if fn.startswith("app:/resfileindex"):
                indexFiles.append(path)
                log.debug(f"Found index file: {path}")

        resources: t.Dict[str, Path] = {}
        for index in indexFiles:
            resources.update(self._read_index_file(index))
        return resources

    @overload
    def extract_resource(self, resource_name: str, decode: t.Literal[True]) -> t.Any: ...

    @overload
    def extract_resource(self, resource_name: str, decode: t.Literal[False]) -> bytes: ...

    def extract_resource(self, resource_name: str, decode: bool = False) -> bytes | t.Any:
        if resource_name not in self.resources:
            raise FileNotFoundError(f"Resource {resource_name} not found.")

        resource_path = self.resources[resource_name]
        if not resource_path.is_file():
            raise FileNotFoundError(f"Resource file {resource_path.absolute()} not found.")

        if decode:
            if resource_name.endswith(".pickle"):
                data = resource_path.read_bytes()
                struct = pickle.loads(data)
                return struct
            elif resource_name.endswith(".fsdbinary") and platform.system() != "Windows":
                raise ValueError(
                    "Decoding .fsdbinary files is only supported on Windows due to the use of .pyd loaders."
                )
            elif resource_name.endswith(".fsdbinary"):
                bin64_path = self.root / "stillness" / "bin64"

                # bin64/myFooLoader.pyd -> {myfoo: myFooLoader}
                loaders = {lo.stem.replace("Loader", "").lower(): lo.stem for lo in bin64_path.glob("*Loader.pyd")}
                loader = loaders.get(Path(resource_name).stem.lower())
                if not loader:
                    raise ValueError(f"No loader found for {resource_name}. Available loaders: {list(loaders.keys())}")

                bin64_in_path = str(bin64_path) in sys.path
                if not bin64_in_path:
                    sys.path.insert(0, str(bin64_path))
                lib = importlib.import_module(loader)
                data = lib.load(str(resource_path))
                if not bin64_in_path:
                    sys.path.pop(0)
                struct = self.decode_cfsd(None, data)
                return struct
            else:
                raise ValueError("Decoding is only supported for .pickle and .fsdbinary files.")
        else:
            return resource_path.read_bytes()

    def decode_cfsd(self, key: str | None, data: t.Any) -> t.Any:
        """
        https://github.com/VULTUR-EveFrontier/eve-frontier-tools
        """
        data_type = type(data)

        if data_type.__module__ == "cfsd" and data_type.__name__ == "dict":
            return {k: self.decode_cfsd(k, v) for k, v in data.items()}

        if data_type.__module__.endswith("Loader"):
            return {x: self.decode_cfsd(x, getattr(data, x)) for x in dir(data) if not x.startswith("__")}

        if data_type.__module__ == "cfsd" and data_type.__name__ == "list":
            return [self.decode_cfsd(None, v) for v in data]

        if isinstance(data, tuple):
            return tuple([self.decode_cfsd(None, v) for v in data])

        if data_type.__name__.endswith("_vector"):
            # TODO: Handle vector types
            return None

        if isinstance(data, int) or data_type.__name__ == "long":
            # In case it is a NameID, look up the name
            if key is not None and isinstance(key, str) and key.lower().endswith("nameid") and key != "dungeonNameID":
                return self.strings[data][0]
            return data

        if isinstance(data, float):
            return data

        if isinstance(data, str):
            return data

        raise ValueError(f"Unknown type: {type(data)}")

    def output(self, data: str | bytes) -> None:
        if isinstance(data, str):
            data = data.encode("utf-8")

        if self.output_file is None:
            sys.stdout.buffer.write(data)
        else:
            self.output_file.write_bytes(data)

    def output_struct(self, struct: t.Any) -> None:
        if self.output_file and self.output_file.suffix in [".ts"]:
            self.output("export default " + json.dumps(struct, indent=2) + " as const;\n")
        else:
            self.output(json.dumps(struct, indent=2) + "\n")

    def load_dogma_attributes(self) -> dict[int, dict[str, float]]:
        """
        Load dogma attributes for all types.
        Returns a mapping from typeID to a dict of attribute names to values.
        """
        RES_DOGMA_ATTRIBUTES = "res:/staticdata/dogmaattributes.fsdbinary"
        RES_TYPE_DOGMA = "res:/staticdata/typedogma.fsdbinary"

        dogma_attributes = self.extract_resource(RES_DOGMA_ATTRIBUTES, decode=True)
        type_dogma = self.extract_resource(RES_TYPE_DOGMA, decode=True)
        typeID_to_attributes: dict[int, dict[str, float]] = {}
        for typeID, typeData in type_dogma.items():
            typeID_to_attributes[typeID] = {
                dogma_attributes[attr["attributeID"]]["name"]: attr["value"] for attr in typeData["dogmaAttributes"]
            }
        return typeID_to_attributes


class ResTool(ResToolBase):
    """Tool for working with EVE Frontier resource files."""

    ADD_OUTPUT = False

    def custom_args(self, parser: argparse.ArgumentParser) -> None:
        subparsers = parser.add_subparsers(dest="cmd")

        _list_parser = subparsers.add_parser("list")

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

    def main(self, args: argparse.Namespace) -> None:
        if args.cmd == "list":
            for file in self.resources.keys():
                print(file)

        if args.cmd == "extract":
            if not args.resource.startswith("res:"):
                args.resource = "res:" + args.resource

            data = self.extract_resource(args.resource, decode=args.unpickle)
            if args.unpickle:
                data = (json.dumps(data, indent=4) + "\n").encode("utf-8")

            if args.output is None:
                args.output = os.path.basename(args.resource)
            if args.output == "-":
                print(data.decode("utf-8"))
            else:
                with open(args.output, "wb") as output_file:
                    output_file.write(data)
