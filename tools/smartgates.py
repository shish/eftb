#!/usr/bin/env python3

import json
from pathlib import Path
import typing as t
import logging
import sqlite3
import argparse

log = logging.getLogger(__name__)


def fetch_gates(db_path: Path) -> t.List[t.Dict[str, t.Any]]:
    con = sqlite3.connect(db_path)
    con.row_factory = sqlite3.Row
    cur = con.cursor()

    PREFIX = "0xcdb380e0cd3949caf70c45c67079f2e27a77fc47__evefrontier__"
    # smart_assembly = f'"{PREFIX}smart_assembly"'
    # smart_gate_config = f'"{PREFIX}smart_gate_config"'
    smart_gate_link = f'"{PREFIX}smart_gate_link"'
    entity_record_meta = f'"{PREFIX}entity_record_meta"'
    entity_record = f'"{PREFIX}entity_record"'
    location = f'"{PREFIX}location"'

    query = f"""
        SELECT
            smart_gate_link.source_gate_id AS id,
            source_meta.name AS name,
            source_entity.item_id AS itemId,
            source_location.solar_system_id AS source,
            destination_location.solar_system_id AS destination
        FROM
            {smart_gate_link} AS smart_gate_link
        JOIN
            {location} AS source_location
            ON source_location.smart_object_id = smart_gate_link.source_gate_id
        LEFT JOIN
            {entity_record_meta} AS source_meta
            ON source_meta.smart_object_id = smart_gate_link.source_gate_id
        LEFT JOIN
            {entity_record} AS source_entity
            ON source_entity.smart_object_id = smart_gate_link.source_gate_id
        JOIN
            {location} AS destination_location
            ON destination_location.smart_object_id = smart_gate_link.destination_gate_id
        WHERE
            smart_gate_link.is_linked = 1
    """

    data = []
    for row in cur.execute(query):
        data.append({
            'id': row['id'].decode(),
            'itemId': row['itemId'].decode() if row['itemId'] else None,
            'name': row['name'].decode() if row['name'] else None,
            'from': row['source'].decode(),
            'to': row['destination'].decode()
        })
    con.close()
    return data


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Fetch smartgates from the blockchain database.")
    parser.add_argument('--debug', action='store_true', help="Enable debug logging")
    parser.add_argument("--db", type=Path, default=Path('data/blockchain.db'), help="Path to the blockchain database")
    parser.add_argument('--output', '-o', type=Path, default=None, help="Where to write data")
    args = parser.parse_args()

    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO, format='%(asctime)s %(message)s')

    if not args.db.exists():
        log.error(f"Blockchain database ({args.db}) not found")
        exit(1)

    data = fetch_gates(args.db)
    if args.output:
        args.output.write_text(json.dumps(data, indent=4))
    else:
        print(json.dumps(data, indent=4))
