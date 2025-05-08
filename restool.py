#!/usr/bin/env python3

# Using https://github.com/frontier-reapers/frontier-static-data
# as a reference for the file formats

import pickle
import argparse
import os.path
import sys
import csv
import typing as t

import jsonpickle


def list_resources(root) -> t.Dict[str, str]:
    indexFiles = [
        # '1d/1d34143a37d4b739_553cd67a2f32c66f69a51f6eae071b9a',
        '1d/1d34143a37d4b739_2010dea88d4c9c47074c073c7d01deea',
        '1d/1d34143a37d4b739_388b20b64620b92a6befcbbe89571575',
        # '56/5610a6eb8b5a4975_1a78826bc07a6e1c4814141ad484df52',
        '56/5610a6eb8b5a4975_59756139412b3a5e548f78e15b27686e',
        '56/5610a6eb8b5a4975_f44159b8bdd132e8e4d6d5bc89dbe5ad',
    ]
    resources = {}
    for index in indexFiles:
        indexPath = os.path.join(root, index)
        if (os.path.isfile(indexPath)):
            with open(indexPath, 'r', newline='') as csvfile:
                reader = csv.reader(csvfile, delimiter=',', quotechar='"')
                for row in reader:
                    resources[row[0]] = os.path.join(root, row[1])
    return resources


if __name__ == "__main__":
    parser = argparse.ArgumentParser(sys.argv[0])
    parser.add_argument('--root', help='the root directory containing ResFiles.', default="./frontier/ResFiles")
    subparsers = parser.add_subparsers(dest='cmd')

    list_parser = subparsers.add_parser('list')

    ex_parser = subparsers.add_parser('extract')
    ex_parser.add_argument('resource', help='the relative name of the resource file.')
    ex_parser.add_argument('--unpickle', '-u', action='store_true', default=False, help='unpickle the resource file.',)
    ex_parser.add_argument('--output', '-o', help='file to output to', default=None)

    args = parser.parse_args()

    if args.cmd == "list":
        for file in list_resources(args.root).keys():
            print(file)

    if args.cmd == "extract":
        files = list_resources(args.root)
        data = open(files[args.resource], 'rb').read()

        if args.unpickle:
            jsonpickle.set_encoder_options('json', sort_keys=True, indent=4)
            struct = pickle.loads(data)
            data = (jsonpickle.encode(struct) + "\n").encode('utf-8')

        if args.output is None:
            args.output = os.path.basename(args.resource)
        if args.output == "-":
            print(data.decode('utf-8'))
        else:
            with open(args.output, 'wb') as output_file:
                output_file.write(data)
