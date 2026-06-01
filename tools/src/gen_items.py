#!/usr/bin/env python3

import logging
import typing as t

import restool

RES_TYPES = "res:/staticdata/types.fsdbinary"


class GenItems(restool.ResTool):
    def main(self, args: restool.Namespace) -> None:
        # Hard-coded list of items to extract
        item_names = [
            "Unstable Fuel",
            "D1 Fuel",
            "D2 Fuel",
            "SOF-40 Fuel",
            "EU-40 Fuel",
            "SOF-80 Fuel",
            "EU-90 Fuel",
            "Feldspar Crystals",
            "Platinum-Palladium Matrix",
            "Hydrated Sulfide Matrix",
            "Building Foam",
            "Printed Circuits",
            "Reinforced Alloys",
            "Carbon Weave",
            "Thermal Composites",
            "Exclave Technocore",
            "Synod Technocore",
        ]

        types: dict[int, dict[str, t.Any]] = self.extract_resource(RES_TYPES, decode=True)

        items: dict[str, dict[str, t.Any]] = {}

        for typeID, typeData in types.items():
            typeName = typeData["typeNameID"]
            if typeName in item_names:
                items[typeName] = {
                    "volume": typeData["volume"],
                    "mass": typeData["mass"],
                }
                logging.debug(f"Found item: {typeName} with volume {typeData['volume']} and mass {typeData['mass']}")

        # Sort items by the order in the original list
        items = {name: items[name] for name in item_names if name in items}

        self.output_struct(items)
