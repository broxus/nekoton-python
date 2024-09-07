from typing import Optional, List

from . import IGiver
import nekoton as _nt

_wallet_code = _nt.Cell.decode(
    "te6ccgEBCQEA5QABFP8A9KQT9LzyyAsBAgEgBAIB6vKDCNcYINMf0z/4I6ofUyC58mPtRNDTH9M/0//0BNFTYIBA9A5voTHyYFFzuvKiB/kBVBCH+RDyowL0BNH4AH+OFiGAEPR4b6UgmALTB9QwAfsAkTLiAbPmW4MlochANIBA9EOK5jHIEssfE8s/y//0AMntVAMANCCAQPSWb6UyURCUMFMDud4gkzM2AZIyMOKzAgFICAUCASAHBgBBvl+XaiaGmPmOmf6f+Y+gJoqRBAIHoHN9CYyS2/yV3R8UABe9nOdqJoaa+Y64X/wABNAw"
)
_default_wallet_id = 0x00000000
_default_ttl = 60

_messages_abi = [
    (
        "messages",
        _nt.AbiMap(
            _nt.AbiUint(16),
            _nt.AbiTuple([("flags", _nt.AbiUint(8)), ("message", _nt.AbiCell())]),
        ),
    )
]


class HighloadWalletV2(IGiver):
    @classmethod
    def compute_address(
        cls,
        public_key: _nt.PublicKey,
        workchain: int = 0,
        wallet_id: int = _default_wallet_id,
    ) -> _nt.Address:
        return cls.compute_state_init(public_key, wallet_id).compute_address(workchain)

    @staticmethod
    def compute_state_init(
        public_key: _nt.PublicKey,
        wallet_id: int = _default_wallet_id,
        last_cleaned: int = 0,
    ) -> _nt.StateInit:
        builder = _nt.CellBuilder()
        builder.store_u32(wallet_id)
        builder.store_u64(last_cleaned)
        builder.store_public_key(public_key)
        builder.store_bit_zero()
        return _nt.StateInit(_wallet_code, builder.build())

    @staticmethod
    def from_address(
        transport: _nt.Transport, keypair: _nt.KeyPair, address: _nt.Address
    ) -> "HighloadWalletV2":
        wallet = HighloadWalletV2(transport, keypair)
        wallet._address = address
        return wallet

    def __init__(
        self,
        transport: _nt.Transport,
        keypair: _nt.KeyPair,
        workchain: int = 0,
        wallet_id: int = _default_wallet_id,
    ):
        state_init = self.compute_state_init(keypair.public_key, wallet_id)

        self._initialized = False
        self._wallet_id = wallet_id
        self._transport = transport
        self._keypair = keypair
        self._state_init = state_init
        self._address = state_init.compute_address(workchain)

    @property
    def address(self) -> _nt.Address:
        return self._address

    @property
    def wallet_id(self) -> int:
        return self._wallet_id

    async def give(self, target: _nt.Address, amount: _nt.Tokens):
        internal_message = _nt.Message(
            header=_nt.InternalMessageHeader(
                value=amount,
                dst=target,
                bounce=False,
            ),
        )

        # Send external message
        tx = await self.send_raw([(internal_message, 3)])

        # Wait until all transactions are produced
        await self._transport.trace_transaction(tx).wait()

    async def send(
        self,
        dst: _nt.Address,
        value: _nt.Tokens,
        payload: Optional[_nt.Cell] = None,
        bounce: bool = False,
        state_init: Optional[_nt.StateInit] = None,
        ttl: Optional[int] = None,
    ) -> _nt.Transaction:
        internal_message = _nt.Message(
            header=_nt.InternalMessageHeader(
                value=value,
                dst=dst,
                bounce=bounce,
            ),
            body=payload,
            state_init=state_init,
        )

        tx = await self.send_raw([(internal_message, 3)], ttl)
        return tx

    async def send_raw(
        self,
        messages: List[tuple[_nt.Message, int]],
        ttl: Optional[int] = None,
    ) -> _nt.Transaction:
        if len(messages) > 255:
            raise RuntimeError("Too many messages at once")

        state_init = await self.__get_state_init()
        signature_id = await self._transport.get_signature_id()

        expire_at = self._transport.clock.now_sec + _default_ttl if ttl is None else ttl

        messages_dict = []
        for i, (message, flags) in enumerate(messages):
            messages_dict.append((i, {"flags": flags, "message": message.build_cell()}))
        messages_dict_cell = _nt.Cell.build(
            abi=_messages_abi, value={"messages": messages_dict}
        )
        messages_dict_hash = messages_dict_cell.repr_hash

        payload_builder = _nt.CellBuilder()
        payload_builder.store_u32(self._wallet_id)
        payload_builder.store_u32(expire_at)
        payload_builder.store_raw(messages_dict_hash[28:32], 32)
        payload_builder.store_slice(messages_dict_cell.as_slice())

        hash_to_sign = payload_builder.build().repr_hash
        signature = self._keypair.sign_raw(hash_to_sign, signature_id)

        body_builder = _nt.CellBuilder()
        body_builder.store_signature(signature)
        body_builder.store_builder(payload_builder)

        body = body_builder.build()

        external_message = _nt.SignedExternalMessage(
            dst=self._address, expire_at=expire_at, body=body, state_init=state_init
        )

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
            if account_state.state_init.data is None:
                raise RuntimeError("Account state does not contain data")

            # NOTE: Update wallet_id just in case
            self._wallet_id = account_state.state_init.data.as_slice().load_u32()

            self._initialized = True
            return None
        else:
            return self._state_init
