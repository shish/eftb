#!/usr/bin/env python3

import sqlite3
import json


con = sqlite3.connect("data/blockchain.db")
cur = con.cursor()

for (namespace, name, k, v) in cur.execute(
    "SELECT namespace, name, key_schema, value_schema FROM __mudStoreTables ORDER BY namespace, name"
):
    print(f"{namespace}.{name} -- {", ".join(json.loads(k)["json"].keys())} -> {", ".join(json.loads(v)["json"].keys())}")
