#!/usr/bin/env python3

import json
import urllib.request
from pathlib import Path
import typing as t
import logging
import sqlite3

try:
    from tqdm import tqdm
except ImportError:
    def tqdm(iterable, *args, **kwargs):
        """A no-op tqdm function for environments where tqdm is not installed."""
        return iterable

log = logging.getLogger(__name__)


def api_get(path: str, base='https://blockchain-gateway-stillness.live.tech.evefrontier.com/v2') -> t.Any:
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

    return data


def fetch_gates(db_path: Path) -> t.List[t.Dict[str, t.Any]]:
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    PREFIX = "0xcdb380e0cd3949caf70c45c67079f2e27a77fc47__evefrontier__"
    # smart_assembly = f'"{PREFIX}smart_assembly"'
    # smart_gate_config = f'"{PREFIX}smart_gate_config"'
    smart_gate_link = f'"{PREFIX}smart_gate_link"'
    location = f'"{PREFIX}location"'

    query = f"""
        SELECT
            smart_gate_link.source_gate_id,
            source_location.solar_system_id,
            destination_location.solar_system_id
        FROM
            {smart_gate_link} AS smart_gate_link
        JOIN
            {location} AS source_location
            ON source_location.smart_object_id = smart_gate_link.source_gate_id
        JOIN
            {location} AS destination_location
            ON destination_location.smart_object_id = smart_gate_link.destination_gate_id
        WHERE
            smart_gate_link.is_linked = 1
    """

    data = []
    for row in tqdm(cur.execute(query), desc='Fetching smart gates'):
        data.append({
            'id': row[0].decode(),
            'itemId': None,
            'name': None,
            'from': row[1].decode(),
            'to': row[2].decode()
        })
    con.close()
    return data


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, format='%(asctime)s %(message)s')

    d = Path('data')
    d.mkdir(parents=True, exist_ok=True)

    # extracted starmap has NPC gates + solar system locations
    log.info("Checking for extracted starmap")
    starmap_path = d / 'extracted-starmap.json'
    if not starmap_path.exists():
        log.error("Extracted starmap not found. Please use restool.py to extract it first.")
        exit(1)

    # solarsystems API has solar system locations + solar system names
    log.info("Fetching solarsystems data from API")
    ss_path = d / 'solarsystems.json'
    if not ss_path.exists():
        data = api_get('solarsystems')
        ss_path.write_text(json.dumps(data, indent=4))

    # types has ship mass / volume for consts.tsx, but
    # the rest of consts.tsx is manually copy-pasted
    # from show-info on market items
    log.info("Fetching types data from API")
    types_path = d / 'types.json'
    if not types_path.exists():
        data = api_get('types')
        types_path.write_text(json.dumps(data, indent=4))

    # blockchain has player smartgates
    log.info("Fetching smartgates from blockchain index")
    db_path = d / 'blockchain.db'
    if not db_path.exists():
        log.error("Blockchain database not found. Please run fetch-blockchain.sh and wait for it to sync.")
        exit(1)
    smartgates_path = d / 'smartgates.json'
    if not smartgates_path.exists() or db_path.stat().st_mtime > smartgates_path.stat().st_mtime:
        data = fetch_gates(db_path)
        smartgates_path.write_text(json.dumps(data, indent=4))
