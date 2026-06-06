#!/usr/bin/env python3

import restool

RES_STARMAP = "res:/staticdata/starmapcache.pickle"
RES_L10N_MAIN = "res:/localizationfsd/localization_fsd_main.pickle"
RES_L10N_ENUS = "res:/localizationfsd/localization_fsd_en-us.pickle"


class GenStarmap(restool.ResToolBase):
    def tool_main(self, args: restool.Namespace) -> None:
        ssid_to_name: dict[int, str] = {}
        l10n_main = self.extract_resource(RES_L10N_MAIN, decode=True)["labels"].values()
        messages = self.extract_resource(RES_L10N_ENUS, decode=True)[1]
        for n in l10n_main:
            if n["FullPath"] == "Map/SolarSystems" and n["label"].startswith("solar_system_"):
                ssid = n["label"].split("solar_system_")[1]
                ssid_to_name[int(ssid)] = messages[n["messageID"]][0]

        data = self.extract_resource(RES_STARMAP, decode=True)
        solarSystems = []
        for ssid, ss in data["solarSystems"].items():
            solarSystems.append(
                {
                    "name": ssid_to_name.get(ssid),
                    "solarSystemID": ssid,
                    "center": ss["center"],
                    "regionID": ss["regionID"],
                }
            )

        jumps = []
        for jump in data["jumps"]:
            jumps.append(
                {
                    "jumpType": jump["jumpType"],
                    "fromSystemID": jump["fromSystemID"],
                    "toSystemID": jump["toSystemID"],
                }
            )

        self.output_struct(
            {
                "solarSystems": solarSystems,
                "jumps": jumps,
            }
        )


def main() -> None:
    GenStarmap().main()


if __name__ == "__main__":
    main()
