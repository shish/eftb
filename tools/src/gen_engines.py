#!/usr/bin/env python3

import logging
import typing as t
from collections import defaultdict

import restool

RES_TYPES = "res:/staticdata/types.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"


class GenEngines(restool.ResTool):
    def main(self, args: restool.Namespace) -> None:
        groups: dict[int, dict[str, t.Any]] = self.extract_resource(RES_GROUPS, decode=True)
        groupID_to_groupName: dict[int, str] = {}
        for groupID, groupData in groups.items():
            groupID_to_groupName[groupID] = groupData["groupNameID"]

        engines: dict[str, dict[str, t.Any]] = defaultdict(dict)
        dogma = self.load_dogma_attributes()
        types: dict[int, dict[str, t.Any]] = self.extract_resource(RES_TYPES, decode=True)
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
                            dogma[typeID].get("canFitShipGroup01"),
                            dogma[typeID].get("canFitShipGroup02"),
                            dogma[typeID].get("canFitShipGroup03"),
                        ]
                        if x is not None
                    ],
                    # "types": [groupID_to_groupName[groups[typeData["groupID"]]["groupID"]]],
                }

        self.output_struct(engines)
