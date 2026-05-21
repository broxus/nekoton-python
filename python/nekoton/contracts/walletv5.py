from typing import List, Optional

import nekoton.nekoton as _nt

from .base import IGiver

_wallet_code = _nt.Cell.decode(
    "b5ee9c7241021401000281000114ff00f4a413f4bcf2c80b01020120020d020148030402dcd020d749c120915b8f6320d70b1f2082106578746ebd21821073696e74bdb0925f03e082106578746eba8eb48020d72101d074d721fa4030fa44f828fa443058bd915be0ed44d0810141d721f4058307f40e6fa1319130e18040d721707fdb3ce03120d749810280b99130e070e2100f020120050c020120060902016e07080019adce76a2684020eb90eb85ffc00019af1df6a2684010eb90eb858fc00201480a0b0017b325fb51341c75c875c2c7e00011b262fb513435c280200019be5f0f6a2684080a0eb90fa02c0102f20e011e20d70b1f82107369676ebaf2e08a7f0f01e68ef0eda2edfb218308d722028308d723208020d721d31fd31fd31fed44d0d200d31f20d31fd3ffd70a000af90140ccf9109a28945f0adb31e1f2c087df02b35007b0f2d0845125baf2e0855036baf2e086f823bbf2d0882292f800de01a47fc8ca00cb1f01cf16c9ed542092f80fde70db3cd81003f6eda2edfb02f404216e926c218e4c0221d73930709421c700b38e2d01d72820761e436c20d749c008f2e09320d74ac002f2e09320d71d06c712c2005230b0f2d089d74cd7393001a4e86c128407bbf2e093d74ac000f2e093ed55e2d20001c000915be0ebd72c08142091709601d72c081c12e25210b1e30f20d74a111213009601fa4001fa44f828fa443058baf2e091ed44d0810141d718f405049d7fc8ca0040048307f453f2e08b8e14038307f45bf2e08c22d70a00216e01b3b0f2d090e2c85003cf1612f400c9ed54007230d72c08248e2d21f2e092d200ed44d0d2005113baf2d08f54503091319c01810140d721d70a00f2e08ee2c8ca0058cf16c9ed5493f2c08de20010935bdb31e1d74cd0b4d6c35e",
    "hex",
)
_default_wallet_id = 0x7FFFFF11
_default_ttl = 60

_op_signed_external = 0x7369676E
_op_action_send_msg = 0x0EC3C86D


class WalletV5(IGiver):
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
        builder.store_bit_one()
        builder.store_u32(0)
        builder.store_u32(wallet_id)
        builder.store_public_key(public_key)
        builder.store_bit_zero()
        return _nt.StateInit(_wallet_code, builder.build())

    @staticmethod
    def from_address(
        transport: _nt.Transport, keypair: _nt.KeyPair, address: _nt.Address
    ) -> "WalletV5":
        wallet = WalletV5(transport, keypair)
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

        tx = await self.send_raw([(internal_message, 3)])

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

        seqno, state_init = await self.__get_seqno_and_state_init()
        context = await self._transport.get_signature_context()

        expire_at = self._transport.clock.now_sec + _default_ttl if ttl is None else ttl

        payload_builder = _nt.CellBuilder()
        payload_builder.store_u32(_op_signed_external)
        payload_builder.store_u32(self._wallet_id)
        payload_builder.store_u32(expire_at)
        payload_builder.store_u32(seqno)
        self.__store_inner_request(payload_builder, messages)

        hash_to_sign = payload_builder.build().repr_hash
        signature = self._keypair.sign_raw(hash_to_sign, context)

        body_builder = _nt.CellBuilder()
        body_builder.store_builder(payload_builder)
        body_builder.store_signature(signature)

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

    @staticmethod
    def __store_inner_request(
        builder: _nt.CellBuilder, messages: List[tuple[_nt.Message, int]]
    ) -> None:
        if len(messages) > 0:
            builder.store_bit_one()
            builder.store_reference(WalletV5.__build_out_list(messages))
        else:
            builder.store_bit_zero()

        builder.store_bit_zero()

    @staticmethod
    def __build_out_list(messages: List[tuple[_nt.Message, int]]) -> _nt.Cell:
        out_list = _nt.Cell()
        for message, flags in messages:
            builder = _nt.CellBuilder()
            builder.store_u32(_op_action_send_msg)
            builder.store_u8(flags)
            builder.store_reference(out_list)
            builder.store_reference(message.build_cell())
            out_list = builder.build()
        return out_list

    async def __get_seqno_and_state_init(self) -> tuple[int, Optional[_nt.StateInit]]:
        account_state = await self.get_account_state()
        if account_state is not None and account_state.state_init is not None:
            if account_state.state_init.data is None:
                raise RuntimeError("Account state does not contain data")

            data = account_state.state_init.data.as_slice()
            data.load_bit()
            seqno = data.load_u32()

            # NOTE: Update wallet_id just in case
            self._wallet_id = data.load_u32()

            return seqno, None
        else:
            return 0, self._state_init
