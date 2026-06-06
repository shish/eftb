#!/usr/bin/env python3

import logging
import typing as t
from collections import defaultdict

import restool

RES_TYPES = "res:/staticdata/types.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"


class GenShips(restool.ResToolBase):
    def tool_main(self, args: restool.Namespace) -> None:
        groups: dict[int, dict[str, t.Any]] = self.extract_resource(RES_GROUPS, decode=True)
        groupID_to_groupName: dict[int, str] = {}
        for groupID, groupData in groups.items():
            groupID_to_groupName[groupID] = groupData["_groupNameID"]

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
        dogma = self.load_dogma_attributes()
        types: dict[int, dict[str, t.Any]] = self.extract_resource(RES_TYPES, decode=True)
        for typeID, typeData in types.items():
            typeName = self.strings[typeData["typeNameID"]]
            groupID = typeData["groupID"]
            if groupID in ship_groups:
                if not typeData["published"]:
                    logging.debug(f"Skipping unpublished ship: {typeName}")
                    continue
                ships[typeName] = {
                    "mass": typeData["mass"],
                    "tank": dogma[typeID]["fuelCapacity"],
                    "heat": dogma[typeID]["heatCapacity"],
                    "type": groupID_to_groupName[groupID],
                    "fuelType": "Hydrogen" if groupID in hydrogen_fuel_groups else "Crude",
                    "cargo": typeData["capacity"],
                }

        self.output_struct(ships)


def main() -> None:
    GenShips().main()


if __name__ == "__main__":
    main()
