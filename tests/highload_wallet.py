import asyncio
from nekoton import *


# Subscriptions
async def main():
    clock = Clock()

    transport = JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()
    signature_id = await transport.get_signature_id()
    assert signature_id is None

    keypair = KeyPair.generate()
    wallet = contracts.HighloadWalletV2(transport, keypair)

    print(wallet.address)
    async with transport.account_states(wallet.address) as states:
        # There is always at least one iteration with the current state
        async for state in states:
            if state is None:
                print("account not exists")
                continue

            print("account balance: ", state.balance)
            if state.balance >= Tokens(1):
                break

    print("Balance is enough")
    tx = await wallet.send(dst=wallet.address, value=Tokens("0.5"))
    await transport.trace_transaction(tx).wait()
    print("Done")


if __name__ == "__main__":
    asyncio.run(main())
