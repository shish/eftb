#!/usr/bin/env python3
import json

types = json.load(open('data/types.json'))
ships = {}
for type in types:
    if type['categoryName'] == "Ship":
        ships[type['name']] = {
            "mass": type['mass'],
            'tank': 0,
            'type': type['groupName'],
            'cargo': 0,
        }

print(json.dumps(ships, indent=4))
