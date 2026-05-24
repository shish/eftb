#!/usr/bin/env python3

import logging
import typing as t

import restool
import lib.type_dogma as td

RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPES = "res:/staticdata/types.fsdbinary"

if __name__ == "__main__":
    parser = restool.ArgumentParser(description="Generate bounds JSON from solarsystem data.")
    args = parser.parse_args()

    groupID_to_groupName: dict[int, str] = {}
    groups: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_GROUPS, decode=True)
    for groupID, groupData in groups.items():
        groupID_to_groupName[groupID] = groupData["groupNameID"]

    fuels: dict[str, dict[str, t.Any]] = {}
    types = restool.extract_resource(args.root, RES_TYPES, decode=True)
    dogma = td.load_dogma_attributes(args.root)
    for typeID, typeData in types.items():
        typeName = typeData["typeNameID"]
        if typeName.endswith(" Fuel"):
            if not typeData["published"]:
                logging.debug(f"Skipping unpublished fuel: {typeName}")
                continue
            fuels[typeName.replace(" Fuel", "")] = {
                "efficiency": dogma[typeID][td.DOGMA_ATTR_FUEL_EFFICIENCY] / 100,
                "group": groupID_to_groupName[typeData["groupID"]],
            }
            logging.debug(
                f"Found fuel: {typeName} with efficiency {dogma[typeID][td.DOGMA_ATTR_FUEL_EFFICIENCY] / 100}%"
            )
    fuels = dict(sorted(fuels.items(), key=lambda item: item[1]["efficiency"], reverse=True))

    parser.output_struct(fuels)
