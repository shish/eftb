#!/usr/bin/env python3
import json

types = json.load(open('data/types.json'))
item = {}
for type in types:
    if (
        type['categoryName'] in ("Asteroid", "Material")
        or type['groupName'] in ("Hydrogen Fuel", "Crude Fuel")
    ):
        item[type['name']] = {
            "mass": type['mass'],
            'volume': type['volume'],
        }

print(json.dumps(item, indent=4))
