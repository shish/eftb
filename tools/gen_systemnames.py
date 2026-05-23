#!/usr/bin/env python3

import json
import logging
from pathlib import Path

import restool

RES_STARMAP = "res:/staticdata/starmapcache.pickle"
RES_L10N_MAIN = "res:/localizationfsd/localization_fsd_main.pickle"
RES_L10N_ENUS = "res:/localizationfsd/localization_fsd_en-us.pickle"

if __name__ == "__main__":
    parser = restool.base_parser(description="Generate solar system names list.")
    parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
    args = parser.parse_args()
    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="%(asctime)s %(message)s",
    )

    stars = restool.extract_resource(args.root, RES_STARMAP, decode=True)["solarSystems"].keys()
    l10n_main = restool.extract_resource(args.root, RES_L10N_MAIN, decode=True)["labels"].values()
    messages = restool.extract_resource(args.root, RES_L10N_ENUS, decode=True)[1]

    ssid_to_name: dict[int, str] = {}
    for n in l10n_main:
        if n["FullPath"] == "Map/SolarSystems" and n["label"].startswith("solar_system_"):
            ssid = n["label"].split("solar_system_")[1]
            ssid_to_name[int(ssid)] = messages[n["messageID"]][0]

    systemnames: list[str] = []
    for ssid in stars:
        systemnames.append(ssid_to_name[ssid])
    systemnames.sort()

    if args.output:
        args.output.write_text(json.dumps(systemnames, indent=2))
    else:
        print(json.dumps(systemnames, indent=2))
