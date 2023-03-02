import asyncio
import logging
from nekoton import *

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)


async def main():
    transport = GqlTransport(endpoints=["mainnet.evercloud.dev/89a3b8f46a484f2ea3bdd364ddaee3a3"])
    await transport.check_connection()

    strange_addr = Address("-1:04f64c6afbff3dd10d8ba6707790ac9670d540f37a9448b0337baa6a5a92acac")

    transactions = await transport.query_transactions([
        gql.tx.AccountAddr() == strange_addr,
        gql.tx.TrType() == TransactionType.Tock
    ], order_by=[
        gql.tx.Lt().desc(),
    ])
    print(transactions)

    messages = await transport.query_messages(gql.or_([
        gql.msg.Src() == strange_addr,
        gql.msg.Dst() == strange_addr,
    ]), order_by=[
        gql.msg.CreatedLt().desc(),
    ])
    print(messages)


asyncio.run(main())
