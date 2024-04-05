from typing import Optional

import nekoton as nt

wallet_abi = nt.ContractAbi("""{
    "ABI version": 2,
    "version": "2.3",
    "header": ["pubkey", "time", "expire"],
    "functions": [{
        "name": "sendTransaction",
        "inputs": [
            {"name": "dest", "type": "address"},
            {"name": "value", "type": "uint128"},
            {"name": "bounce", "type": "bool"},
            {"name": "flags", "type": "uint8"},
            {"name": "payload", "type": "cell"}
        ],
        "outputs": []
    }],
    "events": []
}""")

send_transaction = wallet_abi.get_function("sendTransaction")
assert send_transaction is not None

wallet_code = nt.Cell.decode(
    'te6cckEBBgEA/AABFP8A9KQT9LzyyAsBAgEgAgMABNIwAubycdcBAcAA8nqDCNcY7UTQgwfXAdcLP8j4KM8WI88WyfkAA3HXAQHDAJqDB9cBURO68uBk3oBA1wGAINcBgCDXAVQWdfkQ8qj4I7vyeWa++COBBwiggQPoqFIgvLHydAIgghBM7mRsuuMPAcjL/8s/ye1UBAUAmDAC10zQ+kCDBtcBcdcBeNcB10z4AHCAEASqAhSxyMsFUAXPFlAD+gLLaSLQIc8xIddJoIQJuZgzcAHLAFjPFpcwcQHLABLM4skB+wAAPoIQFp4+EbqOEfgAApMg10qXeNcB1AL7AOjRkzLyPOI+zYS/')


class EverWallet:
    @classmethod
    def compute_address(cls, public_key: nt.PublicKey, workchain: int = 0) -> nt.Address:
        return cls.compute_state_init(public_key).compute_address(workchain)

    @staticmethod
    def compute_state_init(public_key: nt.PublicKey) -> nt.StateInit:
        data_builder = nt.CellBuilder()
        data_builder.store_public_key(public_key)
        data_builder.store_u64(0)
        return nt.StateInit(wallet_code, data_builder.build())

    def __init__(self, transport: nt.Transport, keypair: nt.KeyPair, workchain: int = 0):
        state_init = self.compute_state_init(keypair.public_key)

        self._initialized = False
        self._transport = transport
        self._keypair = keypair
        self._state_init = state_init
        self._address = state_init.compute_address(workchain)

    @property
    def address(self) -> nt.Address:
        return self._address

    async def send(self, dst: nt.Address, value: nt.Tokens, payload: nt.Cell, bounce: bool = False) -> nt.Transaction:
        state_init = await self.__get_state_init()

        signature_id = await self._transport.get_signature_id()

        external_message = send_transaction.encode_external_message(
            self._address,
            input={
                "dest": dst,
                "value": value,
                "bounce": bounce,
                "flags": 3,
                "payload": payload
            },
            public_key=self._keypair.public_key,
            state_init=state_init
        ).sign(self._keypair, signature_id)

        tx = await self._transport.send_external_message(external_message)
        if tx is None:
            raise RuntimeError("Message expired")
        return tx

    async def get_account_state(self) -> Optional[nt.AccountState]:
        return await self._transport.get_account_state(self._address)

    async def get_balance(self) -> nt.Tokens:
        state = await self.get_account_state()
        if state is None:
            return nt.Tokens(0)
        else:
            return state.balance

    async def __get_state_init(self) -> Optional[nt.StateInit]:
        if self._initialized:
            return None

        account_state = await self.get_account_state()
        if account_state is not None and account_state.status == nt.AccountStatus.Active:
            self._initialized = True
            return None
        else:
            return self._state_init
