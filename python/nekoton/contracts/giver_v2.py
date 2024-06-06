from __future__ import annotations

from . import IGiver
import nekoton as _nt

_giver_v2_abi = _nt.ContractAbi("""{
    "ABI version": 2,
    "header": ["time", "expire"],
    "functions": [{
        "name": "constructor",
        "inputs": [],
        "outputs": []
    }, {
        "name": "sendTransaction",
        "inputs": [
            {"name":"dest","type":"address"},
            {"name":"value","type":"uint128"},
            {"name":"bounce","type":"bool"}
        ],
        "outputs": []
    }],
    "events": []
}""")

_giver_v2_constructor = _giver_v2_abi.get_function("constructor")
_giver_v2_send_grams = _giver_v2_abi.get_function("sendTransaction")
_giver_v2_tvc = "te6ccgECIAEAA6YAAgE0BgEBAcACAgPPIAUDAQHeBAAD0CAAQdgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAIm/wD0pCAiwAGS9KDhiu1TWDD0oQkHAQr0pCD0oQgAAAIBIA0KAQL/CwH+fyHtRNAg10nCAZ/T/9MA9AX4an/4Yfhm+GKOG/QFbfhqcAGAQPQO8r3XC//4YnD4Y3D4Zn/4YeLTAAGOEoECANcYIPkBWPhCIPhl+RDyqN4j+EUgbpIwcN74Qrry4GUh0z/THzQg+CO88rki+QAg+EqBAQD0DiCRMd7y0Gb4AAwANiD4SiPIyz9ZgQEA9EP4al8E0x8B8AH4R27yfAIBIBQOAgFYEg8BCbjomPxQEAHW+EFujhLtRNDT/9MA9AX4an/4Yfhm+GLe0XBtbwL4SoEBAPSGlQHXCz9/k3BwcOKRII4yXzPIIs8L/yHPCz8xMQFvIiGkA1mAIPRDbwI0IvhKgQEA9HyVAdcLP3+TcHBw4gI1MzHoXwMhwP8RAJiOLiPQ0wH6QDAxyM+HIM6NBAAAAAAAAAAAAAAAAA90TH4ozxYhbyICyx/0AMlx+wDeMMD/jhL4QsjL//hGzwsA+EoB9ADJ7VTef/hnAQm5Fqvn8BMAtvhBbo427UTQINdJwgGf0//TAPQF+Gp/+GH4Zvhijhv0BW34anABgED0DvK91wv/+GJw+GNw+GZ/+GHi3vhG8nNx+GbR+AD4QsjL//hGzwsA+EoB9ADJ7VR/+GcCASAYFQEJuxXvk1gWAbb4QW6OEu1E0NP/0wD0Bfhqf/hh+Gb4Yt76QNcNf5XU0dDTf9/XDACV1NHQ0gDf0VRxIMjPhYDKAHPPQM4B+gKAa89AyXP7APhKgQEA9IaVAdcLP3+TcHBw4pEgFwCEjigh+CO7myL4SoEBAPRbMPhq3iL4SoEBAPR8lQHXCz9/k3BwcOICNTMx6F8G+ELIy//4Rs8LAPhKAfQAye1Uf/hnAgEgGxkBCbjkYYdQGgC++EFujhLtRNDT/9MA9AX4an/4Yfhm+GLe1NH4RSBukjBw3vhCuvLgZfgA+ELIy//4Rs8LAPhKAfQAye1U+A8g+wQg0O0e7VPwAjD4QsjL//hGzwsA+EoB9ADJ7VR/+GcCAtoeHAEBSB0ALPhCyMv/+EbPCwD4SgH0AMntVPgP8gABAUgfAFhwItDWAjHSADDcIccA3CHXDR/yvFMR3cEEIoIQ/////byx8nwB8AH4R27yfA=="


class GiverV2(IGiver):
    @classmethod
    def compute_address(
        cls, public_key: _nt.PublicKey, workchain: int = 0
    ) -> _nt.Address:
        return cls.compute_state_init(public_key).compute_address(workchain)

    @staticmethod
    def compute_state_init(public_key: _nt.PublicKey) -> _nt.StateInit:
        state_init = _nt.StateInit.decode(_giver_v2_tvc)
        state_init.data = _giver_v2_abi.encode_init_data(
            {}, public_key, state_init.data
        )
        return state_init

    @staticmethod
    def from_address(
        transport: _nt.Transport, keypair: _nt.KeyPair, address: _nt.Address
    ) -> "GiverV2":
        wallet = GiverV2(transport, keypair)
        wallet._address = address
        return wallet

    @classmethod
    async def deploy(
        cls,
        transport: _nt.Transport,
        keypair: _nt.KeyPair,
        workchain: int = 0,
        other_giver: IGiver | None = None,
    ) -> "GiverV2":
        # Compute giver address
        state_init = cls.compute_state_init(keypair.public_key)
        address = state_init.compute_address(workchain)

        # Ensure that giver account exists
        initial_balance = _nt.Tokens(1)
        state = await transport.get_account_state(address)
        if state is None:
            if other_giver is None:
                raise RuntimeError("Account does not have enough balance")

            tx = await other_giver.give(address, initial_balance)
            if tx is None:
                raise RuntimeError("Message expired")
            await transport.trace_transaction(tx).wait()

        # Deploy account
        if state.status == _nt.AccountStatus.Active:
            return GiverV2(transport, workchain)
        elif state.status == _nt.AccountStatus.Frozen:
            raise RuntimeError("Giver account is frozen")
        elif (
            state.status == _nt.AccountStatus.Uninit and state.balance < initial_balance
        ):
            tx = await other_giver.give(address, initial_balance)
            if tx is None:
                raise RuntimeError("Message expired")
            await transport.trace_transaction(tx).wait()

        signature_id = await transport.get_signature_id()
        external_message = _giver_v2_constructor.encode_external_message(
            address,
            input={},
            public_key=keypair.public_key,
            state_init=state_init,
        ).sign(keypair, signature_id)
        tx = await transport.send_external_message(external_message)
        if tx is None:
            raise RuntimeError("Message expired")
        await transport.trace_transaction(tx).wait()

        return GiverV2(transport, workchain)

    def __init__(
        self, transport: _nt.Transport, keypair: _nt.KeyPair, workchain: int = 0
    ):
        self._transport = transport
        self._keypair = keypair
        self._address = GiverV2.compute_address(keypair.public_key, workchain)

    @property
    def address(self) -> _nt.Address:
        return self._address

    async def give(self, target: _nt.Address, amount: _nt.Tokens):
        signature_id = await self._transport.get_signature_id()

        # Prepare external message
        message = _giver_v2_send_grams.encode_external_message(
            self._address,
            input={
                "dest": target,
                "value": amount,
                "bounce": False,
            },
            public_key=self._keypair.public_key,
        ).sign(self._keypair, signature_id)

        # Send external message
        tx = await self._transport.send_external_message(message)
        if tx is None:
            raise RuntimeError("Message expired")

        # Wait until all transactions are produced
        await self._transport.trace_transaction(tx).wait()
