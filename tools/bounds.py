#!/usr/bin/env python3
import json

star_data = json.load(open('data/solarsystems.json'))

min_x = 0
max_x = 0
min_y = 0
max_y = 0
min_z = 0
max_z = 0

for data in star_data:
    loc = data['location']
    min_x = min(min_x, loc['x'])
    max_x = max(max_x, loc['x'])
    min_y = min(min_y, loc['y'])
    max_y = max(max_y, loc['y'])
    min_z = min(min_z, loc['z'])
    max_z = max(max_z, loc['z'])

print("x:", min_x, "/", max_x)
print("y:", min_y, "/", max_y)
print("z:", min_z, "/", max_z)
