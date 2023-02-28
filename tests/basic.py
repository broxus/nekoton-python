import asyncio
import os
import logging

from nekoton import *

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

dirname = os.path.dirname(__file__)

# Address
assert (not Address.validate("totally invalid address"))
assert (Address.validate("0:d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf"))
my_addr = Address('-1:d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf')
assert (my_addr.workchain == -1 and len(my_addr.account) == 32)
my_addr.workchain = 0
assert (my_addr.workchain == 0)

# Cells
cell1 = Cell()
assert (len(Cell().repr_hash) == 32)
assert (Cell() == Cell())

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
assert (Cell.decode(complex_cell.encode('base64'), 'base64') == complex_cell)
assert (Cell.from_bytes(complex_cell.to_bytes()) == complex_cell)

decoded = complex_cell.unpack(abi=complex_cell_abi)
assert (decoded["first"] == 123)
assert (decoded["second"] is True)

# Crypto
some_pubkey = PublicKey('7b671b6bfd43e306d4accb46113a871e66b30cc587a57635766a2f360ee831c6')
print(some_pubkey)
assert some_pubkey == PublicKey.from_int(55816654881951532897500201008042388765613920635159317416907795053873153454534)
assert (some_pubkey.to_int() == 55816654881951532897500201008042388765613920635159317416907795053873153454534)

assert PublicKey('00005a641f7deda1442badd9ed761dd4e948580c7d7f36b3f858ab26b1af6fa1') == PublicKey.from_int(
    623856482362781547816624847020167340480982121355398289209492515375247265)
assert (PublicKey('00005a641f7deda1442badd9ed761dd4e948580c7d7f36b3f858ab26b1af6fa1').to_int()
        == 623856482362781547816624847020167340480982121355398289209492515375247265)

cell_with_pubkey_abi = [
    ("pubkey", AbiUint(256)),
    ("not_pubkey", AbiUint(256)),
]
cell_with_pubkey = Cell.build(
    abi=cell_with_pubkey_abi,
    value={
        "pubkey": some_pubkey,
        "not_pubkey": 123123,
    }
)
decoded_cell_with_pubkey = cell_with_pubkey.unpack(cell_with_pubkey_abi)
assert PublicKey.from_int(decoded_cell_with_pubkey['pubkey']) == some_pubkey
assert decoded_cell_with_pubkey['not_pubkey'] == 123123

seed = Bip39Seed.generate()
assert (seed.word_count == 12)
print(seed)

keypair0 = seed.derive()
assert (len(keypair0.public_key.to_bytes()) == 32)
assert (seed.derive() == seed.derive(path=Bip39Seed.path_for_account(0)))

keypair1 = seed.derive(path=Bip39Seed.path_for_account(1))
assert (keypair0 != keypair1)
assert (len(keypair1.public_key.encode('hex')) == 64)

# Abi
with open(os.path.join(dirname, 'wallet.abi.json'), 'r') as json:
    abi = ContractAbi(json.read())

assert (abi.abi_version == AbiVersion(2, 3))
assert (abi.get_function("non-existing") is None)
assert (abi.get_event("non-existing") is None)

send_transaction_func = abi.get_function("sendTransaction")
assert (send_transaction_func is not None)

send_transaction_input = {
    "dest": my_addr,
    "value": Tokens('1.5'),
    "bounce": False,
    "flags": 3,
    "payload": Cell(),
}
body_cell = send_transaction_func.encode_internal_input(send_transaction_input)
print(body_cell)
assert (body_cell != Cell())

internal_msg = send_transaction_func.encode_internal_message(
    input=send_transaction_input,
    value=Tokens('1.5'),
    bounce=False,
    dst=my_addr,
)
print(internal_msg)
assert (internal_msg.state_init is None)
assert (internal_msg.body == body_cell)
assert (isinstance(internal_msg.header, InternalMessageHeader))

unpacked_body = body_cell.unpack(abi=[
    ("function_id", AbiUint(32)),
    ("dest", AbiAddress()),
    ("value", AbiUint(128)),
    ("bounce", AbiBool()),
    ("flags", AbiUint(8)),
    ("payload", AbiCell()),
])
decoded_body = send_transaction_func.decode_input(message_body=body_cell, internal=True)

for field, item in send_transaction_input.items():
    if isinstance(item, Tokens):
        assert (Tokens.from_nano(unpacked_body[field]) == item)
        assert (Tokens.from_nano(decoded_body[field]) == item)
    else:
        assert (unpacked_body[field] == item)
        assert (decoded_body[field] == item)

unsigned_body = send_transaction_func.encode_external_input(send_transaction_input, public_key=None, address=my_addr)
assert (unsigned_body.sign(keypair0, signature_id=None) == unsigned_body.with_signature(
    keypair0.sign(unsigned_body.hash, signature_id=None)))

unsigned_message = send_transaction_func.encode_external_message(my_addr, send_transaction_input, public_key=None)
print(unsigned_message.without_signature())
external_msg, _ = unsigned_message.without_signature().split()
assert (len(external_msg.hash) == 32)
assert (isinstance(external_msg.header, ExternalInMessageHeader))

depool_abi = ContractAbi.from_file(os.path.join(dirname, 'depool.abi.json'))

tokens = Tokens('10.123456789')
assert (2 * tokens / 2) == tokens


# Subscriptions
async def main():
    clock = Clock()

    transport = JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()
    signature_id = await transport.get_signature_id()
    assert (signature_id is None)

    config = await transport.get_blockchain_config()
    assert (config.contains_param(0))
    assert (config.config_address == Address("-1:5555555555555555555555555555555555555555555555555555555555555555"))
    assert (config.elector_address == Address("-1:3333333333333333333333333333333333333333333333333333333333333333"))

    account = await transport.get_account_state(my_addr)
    print(account)
    assert (account is not None)
    assert (account.status == AccountStatus.Active)
    assert (not account.balance.is_zero)
    assert (account.state_init.code is not None)

    executor = TransactionExecutor(config, check_signature=False)
    tx, new_state = executor.execute(unsigned_message.with_fake_signature(), account)
    assert (not tx.aborted)
    assert (tx.compute_phase.exit_code == 0)
    assert (new_state is not None)

    depool_addr = Address("0:d9cf3648c1c9436785ed628d5d83a66853eb85feb94a9cfed6239056a32cc149")
    depool_state = await transport.get_account_state(depool_addr)

    depool_info = depool_abi.get_function("getDePoolInfo").call(depool_state, input={})
    assert (depool_info.exit_code == 0)
    assert (depool_info.output is not None)

    stake_accept_tx = await transport.get_transaction(
        bytes.fromhex("60a311aa3e3c1f30deb3010bb09d0079713ab1b0af07f5fd2ca87f5b282912a4"))
    print(stake_accept_tx)
    on_stake_accept_func = depool_abi.get_function("onStakeAccept")
    parsed_stake_accept = on_stake_accept_func.decode_transaction(stake_accept_tx)
    assert (parsed_stake_accept.input['queryId'] == 93)
    assert (parsed_stake_accept.output == {})

    await transport.trace_transaction(stake_accept_tx).wait()

    full_parsed_stake_accept = depool_abi.decode_transaction(stake_accept_tx)
    assert (full_parsed_stake_accept.function == on_stake_accept_func)
    assert (len(full_parsed_stake_accept.events) == 1)
    assert (full_parsed_stake_accept.events[0][0] == depool_abi.get_event("RoundStakeIsAccepted"))

    code_hash = bytes.fromhex("7d0996943406f7d62a4ff291b1228bf06ebd3e048b58436c5b70fb77ff8b4bf2")
    addresses = await transport.get_accounts_by_code_hash(code_hash, limit=10)
    assert (len(addresses) == 10)

    transactions = await transport.get_transactions(my_addr, limit=10)
    assert (len(transactions) == 10)
    for tx in transactions:
        assert (len(tx.hash) == 32)
        assert tx.has_in_msg
        assert (tx.type == TransactionType.Ordinary)

    latest_tx = transactions[0]
    fetched_tx = await transport.get_transaction(latest_tx.hash)
    assert (latest_tx == fetched_tx)
    fetched_next_tx = await transport.get_dst_transaction(latest_tx.in_msg_hash)
    assert (fetched_next_tx == fetched_tx)

    async with transport.account_states(my_addr) as states:
        # There is always at least one iteration with the current state
        async for state in states:
            if state is not None:
                print(my_addr, state.balance)
            else:
                print(my_addr, 'account not exists')

            break  # for tests

    async with transport.account_transactions(config.elector_address) as batches:
        async for batch, batch_info in batches:
            print(batch)
            assert len(batch) > 0
            break

    deep_tx_hash = bytes.fromhex('a82e6603dec13499dd43486203b3304122ae89c13a80b189eef8976834cba413')
    deep_tx_nodes = 0
    async for transaction in transport.trace_transaction(deep_tx_hash, yield_root=True):
        deep_tx_nodes += 1
        print(transaction)
    assert deep_tx_nodes == 12


if __name__ == "__main__":
    asyncio.run(main())
