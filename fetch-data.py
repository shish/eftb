#!/usr/bin/env python3

import json
import urllib.request
import pathlib
import typing as t
from tqdm import tqdm

def api_get(path: str, base='https://blockchain-gateway-stillness.live.tech.evefrontier.com/v2') -> t.Any:
    cache = pathlib.Path(f'data/{path}.json')
    if cache.exists():
        data = json.loads(cache.read_text())
    else:
        url = f'{base}/{path}'

        first = json.loads(urllib.request.urlopen(url).read())
        total = first["metadata"]["total"]
        limit = first["metadata"]["limit"]

        data = []
        for offset in tqdm(range(0, total, limit), desc=f'Fetching {path}'):
            paged_url = f'{url}?limit={limit}&offset={offset}'
            response = urllib.request.urlopen(paged_url).read()
            page_data = json.loads(response.decode('utf-8'))
            if not page_data["data"]:
                break
            if offset == 0:
                data = page_data["data"]
            else:
                data.extend(page_data["data"])
            offset += limit

        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(data, indent=4))
    return data


api_get('solarsystems')
api_get('types')

gates_to_gates = []
for sass in tqdm(api_get('smartassemblies')):
    if sass['type'] == 'SmartGate' and sass['state'] == "online":
        # currently there are no smartgates online, so I don't know
        # what the data for an online gate looks like...
        print(sass)
        import sys; sys.exit(1)
        #try:
        #    gate = api_get(f'smartassemblies/{sass["id"]}')
        #    if gate['gateLink']['isLinked']:
        #        gates_to_gates.append({
        #            'id': gate['id'],
        #            'itemId': gate['itemId'],
        #            'name': gate['name'],
        #            # 'from': gate['solarSystem']['solarSystemId'],  # refers to phase-V SolarSystemId
        #            'from': gate['solarSystemId'],  # refers to alpha SolarSystemId
        #            # 'to': dest['solarSystem']['solarSystemId'],  # refers to phase-V SolarSystemId
        #            'to': gate['gateLink']['destinationGate']  # refers to alpha SmartAssemblyId
        #        })
        #except Exception as e:
        #    print(f'Error fetching gate {sass["id"][:10]}: {e}')

# Now that we've loaded all the gates, update `gate.to` to be a
# SolarSystemId instead of a GateId
gate_id_to_solar_system_id = {gate['id']: gate['from'] for gate in gates_to_gates}
gates_to_solar_systems = []
for gate in gates_to_gates:
    ssid = gate_id_to_solar_system_id.get(gate['to'])
    if ssid:
        gates_to_solar_systems.append(gate | {'to': ssid})
    else:
        print(f'Gate {gate["id"][:10]} has invalid destination ({gate["to"][:10]})')
# filter out some invalid test-gates
#gates = [g for g in gates if g['to'] != 0]
with open('data/smartgates.json', 'w') as f:
    json.dump(gates_to_solar_systems, f, indent=4)
