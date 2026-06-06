#!/usr/bin/env python3

import logging
import typing as t

import restool

RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPES = "res:/staticdata/types.fsdbinary"


class GenFuels(restool.ResToolBase):
    def tool_main(self, args: restool.Namespace) -> None:
        groupID_to_groupName: dict[int, str] = {}
        groups: dict[int, dict[str, t.Any]] = self.extract_resource(RES_GROUPS, decode=True)
        for groupID, groupData in groups.items():
            groupID_to_groupName[groupID] = groupData["_groupNameID"]

        fuels: dict[str, dict[str, t.Any]] = {}
        types = self.extract_resource(RES_TYPES, decode=True)
        dogma = self.load_dogma_attributes()
        for typeID, typeData in types.items():
            typeName = typeData["_typeNameID"]
            if typeData.get("marketGroupID") == 3559 or typeID == 77818:  # Ship Engine Fuel
                if not typeData["published"]:
                    logging.debug(f"Skipping unpublished fuel: {typeName}")
                    continue
                fuels[typeName.replace(" Fuel", "")] = {
                    "efficiency": dogma[typeID]["fuelEfficiency"] / 100,
                    "fuelType": groupID_to_groupName[typeData["groupID"]].replace(" Fuel", ""),
                }
                logging.debug(f"Found fuel: {typeName} with efficiency {dogma[typeID]['fuelEfficiency'] / 100}%")
        fuels = dict(sorted(fuels.items(), key=lambda item: item[1]["efficiency"], reverse=True))

        self.output_struct(fuels)


def main() -> None:
    GenFuels().main()


if __name__ == "__main__":
    main()
