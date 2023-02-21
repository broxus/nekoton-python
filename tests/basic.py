import asyncio
import os

from nekoton import *

dirname = os.path.dirname(__file__)

print(nekoton.check_address(""))

cell1 = nekoton.Cell()
print(cell1.repr_hash)

cell2 = nekoton.Cell()
print(cell2.repr_hash)

print(cell1 == cell2)

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
print(complex_cell.encode('base64'))

decoded = complex_cell.unpack(abi=complex_cell_abi)
print(decoded)

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
