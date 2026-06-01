#!/usr/bin/env python3

import logging
import typing as t
from collections import defaultdict

import lib.type_dogma as td
import restool

RES_TYPES = "res:/staticdata/types.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPE_DOGMA = "res:/staticdata/typedogma.fsdbinary"


def main() -> None:
    parser = restool.ArgumentParser()
    args = parser.parse_args_and_setup()

    groups: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_GROUPS, decode=True)
    groupID_to_groupName: dict[int, str] = {}
    for groupID, groupData in groups.items():
        groupID_to_groupName[groupID] = groupData["groupNameID"]

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
                "fuelType": "Crude" if typeData["groupID"] == 4619 else "Hydrogen",
                "canFitShipGroups": [
                    groupID_to_groupName[int(x)]
                    for x in [
                        dogma[typeID].get(td.DOGMA_ATTR_CAN_FIT_SHIP_GROUP_01),
                        dogma[typeID].get(td.DOGMA_ATTR_CAN_FIT_SHIP_GROUP_02),
                        dogma[typeID].get(td.DOGMA_ATTR_CAN_FIT_SHIP_GROUP_03),
                    ]
                    if x is not None
                ],
                # "types": [groupID_to_groupName[groups[typeData["groupID"]]["groupID"]]],
            }

    parser.output_struct(engines)


if __name__ == "__main__":
    main()
