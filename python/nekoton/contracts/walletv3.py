from typing import Optional, List

from . import IGiver
import nekoton as _nt

_wallet_code = _nt.Cell.decode(
    "te6ccgEBAQEAcQAA3v8AIN0gggFMl7ohggEznLqxn3Gw7UTQ0x/THzHXC//jBOCk8mCDCNcYINMf0x/TH/gjE7vyY+1E0NMf0x/T/9FRMrryoVFEuvKiBPkBVBBV+RDyo/gAkyDXSpbTB9QC+wDo0QGkyMsfyx/L/8ntVA=="
)
_default_wallet_id = 0x4BA92D8A
_default_ttl = 60


class WalletV3(IGiver):
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
        public_key: _nt.PublicKey, wallet_id: int = _default_wallet_id
    ) -> _nt.StateInit:
        builder = _nt.CellBuilder()
        builder.store_u32(0)
        builder.store_u32(wallet_id)
        builder.store_public_key(public_key)
        return _nt.StateInit(_wallet_code, builder.build())

    @staticmethod
    def from_address(
        transport: _nt.Transport, keypair: _nt.KeyPair, address: _nt.Address
    ) -> "WalletV3":
        wallet = WalletV3(transport, keypair)
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
        if len(messages) > 4:
            raise RuntimeError("Too many messages at once")

        seqno, state_init = await self.__get_seqno_and_state_init()
        signature_id = await self._transport.get_signature_id()

        expire_at = self._transport.clock.now_sec + _default_ttl if ttl is None else ttl

        payload_builder = _nt.CellBuilder()
        payload_builder.store_u32(self._wallet_id)
        payload_builder.store_u32(expire_at)
        payload_builder.store_u32(seqno)

        for message, flags in messages:
            payload_builder.store_u8(flags)
            payload_builder.store_reference(message.build_cell())

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

    async def __get_seqno_and_state_init(self) -> tuple[int, Optional[_nt.StateInit]]:
        if self._initialized:
            return None

        account_state = await self.get_account_state()
        if (
            account_state is not None
            and account_state.status == _nt.AccountStatus.Active
        ):
            if account_state.state_init.data is None:
                raise RuntimeError("Account state does not contain data")

            data = account_state.state_init.data.as_slice()
            seqno = data.load_u32()

            # NOTE: Update wallet_id just in case
            self._wallet_id = data.load_u32()

            self._initialized = True
            return seqno, None
        else:
            return 0, self._state_init
