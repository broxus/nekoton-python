import asyncio
import os

from nekoton import *

dirname = os.path.dirname(__file__)

# Cells
assert(not Address.validate("totally invalid address"))
assert(Address.validate("0:a921453472366b7feeec15323a96b5dcf17197c88dc0d4578dfa52900b8a33cb"))

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

# Subscriptions
async def main():
    clock = Clock()

    transport = JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()

    my_addr = Address('0:a921453472366b7feeec15323a96b5dcf17197c88dc0d4578dfa52900b8a33cb')
    subscription = await transport.subscribe(my_addr)
    print(subscription)

    with open(os.path.join(dirname, 'wallet.abi.json'), 'r') as json:
        abi = ContractAbi(json.read())

    print(abi.abi_version)
    send_transaction_func = abi.get_function("sendTransaction")
    body_cell = send_transaction_func.encode_internal_input({
        "dest": my_addr,
        "value": 123,
        "bounce": False,
        "flags": 3,
        "payload": Cell(),
    })
    print(body_cell)

    decoded_body = send_transaction_func.decode_input(message_body=body_cell, internal=True)
    print(decoded_body)


if __name__ == "__main__":
    asyncio.run(main())
