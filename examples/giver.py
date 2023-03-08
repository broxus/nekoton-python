import nekoton as nt

giver_abi = nt.ContractAbi("""{
    "ABI version": 1,
    "functions": [{
        "name": "sendGrams",
        "inputs": [
            {"name": "dest", "type": "address"},
            {"name": "amount", "type": "uint64"}
        ],
        "outputs": []
    }],
    "events": []
}""")

send_grams = giver_abi.get_function("sendGrams")
assert send_grams is not None


class Giver:
    def __init__(self, transport: nt.Transport, address: nt.Address):
        self._transport = transport
        self._address = address

    @property
    def address(self) -> nt.Address:
        return self._address

    async def give(self, target: nt.Address, amount: nt.Tokens):
        # Prepare external message
        message = send_grams.encode_external_message(
            self._address,
            input={
                "dest": target,
                "amount": amount,
            },
            public_key=None
        ).without_signature()

        # Send external message
        tx = await self._transport.send_external_message(message)
        if tx is None:
            raise RuntimeError("Message expired")

        # Wait until all transactions are produced
        await self._transport.trace_transaction(tx).wait()
