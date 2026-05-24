#!/usr/bin/env python3

import typing as t
import logging
from collections import defaultdict

import restool
import lib.type_dogma as td

RES_TYPES = "res:/staticdata/types.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPE_DOGMA = "res:/staticdata/typedogma.fsdbinary"

if __name__ == "__main__":
    parser = restool.ArgumentParser()
    args = parser.parse_args()

    engines: dict[str, dict[str, t.Any]] = defaultdict(dict)
    dogma = td.load_dogma_attributes(args.root)
    types: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_TYPES, decode=True)
    for typeID, typeData in types.items():
        typeName = typeData["typeNameID"]
        if typeData["groupID"] in (4619, 4741):
            if not typeData["published"]:
                logging.debug(f"Skipping unpublished engine: {typeName}")
                continue
            engines[typeName] = {
                "mass": typeData["mass"],
                "fuel": "SOF-40" if typeData["groupID"] == 4619 else "D1",
                # "types": [groupID_to_groupName[groups[typeData["groupID"]]["groupID"]]],
            }

    parser.output_struct(engines)
