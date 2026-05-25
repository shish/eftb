from pathlib import Path

import restool

RES_TYPE_DOGMA = "res:/staticdata/typedogma.fsdbinary"
RES_DOGMA_ATTRIBUTES = "res:/staticdata/dogmaattributes.fsdbinary"
DOGMA_ATTR_FUEL_CAPACITY = "fuelCapacity"
DOGMA_ATTR_FUEL_EFFICIENCY = "fuelEfficiency"
DOGMA_ATTR_HEAT_CAPACITY = "heatCapacity"
DOGMA_ATTR_CAN_FIT_SHIP_GROUP_01 = "canFitShipGroup01"
DOGMA_ATTR_CAN_FIT_SHIP_GROUP_02 = "canFitShipGroup02"
DOGMA_ATTR_CAN_FIT_SHIP_GROUP_03 = "canFitShipGroup03"


def load_dogma_attributes(root: Path) -> dict[int, dict[str, int]]:
    dogma_attributes = restool.extract_resource(root, RES_DOGMA_ATTRIBUTES, decode=True)
    type_dogma = restool.extract_resource(root, RES_TYPE_DOGMA, decode=True)
    typeID_to_attributes: dict[int, dict[str, int]] = {}
    for typeID, typeData in type_dogma.items():
        typeID_to_attributes[typeID] = {
            dogma_attributes[attr["attributeID"]]["name"]: attr["value"] for attr in typeData["dogmaAttributes"]
        }
    return typeID_to_attributes
