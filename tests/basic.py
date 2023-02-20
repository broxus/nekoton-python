import asyncio
from nekoton import *

print(nekoton.check_address(""))

cell1 = nekoton.Cell()
print(cell1.repr_hash)

cell2 = nekoton.Cell()
print(cell2.repr_hash)

print(cell1 == cell2)


async def main():
    clock = Clock()

    transport = JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()

    my_addr = Address('0:a921453472366b7feeec15323a96b5dcf17197c88dc0d4578dfa52900b8a33cb')
    subscription = await transport.subscribe(my_addr)
    print(subscription)

    with open('wallet.abi.json', 'r') as json:
        abi = ContractAbi(json.read())

    print(abi.abi_version)
    send_transaction_func = abi.get_function("sendTransaction")
    body = send_transaction_func.encode_internal_input({
        "dest": my_addr,
        "value": 123,
        "bounce": False,
        "flags": 3,
        "payload": Cell(),
    })
    print(body)


if __name__ == "__main__":
    asyncio.run(main())
