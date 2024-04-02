import os
import argparse

from typing import Optional

from . import generator

parser = argparse.ArgumentParser(
    prog="nekoton", description="Generates a wrapper using an ABI"
)

parser.add_argument("filename")
parser.add_argument("-n", "--name", required=False)

args = parser.parse_args()

with open(args.filename) as abi_file:
    abi = abi_file.read()

    contract_name: Optional[str] = args.name
    if contract_name is None:
        contract_name = os.path.basename(args.filename).removesuffix(".abi.json")

    print(generator.generate(contract_name, abi))
