#!/usr/bin/env python3
import json

data = json.load(open('data/solarsystems.json'))

names = [ss['name'] for ss in data]
print(json.dumps(names))
