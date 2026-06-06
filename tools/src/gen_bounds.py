#!/usr/bin/env python3

import restool

RES_STARMAP = "res:/staticdata/starmapcache.pickle"


class GenBounds(restool.ResToolBase):
    def tool_main(self, args: restool.Namespace) -> None:
        data = self.extract_resource(RES_STARMAP, decode=True)

        min_x = 0
        max_x = 0
        min_y = 0
        max_y = 0
        min_z = 0
        max_z = 0

        for star in data["solarSystems"].values():
            x, y, z = star["center"]
            min_x = min(min_x, int(x))
            max_x = max(max_x, int(x))
            min_y = min(min_y, int(y))
            max_y = max(max_y, int(y))
            min_z = min(min_z, int(z))
            max_z = max(max_z, int(z))

        bounds = {"x": [min_x, max_x], "y": [min_y, max_y], "z": [min_z, max_z]}

        self.output_struct(bounds)


def main() -> None:
    GenBounds().main()


if __name__ == "__main__":
    main()
