#!/usr/bin/env python3

import json
import urllib.request
from pathlib import Path
import typing as t
import logging
import argparse

try:
    from tqdm import tqdm
except ImportError:
    def tqdm(iterable, *args, **kwargs):
        """A no-op tqdm function for environments where tqdm is not installed."""
        return iterable

log = logging.getLogger(__name__)


def api_get(path: str, base: str) -> t.Any:
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


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Fetch data from EVE Frontier API")
    parser.add_argument('--base-url', default='https://blockchain-gateway-stillness.live.tech.evefrontier.com/v2', help='Base URL for the API')
    parser.add_argument('--debug', action='store_true', help="Enable debug logging")
    parser.add_argument("--output", "-o", type=Path, default=None, help="Where to write data")
    parser.add_argument("path")
    args = parser.parse_args()

    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO, format='%(asctime)s %(message)s')

    data = api_get(args.path, base=args.base_url)
    if args.output:
        args.output.write_text(json.dumps(data, indent=4))
    else:
        print(json.dumps(data, indent=4))
