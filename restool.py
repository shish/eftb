#!/usr/bin/env python3

# Based on https://github.com/frontier-reapers/frontier-static-data

import pickle
import argparse
import os.path
import sys
import csv
import typing as t

import jsonpickle


class ResFile:
    def __init__(self, resName: str, res: str, index: int, filepath: str):
        self.resName = resName
        self.res = res
        self.index = index
        self.filepath = filepath

    def __str__(self):
        return self.resName

class clr:
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    ENDC = '\033[0m'


def get_indexfiles():
    indexFiles = list()
    #indexFiles.append('1d/1d34143a37d4b739_553cd67a2f32c66f69a51f6eae071b9a')
    #indexFiles.append('56/5610a6eb8b5a4975_1a78826bc07a6e1c4814141ad484df52')

    indexFiles.append('1d/1d34143a37d4b739_2010dea88d4c9c47074c073c7d01deea')
    indexFiles.append('1d/1d34143a37d4b739_388b20b64620b92a6befcbbe89571575')
    indexFiles.append('56/5610a6eb8b5a4975_59756139412b3a5e548f78e15b27686e')
    indexFiles.append('56/5610a6eb8b5a4975_f44159b8bdd132e8e4d6d5bc89dbe5ad')

    return indexFiles

def find_resfile(root, file) -> ResFile:
    for resfile in list_resfiles(root):
        if (resfile.resName == file):
            return resfile
    raise Exception(f"File {file} not found")

def list_resfiles(root) -> t.Generator[ResFile]:
    indexFiles = get_indexfiles()
    for index in indexFiles:
        indexPath = os.path.join(root, index)
        if (os.path.isfile(indexPath)):
            with open(indexPath, 'r', newline='') as csvfile:
                reader = csv.reader(csvfile, delimiter=',', quotechar='"')
                for row in reader:
                    yield ResFile(row[0], row[1], index, os.path.join(root, row[1]))


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
        for file in list_resfiles(args.root):
            print(file)

    if args.cmd == "extract":
        output = find_resfile(args.root, args.resource)
        data = open(output.filepath, 'rb').read()

        if args.unpickle:
            jsonpickle.set_encoder_options('json', sort_keys=True, indent=4)
            struct = pickle.loads(data)
            data = (jsonpickle.encode(struct) + "\n").encode('utf-8')

        if args.output:
            with open(args.output, 'wb') as output_file:
                output_file.write(data)
        else:
            print(data.decode('utf-8'))
