from __future__ import annotations

from . import IGiver
import nekoton as _nt

_giver_v1_abi = _nt.ContractAbi("""{
    "ABI version": 1,
    "functions": [{
        "name": "constructor",
        "inputs": [],
        "outputs": []
    }, {
        "name": "sendGrams",
        "inputs": [
            {"name": "dest", "type": "address"},
            {"name": "amount", "type": "uint64"}
        ],
        "outputs": []
    }],
    "events": []
}""")

_giver_v1_constructor = _giver_v1_abi.get_function("constructor")
_giver_v1_send_grams = _giver_v1_abi.get_function("sendGrams")
_giver_v1_tvc = "te6ccgECJQEABaMAAgE0BgEBAcACAgPPIAUDAQHeBAAD0CAAQdgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAIo/wAgwAH0pCBYkvSg4YrtU1gw9KATBwEK9KQg9KEIAgPNQBAJAgHODQoCASAMCwAHDDbMIAAdPAZIbzyvCEhcHHwCl8CgAgEgDw4AASAA1T++wFkZWNvZGVfYWRkciD6QDL6QiBvECByuiFzurHy4H0hbxFu8uB9yHTPCwIibxLPCgcibxMicrqWI28TIs4ynyGBAQAi10mhz0AyICLOMuL+/AFkZWNvZGVfYWRkcjAhydAlVUFfBdswgAgEgEhEAK6T/fYCzsrovsTC2MLcxsvwTt4htmEAApaV/fYCwsa+6OTC3ObMyuWQ5Z6ARZ4UAOOegfBRnixJnixH9ATjnoDh9ATh9AUAgZ6B8EeeFj7lnoBBkkX2Af3+AsLGvujkwtzmzMrkvsrcyL4LAAgEgGhQB4P/+/QFtYWluX2V4dGVybmFsIY5Z/vwBZ2V0X3NyY19hZGRyINAg0wAycL2OGv79AWdldF9zcmNfYWRkcjBwyMnQVRFfAtsw4CBy1yExINMAMiH6QDP+/QFnZXRfc3JjX2FkZHIxISFVMV8E2zDYMSEVAfiOdf7+AWdldF9tc2dfcHVia2V5IMcCjhb+/wFnZXRfbXNnX3B1YmtleTFwMdsw4NUgxwGOF/7/AWdldF9tc2dfcHVia2V5MnAxMdsw4CCBAgDXIdcL/yL5ASIi+RDyqP7/AWdldF9tc2dfcHVia2V5MyADXwPbMNgixwKzFgHMlCLUMTPeJCIijjj++QFzdG9yZV9zaWdvACFvjCJvjCNvjO1HIW+M7UTQ9AVvjCDtV/79AXN0b3JlX3NpZ19lbmRfBdgixwGOE/78AW1zZ19pc19lbXB0eV8G2zDgItMfNCPTPzUgFwF2joDYji/+/gFtYWluX2V4dGVybmFsMiQiVXFfCPFAAf7+AW1haW5fZXh0ZXJuYWwzXwjbMOCAfPLwXwgYAf7++wFyZXBsYXlfcHJvdHBwcO1E0CD0BDI0IIEAgNdFmiDTPzIzINM/MjKWgggbd0Ay4iIluSX4I4ED6KgkoLmwjinIJAH0ACXPCz8izws/Ic8WIMntVP78AXJlcGxheV9wcm90Mn8GXwbbMOD+/AFyZXBsYXlfcHJvdDNwBV8FGQAE2zACASAcGwAPvOP3EDmG2YQCASAeHQCJuyXMvJ+ADwINM/MPAi/vwBcHVzaHBkYzd0b2M07UTQ9AHI7UdvEgH0ACHPFiDJ7VT+/QFwdXNocGRjN3RvYzQwXwLbMIAgEgIh8BCbiJACdQIAH+/v0BY29uc3RyX3Byb3RfMHBwgggbd0DtRNAg9AQyNCCBAIDXRY4UINI/MjMg0j8yMiBx10WUgHvy8N7eyCQB9AAjzws/Is8LP3HPQSHPFiDJ7VT+/QFjb25zdHJfcHJvdF8xXwX4ADDwIf78AXB1c2hwZGM3dG9jNO1E0PQByCEARO1HbxIB9AAhzxYgye1U/v0BcHVzaHBkYzd0b2M0MF8C2zAB4tz+/QFtYWluX2ludGVybmFsIY5Z/vwBZ2V0X3NyY19hZGRyINAg0wAycL2OGv79AWdldF9zcmNfYWRkcjBwyMnQVRFfAtsw4CBy1yExINMAMiH6QDP+/QFnZXRfc3JjX2FkZHIxISFVMV8E2zDYJCFwIwHqjjj++QFzdG9yZV9zaWdvACFvjCJvjCNvjO1HIW+M7UTQ9AVvjCDtV/79AXN0b3JlX3NpZ19lbmRfBdgixwCOHCFwuo4SIoIQXH7iB1VRXwbxQAFfBtsw4F8G2zDg/v4BbWFpbl9pbnRlcm5hbDEi0x80InG6JAA2niCAI1VhXwfxQAFfB9sw4CMhVWFfB/FAAV8H"


class GiverV1(IGiver):
    @staticmethod
    def compute_address(workchain: int = 0) -> _nt.Address:
        return _nt.Address.from_parts(
            workchain, _nt.Cell.decode(_giver_v1_tvc).repr_hash
        )

    @staticmethod
    async def deploy(
        transport: _nt.Transport,
        workchain: int = 0,
        other_giver: IGiver | None = None,
    ) -> GiverV1:
        # Compute giver address
        state_init_cell = _nt.Cell.decode(_giver_v1_tvc)
        address = _nt.Address.from_parts(workchain, state_init_cell.repr_hash)
        state_init = _nt.StateInit.from_cell(state_init_cell)

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
            return GiverV1(transport, workchain)
        elif state.status == _nt.AccountStatus.Frozen:
            raise RuntimeError("Giver account is frozen")
        elif (
            state.status == _nt.AccountStatus.Uninit and state.balance < initial_balance
        ):
            tx = await other_giver.give(address, initial_balance)
            if tx is None:
                raise RuntimeError("Message expired")
            await transport.trace_transaction(tx).wait()

        external_message = _giver_v1_constructor.encode_external_message(
            address,
            input={},
            public_key=None,
            state_init=state_init,
        ).without_signature()
        tx = await transport.send_external_message(external_message)
        if tx is None:
            raise RuntimeError("Message expired")
        await transport.trace_transaction(tx).wait()

        return GiverV1(transport, workchain)

    @staticmethod
    def from_address(transport: _nt.Transport, address: _nt.Address) -> "GiverV1":
        giver = GiverV1(transport)
        giver._address = address
        return giver

    def __init__(self, transport: _nt.Transport, workchain: int = 0):
        self._transport = transport
        self._address = GiverV1.compute_address(workchain)

    @property
    def address(self) -> _nt.Address:
        return self._address

    async def give(self, target: _nt.Address, amount: _nt.Tokens):
        # Prepare external message
        message = _giver_v1_send_grams.encode_external_message(
            self._address,
            input={
                "dest": target,
                "amount": amount,
            },
            public_key=None,
        ).without_signature()

        # Send external message
        tx = await self._transport.send_external_message(message)
        if tx is None:
            raise RuntimeError("Message expired")

        # Wait until all transactions are produced
        await self._transport.trace_transaction(tx).wait()
