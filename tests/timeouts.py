import asyncio
import logging

from nekoton import *

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)


async def main():
    clock = Clock()
    transport = ProtoTransport(endpoint="https://jrpc.everwallet.net", clock=clock)
    await transport.check_connection()

    address = Address(
        "0:0000000000000000000000000000000000000000000000000000000000000000"
    )

    external_message = SignedExternalMessage(
        address,
        clock.now_sec + 60,
        body=None,
        state_init=None,
    )
    tx = await transport.send_external_message(external_message)
    print(tx)
    await transport.trace_transaction(tx).wait()


if __name__ == "__main__":
    asyncio.run(main())
