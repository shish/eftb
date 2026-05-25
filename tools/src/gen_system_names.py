#!/usr/bin/env python3

import restool

RES_STARMAP = "res:/staticdata/starmapcache.pickle"
RES_L10N_MAIN = "res:/localizationfsd/localization_fsd_main.pickle"
RES_L10N_ENUS = "res:/localizationfsd/localization_fsd_en-us.pickle"


def main() -> None:
    parser = restool.ArgumentParser()
    args = parser.parse_args()

    ssid_to_name: dict[int, str] = {}
    l10n_main = restool.extract_resource(args.root, RES_L10N_MAIN, decode=True)["labels"].values()
    messages = restool.extract_resource(args.root, RES_L10N_ENUS, decode=True)[1]
    for n in l10n_main:
        if n["FullPath"] == "Map/SolarSystems" and n["label"].startswith("solar_system_"):
            ssid = n["label"].split("solar_system_")[1]
            ssid_to_name[int(ssid)] = messages[n["messageID"]][0]

    system_names: list[str] = []
    stars = restool.extract_resource(args.root, RES_STARMAP, decode=True)["solarSystems"].keys()
    for ssid in stars:
        system_names.append(ssid_to_name[ssid])
    system_names.sort()

    parser.output_struct(system_names)


if __name__ == "__main__":
    main()
