#!/usr/bin/env python3

import json
import logging
from pathlib import Path
import typing as t

import restool

RES_TYPEDOGMA = "res:/staticdata/typedogma.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPES = "res:/staticdata/types.fsdbinary"

if __name__ == "__main__":
    parser = restool.base_parser(description="Generate bounds JSON from solarsystem data.")
    parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
    args = parser.parse_args()
    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="%(asctime)s %(message)s",
    )

    fuel_efficiencies: dict[str, float] = {}
    fuel_groups: dict[str, str] = {}

    dogma = restool.extract_resource(args.root, RES_TYPEDOGMA, decode=True)
    typeID_to_efficiency: dict[int, float] = {}
    for typeID, typeData in dogma.items():
        for attr in typeData["dogmaAttributes"]:
            if attr["attributeID"] == 5607 and attr["value"] > 2:
                typeID_to_efficiency[typeID] = attr["value"]

    groups: dict[int, dict[str, t.Any]] = restool.extract_resource(args.root, RES_GROUPS, decode=True)
    groupID_to_groupName: dict[int, str] = {}
    for groupID, groupData in groups.items():
        groupID_to_groupName[groupID] = groupData["groupNameID"]

    types = restool.extract_resource(args.root, RES_TYPES, decode=True)
    for typeID, typeData in types.items():
        typeName = typeData["typeNameID"]
        if typeID in typeID_to_efficiency and typeName.endswith(" Fuel"):
            typeName = typeName.replace(" Fuel", "")
            if typeName in ("Unstable", "SOF-42", "HAK-55", "HAK-50"):
                continue
            fuel_efficiencies[typeName] = typeID_to_efficiency[typeID] / 100
            fuel_groups[typeName] = groupID_to_groupName[typeData["groupID"]]
            logging.info(f"Found fuel: {typeName} with efficiency {typeID_to_efficiency[typeID]}%")


    fuel_efficiencies = dict(sorted(fuel_efficiencies.items(), key=lambda item: item[1]))

    text = f"""
export type FuelName = {" | ".join([json.dumps(k) for k in fuel_efficiencies.keys()])};
export type Fuel = number;
export const fuels: {{ [key in FuelName]: Fuel }} = {json.dumps(fuel_efficiencies, indent=2)};
const groups = {json.dumps(fuel_groups, indent=2)};

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {{
  return groups[fuel1] === groups[fuel2];
}}
"""
    if args.output:
        args.output.write_text(text)
    else:
        print(text)
