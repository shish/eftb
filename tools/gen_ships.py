#!/usr/bin/env python3

import json
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

    ship_types: list[str] = []
    groups: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_GROUPS, decode=True)
    groupID_to_groupName: dict[int, str] = {}
    for groupID, groupData in groups.items():
        if groupData["categoryID"] == 6:
            ship_types.append(groupData["groupNameID"])
        groupID_to_groupName[groupID] = groupData["groupNameID"]

    groupID_to_shipType: dict[int, str] = {
        237: "Corvette",
        31: "Shuttle",
        25: "Frigate",  # HAF
        419: "Combat Battlecruiser",  # Chumaq
    }

    ships: dict[str, dict[str, t.Any]] = defaultdict(dict)
    dogma = td.load_dogma_attributes(args.root)
    types: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_TYPES, decode=True)
    for typeID, typeData in types.items():
        typeName = typeData["typeNameID"]
        if typeData["groupID"] in (419, 25, 237):
            if not typeData["published"]:
                logging.debug(f"Skipping unpublished ship: {typeName}")
                continue
            ships[typeName] = {
                "mass": typeData["mass"],
                "tank": dogma[typeID][td.DOGMA_ATTR_FUEL_CAPACITY],
                "heat": dogma[typeID][td.DOGMA_ATTR_HEAT_CAPACITY],
                "type": groupID_to_shipType[typeData["groupID"]],
                "cargo": typeData["capacity"],
            }

    text = f"""\
export type ShipType = {" | ".join([json.dumps(k) for k in ship_types])};
export type ShipName = {" | ".join([json.dumps(k) for k in ships.keys()])};

export type Ship = {{
  mass: number;
  tank: number;
  type: ShipType;
  cargo: number;
}};
// prettier-ignore
export const ships: {{ [key in ShipName]: Ship }} = {json.dumps(ships, indent=2)} as const;

export function getEngine(type: ShipType): Engine {{
  for (const engineNameS of Object.keys(engines)) {{
    const engineName = engineNameS as EngineName;
    if (engines[engineName].types.includes(type)) {{
      return engines[engineName];
    }}
  }}
  throw new Error(`No engine found for type ${{type}}`);
}}

"""

    # parser.output(text)
    parser.output("export default " + json.dumps(ships, indent=2) + " as const;\n")
