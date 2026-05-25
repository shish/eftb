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
    args = parser.parse_args()

    groups: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_GROUPS, decode=True)
    groupID_to_groupName: dict[int, str] = {}
    for groupID, groupData in groups.items():
        groupID_to_groupName[groupID] = groupData["groupNameID"]

    # Can we get these from a parent group somehow?
    ship_groups = {
        31,  # Shuttle
        237,  # Corvette
        25,  # Frigate
        420,  # Destroyer
        26,  # Cruiser
        419,  # Combat Battlecruiser
        27,  # Battleship
    }
    # Is there _nothing_ in the database that links ships to fuel types???
    hydrogen_fuel_groups = {31, 237}

    ships: dict[str, dict[str, t.Any]] = defaultdict(dict)
    dogma = td.load_dogma_attributes(args.root)
    types: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_TYPES, decode=True)
    for typeID, typeData in types.items():
        typeName = typeData["typeNameID"]
        groupID = typeData["groupID"]
        if groupID in ship_groups:
            if not typeData["published"]:
                logging.debug(f"Skipping unpublished ship: {typeName}")
                continue
            ships[typeName] = {
                "mass": typeData["mass"],
                "tank": dogma[typeID][td.DOGMA_ATTR_FUEL_CAPACITY],
                "heat": dogma[typeID][td.DOGMA_ATTR_HEAT_CAPACITY],
                "type": groupID_to_groupName[groupID],
                "fuelType": "Hydrogen" if groupID in hydrogen_fuel_groups else "Crude",
                "cargo": typeData["capacity"],
            }

    parser.output_struct(ships)


if __name__ == "__main__":
    main()
