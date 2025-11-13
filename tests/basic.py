import asyncio
import logging
import os

import nekoton as nt

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

dirname = os.path.dirname(__file__)

# Address
assert not nt.Address.validate("totally invalid address")
assert nt.Address.validate(
    "0:d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf"
)
my_addr = nt.Address(
    "-1:d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf"
)
assert my_addr.workchain == -1 and len(my_addr.account) == 32

assert my_addr.to_base64() == "Uf_YTpaf6wJIGTM4LEVE6f8kovNZhH-IlrqobFAcPRsAzwXV"

my_addr.workchain = 0
assert my_addr.workchain == 0
assert my_addr.__hash__() == my_addr.__hash__()

address_dict = {my_addr: 123}
assert address_dict[my_addr] == 123

# CellBuilder
builder = nt.CellBuilder()
builder.store_zeros(10)
builder.store_ones(6)
builder.store_reference(nt.Cell())
builder.store_u16(123)
builder.store_bytes(
    bytes.fromhex("d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf")
)
print(builder.build().encode())

# CellSlice
cs = builder.build().as_slice()
assert cs.load_u16() == 63
assert cs.load_u16() == 123
assert cs.load_reference() == nt.Cell()
assert cs.load_bytes(32) == bytes.fromhex(
    "d84e969feb02481933382c4544e9ff24a2f359847f8896baa86c501c3d1b00cf"
)
assert cs.is_empty()

# Cells
cell1 = nt.Cell()
assert len(nt.Cell().repr_hash) == 32
assert nt.Cell() == nt.Cell()

complex_cell_abi = [
    ("first", nt.AbiUint(32)),
    ("second", nt.AbiBool()),
]
complex_cell = nt.Cell.build(
    abi=complex_cell_abi,
    value={
        "first": 123,
        "second": True,
    },
)
assert nt.Cell.decode(complex_cell.encode("base64"), "base64") == complex_cell
assert nt.Cell.from_bytes(complex_cell.to_bytes()) == complex_cell

decoded = complex_cell.unpack(abi=complex_cell_abi)
assert decoded["first"] == 123
assert decoded["second"] is True

# Crypto
some_pubkey = nt.PublicKey(
    "7b671b6bfd43e306d4accb46113a871e66b30cc587a57635766a2f360ee831c6"
)
print(some_pubkey)
assert some_pubkey == nt.PublicKey.from_int(
    55816654881951532897500201008042388765613920635159317416907795053873153454534
)
assert (
    some_pubkey.to_int()
    == 55816654881951532897500201008042388765613920635159317416907795053873153454534
)

assert nt.PublicKey(
    "00005a641f7deda1442badd9ed761dd4e948580c7d7f36b3f858ab26b1af6fa1"
) == nt.PublicKey.from_int(
    623856482362781547816624847020167340480982121355398289209492515375247265
)
assert (
    nt.PublicKey(
        "00005a641f7deda1442badd9ed761dd4e948580c7d7f36b3f858ab26b1af6fa1"
    ).to_int()
    == 623856482362781547816624847020167340480982121355398289209492515375247265
)

cell_with_pubkey_abi: list[tuple[str, nt.AbiParam]] = [
    ("pubkey", nt.AbiUint(256)),
    ("not_pubkey", nt.AbiUint(256)),
]
cell_with_pubkey = nt.Cell.build(
    abi=cell_with_pubkey_abi,
    value={
        "pubkey": some_pubkey,
        "not_pubkey": 123123,
    },
)

decoded_cell_with_pubkey = cell_with_pubkey.unpack(cell_with_pubkey_abi)
assert nt.PublicKey.from_int(decoded_cell_with_pubkey["pubkey"]) == some_pubkey
assert decoded_cell_with_pubkey["not_pubkey"] == 123123

seed = nt.Bip39Seed.generate()
assert seed.word_count == 12
print(seed)

keypair0 = seed.derive()
assert len(keypair0.public_key.to_bytes()) == 32
assert seed.derive() == seed.derive(path=nt.Bip39Seed.path_for_account(0))

keypair1 = seed.derive(path=nt.Bip39Seed.path_for_account(1))
assert keypair0 != keypair1
assert len(keypair1.public_key.encode("hex")) == 64

# Abi
with open(os.path.join(dirname, "wallet.abi.json"), "r") as json:
    abi = nt.ContractAbi(json.read())

assert abi.abi_version == nt.AbiVersion(2, 3)
assert abi.get_function("non-existing") is None
assert abi.get_event("non-existing") is None

send_transaction_func = abi.function("sendTransaction")
assert send_transaction_func is not None

send_transaction_input = {
    "dest": my_addr,
    "value": nt.Tokens("1.5"),
    "bounce": False,
    "flags": 3,
    "payload": nt.Cell(),
}
body_cell = send_transaction_func.encode_internal_input(send_transaction_input)
print(body_cell)
assert body_cell != nt.Cell()

internal_msg = send_transaction_func.encode_internal_message(
    input=send_transaction_input,
    value=nt.Tokens("1.5"),
    bounce=False,
    dst=my_addr,
)
print(internal_msg)
assert internal_msg.state_init is None
assert internal_msg.body == body_cell
assert isinstance(internal_msg.header, nt.InternalMessageHeader)

unpacked_body = body_cell.unpack(
    abi=[
        ("function_id", nt.AbiUint(32)),
        ("dest", nt.AbiAddress()),
        ("value", nt.AbiUint(128)),
        ("bounce", nt.AbiBool()),
        ("flags", nt.AbiUint(8)),
        ("payload", nt.AbiCell()),
    ]
)
decoded_body = send_transaction_func.decode_input(message_body=body_cell, internal=True)

for field, item in send_transaction_input.items():
    if isinstance(item, nt.Tokens):
        assert nt.Tokens.from_nano(unpacked_body[field]) == item
        assert nt.Tokens.from_nano(decoded_body[field]) == item
    else:
        assert unpacked_body[field] == item
        assert decoded_body[field] == item

unsigned_body = send_transaction_func.encode_external_input(
    send_transaction_input, public_key=None, address=my_addr
)
assert unsigned_body.sign(keypair0, context=None) == unsigned_body.with_signature(
    keypair0.sign_raw(unsigned_body.hash, context=None)
)

unsigned_message = send_transaction_func.encode_external_message(
    my_addr, send_transaction_input, public_key=None
)
print(unsigned_message.without_signature())
external_msg, _ = unsigned_message.without_signature().split()
assert len(external_msg.hash) == 32
assert isinstance(external_msg.header, nt.ExternalInMessageHeader)

depool_abi = nt.ContractAbi.from_file(os.path.join(dirname, "depool.abi.json"))

tokens = nt.Tokens("10.123456789")
assert (2 * tokens / 2) == tokens

token_wallet_abi = nt.ContractAbi.from_file(
    os.path.join(dirname, "token_wallet.abi.json")
)

base64_string = "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"
tree = nt.TransactionTree.decode(base64_string)
assert tree.root is not None
assert tree.root.out_msgs_len == len(tree.children)
for tx in tree:
    assert tx is not None

# Asm
code = nt.Asm.compile("""
SETCP 0
DICTPUSHCONST 19, [
    0 => {
        DUP
        CALLDICT 22
        INC
    }
    22 => {
        MUL
    }
]
DICTIGETJMPZ
THROWARG 11
""")
assert code == nt.Cell.decode(
    "te6ccgEBBAEAHgABFP8A9KQT9LzyyAsBAgLOAwIABaNUQAAJ0IPAWpI="
)


# Subscriptions
async def main():
    _clock = nt.Clock()

    transport = nt.JrpcTransport(endpoint="https://jrpc.everwallet.net")
    await transport.check_connection()
    signature_id = await transport.get_signature_id()
    assert signature_id is None

    config = await transport.get_blockchain_config()
    assert config.contains_param(0)
    assert config.config_address == nt.Address(
        "-1:5555555555555555555555555555555555555555555555555555555555555555"
    )
    assert config.elector_address == nt.Address(
        "-1:3333333333333333333333333333333333333333333333333333333333333333"
    )

    account = await transport.get_account_state(my_addr)
    print(account)
    assert account is not None
    assert account.status == nt.AccountStatus.Active
    assert not account.balance.is_zero
    assert account.state_init is not None
    assert account.state_init.code is not None

    token_wallet_addr = nt.Address(
        "0:c7c7abe480e6ad15631e4353ca5dd395ee91b56c50e71cfc67780b928b138974"
    )
    token_wallet_account = await transport.get_account_state(token_wallet_addr)
    assert token_wallet_account is not None
    fields = token_wallet_abi.decode_fields(token_wallet_account)
    assert fields["root_"] == nt.Address(
        "0:a519f99bb5d6d51ef958ed24d337ad75a1c770885dcd42d51d6663f9fcdacfb2"
    )

    executor = nt.TransactionExecutor(config, check_signature=False)
    tx, new_state = executor.execute(unsigned_message.with_fake_signature(), account)
    assert not tx.aborted
    assert tx.compute_phase is not None
    assert tx.compute_phase.exit_code == 0
    assert new_state is not None

    depool_addr = nt.Address(
        "0:c9b7e458134c655123878fe7980c7118adb314fbbec32e3b7c155fea90f87a97"
    )
    depool_state = await transport.get_account_state(depool_addr)
    assert depool_state is not None

    depool_info = depool_abi.function("getDePoolInfo").call(depool_state, input={})
    assert depool_info.exit_code == 0
    assert depool_info.output is not None

    stake_accept_tx = await transport.get_transaction(
        bytes.fromhex(
            "60a311aa3e3c1f30deb3010bb09d0079713ab1b0af07f5fd2ca87f5b282912a4"
        )
    )
    assert stake_accept_tx is not None

    print(stake_accept_tx)
    on_stake_accept_func = depool_abi.function("onStakeAccept")
    parsed_stake_accept = on_stake_accept_func.decode_transaction(stake_accept_tx)
    assert parsed_stake_accept.input["queryId"] == 93
    assert parsed_stake_accept.output == {}

    await transport.trace_transaction(stake_accept_tx).wait()

    full_parsed_stake_accept = depool_abi.decode_transaction(stake_accept_tx)
    assert full_parsed_stake_accept is not None
    assert full_parsed_stake_accept.function == on_stake_accept_func
    assert len(full_parsed_stake_accept.events) == 1
    assert full_parsed_stake_accept.events[0][0] == depool_abi.event(
        "RoundStakeIsAccepted"
    )

    code_hash = bytes.fromhex(
        "7d0996943406f7d62a4ff291b1228bf06ebd3e048b58436c5b70fb77ff8b4bf2"
    )
    addresses = await transport.get_accounts_by_code_hash(code_hash, limit=10)
    assert len(addresses) == 10

    transactions = await transport.get_transactions(my_addr, limit=10)
    assert len(transactions) == 10
    for tx in transactions:
        assert len(tx.hash) == 32
        assert tx.has_in_msg
        assert tx.type == nt.TransactionType.Ordinary

    latest_tx = transactions[0]
    fetched_tx = await transport.get_transaction(latest_tx.hash)
    assert latest_tx == fetched_tx
    assert latest_tx.in_msg_hash is not None
    fetched_next_tx = await transport.get_dst_transaction(latest_tx.in_msg_hash)
    assert fetched_next_tx == fetched_tx

    async with transport.account_states(my_addr) as states:
        # There is always at least one iteration with the current state
        async for state in states:
            if state is not None:
                print(my_addr, state.balance)
            else:
                print(my_addr, "account not exists")

            break  # for tests

    async with transport.account_transactions(config.elector_address) as batches:
        async for batch, batch_info in batches:
            print(batch)
            assert len(batch) > 0
            break

    deep_tx_hash = bytes.fromhex(
        "a82e6603dec13499dd43486203b3304122ae89c13a80b189eef8976834cba413"
    )
    deep_tx_nodes = 0
    async for transaction in transport.trace_transaction(deep_tx_hash, yield_root=True):
        deep_tx_nodes += 1
        print(transaction)
    assert deep_tx_nodes == 12


if __name__ == "__main__":
    asyncio.run(main())
