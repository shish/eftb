#!/usr/bin/env python3

import typing as t

import restool

RES_SPACE_COMPONENTS = "res:/staticdata/spacecomponentsbytype.fsdbinary"
RES_GROUPS = "res:/staticdata/groups.fsdbinary"
RES_TYPES = "res:/staticdata/types.fsdbinary"


class GenStructures(restool.ResToolBase):
    def main(self, args: restool.Namespace) -> None:
        types: dict[int, dict[str, t.Any]] = self.extract_resource(RES_TYPES, decode=True)

        groups: dict[int, dict[str, t.Any]] = self.extract_resource(RES_GROUPS, decode=True)
        groupID_to_groupName: dict[int, str] = {}
        for groupID, groupData in groups.items():
            groupID_to_groupName[groupID] = groupData["groupNameID"]

        structures = {}
        space_components: dict[int, dict[str, t.Any]] = self.extract_resource(RES_SPACE_COMPONENTS, decode=True)
        for _typeID, componentData in space_components.items():
            assemblyConstruction = componentData.get("assemblyConstruction", None)
            if assemblyConstruction is None:
                continue

            constructedItem = assemblyConstruction.get("constructedItem")
            inputItems = assemblyConstruction.get("inputItems", {})
            if constructedItem is None or not inputItems:
                continue

            structures[types[constructedItem]["typeNameID"]] = {
                "components": {types[item]["typeNameID"]: count for item, count in inputItems.items()},
                "group": groupID_to_groupName[types[constructedItem]["groupID"]],
            }

        self.output_struct(structures)


def main() -> None:
    GenStructures()


if __name__ == "__main__":
    main()
