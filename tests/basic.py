import asyncio
import os

from nekoton import *

dirname = os.path.dirname(__file__)

# Address
assert(not Address.validate("totally invalid address"))
assert(Address.validate("0:a921453472366b7feeec15323a96b5dcf17197c88dc0d4578dfa52900b8a33cb"))
my_addr = Address('0:a921453472366b7feeec15323a96b5dcf17197c88dc0d4578dfa52900b8a33cb')

# Cells
cell1 = Cell()
assert(len(Cell().repr_hash) == 32)
assert(Cell() == Cell())

complex_cell_abi = [
    ("first", AbiUint(32)),
    ("second", AbiBool()),
]
complex_cell = Cell.build(
    abi=complex_cell_abi,
    value={
        "first": 123,
        "second": True,
    }
)
assert(Cell.decode(complex_cell.encode('base64'), 'base64') == complex_cell)
assert(Cell.from_bytes(complex_cell.to_bytes()) == complex_cell)

decoded = complex_cell.unpack(abi=complex_cell_abi)
assert(decoded["first"] == 123)
assert(decoded["second"] == True)

# Crypto
seed = Bip39Seed.generate()
assert(seed.word_count == 24)

keypair0 = seed.derive()
assert(len(keypair0.public_key.to_bytes()) == 32)
assert(seed.derive() == seed.derive(path=Bip39Seed.path_for_account(0)))

keypair1 = seed.derive(path=Bip39Seed.path_for_account(1))
assert(keypair0 != keypair1)
assert(len(keypair1.public_key.encode('hex')) == 64)

# Abi
with open(os.path.join(dirname, 'wallet.abi.json'), 'r') as json:
    abi = ContractAbi(json.read())

assert(abi.abi_version == AbiVersion(2, 3))
assert(abi.get_function("non-existing") is None)
assert(abi.get_event("non-existing") is None)

send_transaction_func = abi.get_function("sendTransaction")
body_cell = send_transaction_func.encode_internal_input({
    "dest": my_addr,
    "value": 123,
    "bounce": False,
    "flags": 3,
    "payload": Cell(),
})
assert(body_cell != Cell())

unpacked_body = body_cell.unpack(abi=[
    ("function_id", AbiUint(32)),
    ("dest", AbiAddress()),
    ("value", AbiUint(128)),
    ("bounce", AbiBool()),
    ("flags", AbiUint(8)),
    ("payload", AbiCell()),
])
assert(unpacked_body["dest"] == my_addr)
assert(unpacked_body["value"] == 123)
assert(unpacked_body["bounce"] == False)
assert(unpacked_body["flags"] == 3)
assert(unpacked_body["payload"] == Cell())

decoded_body = send_transaction_func.decode_input(message_body=body_cell, internal=True)
assert(decoded_body["dest"] == my_addr)
assert(decoded_body["value"] == 123)
assert(decoded_body["bounce"] == False)
assert(decoded_body["flags"] == 3)
assert(decoded_body["payload"] == Cell())


# Subscriptions
async def main():
    clock = Clock()

    transport = JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()

    subscription = await transport.subscribe(my_addr)


if __name__ == "__main__":
    asyncio.run(main())
