from pathlib import Path

import restool


RES_TYPE_DOGMA = "res:/staticdata/typedogma.fsdbinary"
DOGMA_ATTR_FUEL_CAPACITY = 5633
DOGMA_ATTR_FUEL_EFFICIENCY = 5607
DOGMA_ATTR_HEAT_CAPACITY = 5762


def load_dogma_attributes(root: Path) -> dict[int, dict[int, int]]:
    type_dogma = restool.extract_resource(root, RES_TYPE_DOGMA, decode=True)
    typeID_to_attributes: dict[int, dict[int, int]] = {}
    for typeID, typeData in type_dogma.items():
        typeID_to_attributes[typeID] = {attr["attributeID"]: attr["value"] for attr in typeData["dogmaAttributes"]}
    return typeID_to_attributes
