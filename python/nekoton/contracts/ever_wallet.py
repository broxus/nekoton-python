from typing import Optional, List

from . import IGiver
import nekoton as _nt


_wallet_abi = _nt.ContractAbi("""{
  "ABI version": 2,
  "version": "2.3",
  "header": ["pubkey", "time", "expire"],
  "functions": [
    {
      "name": "sendTransaction",
      "inputs": [
        { "name": "dest", "type": "address" },
        { "name": "value", "type": "uint128" },
        { "name": "bounce", "type": "bool" },
        { "name": "flags", "type": "uint8" },
        { "name": "payload", "type": "cell" }
      ],
      "outputs": []
    },
    {
      "name": "sendTransactionRaw0",
      "inputs": [],
      "outputs": [],
      "id": "0x169e3e11"
    },
    {
      "name": "sendTransactionRaw1",
      "inputs": [
        { "name": "flags0", "type": "uint8" },
        { "name": "message0", "type": "cell" }
      ],
      "outputs": [],
      "id": "0x169e3e11"
    },
    {
      "name": "sendTransactionRaw2",
      "inputs": [
        { "name": "flags0", "type": "uint8" },
        { "name": "message0", "type": "cell" },
        { "name": "flags1", "type": "uint8" },
        { "name": "message1", "type": "cell" }
      ],
      "outputs": [],
      "id": "0x169e3e11"
    },
    {
      "name": "sendTransactionRaw3",
      "inputs": [
        { "name": "flags0", "type": "uint8" },
        { "name": "message0", "type": "cell" },
        { "name": "flags1", "type": "uint8" },
        { "name": "message1", "type": "cell" },
        { "name": "flags2", "type": "uint8" },
        { "name": "message2", "type": "cell" }
      ],
      "outputs": [],
      "id": "0x169e3e11"
    },
    {
      "name": "sendTransactionRaw4",
      "inputs": [
        { "name": "flags0", "type": "uint8" },
        { "name": "message0", "type": "cell" },
        { "name": "flags1", "type": "uint8" },
        { "name": "message1", "type": "cell" },
        { "name": "flags2", "type": "uint8" },
        { "name": "message2", "type": "cell" },
        { "name": "flags3", "type": "uint8" },
        { "name": "message3", "type": "cell" }
      ],
      "outputs": [],
      "id": "0x169e3e11"
    }
  ],
  "events": []
}
""")

_send_transaction = _wallet_abi.get_function("sendTransaction")
assert _send_transaction is not None

_send_transaction_raw = list()
for i in range(4):
    abi = _wallet_abi.get_function(f"sendTransactionRaw{i}")
    assert abi is not None
    _send_transaction_raw.append(abi)

_wallet_code = _nt.Cell.decode(
    "te6cckEBBgEA/AABFP8A9KQT9LzyyAsBAgEgAgMABNIwAubycdcBAcAA8nqDCNcY7UTQgwfXAdcLP8j4KM8WI88WyfkAA3HXAQHDAJqDB9cBURO68uBk3oBA1wGAINcBgCDXAVQWdfkQ8qj4I7vyeWa++COBBwiggQPoqFIgvLHydAIgghBM7mRsuuMPAcjL/8s/ye1UBAUAmDAC10zQ+kCDBtcBcdcBeNcB10z4AHCAEASqAhSxyMsFUAXPFlAD+gLLaSLQIc8xIddJoIQJuZgzcAHLAFjPFpcwcQHLABLM4skB+wAAPoIQFp4+EbqOEfgAApMg10qXeNcB1AL7AOjRkzLyPOI+zYS/"
)


class EverWallet(IGiver):
    @classmethod
    def compute_address(
        cls, public_key: _nt.PublicKey, workchain: int = 0
    ) -> _nt.Address:
        return cls.compute_state_init(public_key).compute_address(workchain)

    @staticmethod
    def compute_state_init(public_key: _nt.PublicKey) -> _nt.StateInit:
        builder = _nt.CellBuilder()
        builder.store_public_key(public_key)
        builder.store_u64(0)
        return _nt.StateInit(_wallet_code, builder.build())

    @staticmethod
    def from_address(
        transport: _nt.Transport, keypair: _nt.KeyPair, address: _nt.Address
    ) -> "EverWallet":
        wallet = EverWallet(transport, keypair)
        wallet._initialized = True
        wallet._address = address
        return wallet

    def __init__(
        self, transport: _nt.Transport, keypair: _nt.KeyPair, workchain: int = 0
    ):
        state_init = self.compute_state_init(keypair.public_key)

        self._initialized = False
        self._transport = transport
        self._keypair = keypair
        self._state_init = state_init
        self._address = state_init.compute_address(workchain)

    @property
    def address(self) -> _nt.Address:
        return self._address

    async def give(self, target: _nt.Address, amount: _nt.Tokens):
        # Send external message
        tx = await self.send(dst=target, value=amount, payload=_nt.Cell(), bounce=False)

        # Wait until all transactions are produced
        await self._transport.trace_transaction(tx).wait()

    async def send(
        self,
        dst: _nt.Address,
        value: _nt.Tokens,
        payload: _nt.Cell,
        bounce: bool = False,
    ) -> _nt.Transaction:
        state_init = await self.__get_state_init()

        signature_id = await self._transport.get_signature_id()

        external_message = _send_transaction.encode_external_message(
            self._address,
            input={
                "dest": dst,
                "value": value,
                "bounce": bounce,
                "flags": 3,
                "payload": payload,
            },
            public_key=self._keypair.public_key,
            state_init=state_init,
        ).sign(self._keypair, signature_id)

        tx = await self._transport.send_external_message(external_message)
        if tx is None:
            raise RuntimeError("Message expired")
        return tx

    async def send_raw(
        self,
        messages: List[tuple[_nt.Message, int]],
    ) -> _nt.Transaction:
        if len(messages) > 4:
            raise RuntimeError("Too many messages at once")

        state_init = await self.__get_state_init()
        signature_id = await self._transport.get_signature_id()

        abi = _send_transaction_raw[len(messages)]
        input = dict()
        for i, (message, flags) in enumerate(messages):
            input[f"flags{i}"] = flags
            input[f"message{i}"] = message.build_cell()

        external_message = abi.encode_external_message(
            self._address,
            input,
            public_key=self._keypair.public_key,
            state_init=state_init,
        ).sign(self._keypair, signature_id)

        tx = await self._transport.send_external_message(external_message)
        if tx is None:
            raise RuntimeError("Message expired")
        return tx

    async def get_account_state(self) -> Optional[_nt.AccountState]:
        return await self._transport.get_account_state(self._address)

    async def get_balance(self) -> _nt.Tokens:
        state = await self.get_account_state()
        if state is None:
            return _nt.Tokens(0)
        else:
            return state.balance

    async def __get_state_init(self) -> Optional[_nt.StateInit]:
        if self._initialized:
            return None

        account_state = await self.get_account_state()
        if (
            account_state is not None
            and account_state.status == _nt.AccountStatus.Active
        ):
            self._initialized = True
            return None
        else:
            return self._state_init
