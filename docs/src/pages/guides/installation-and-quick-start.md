# Installation & Quick Start

This section will guide you through the process of installing Nekoton-Python and running a simple example to ensure everything is set up correctly.

## Prerequisites

- Python 3.9 or higher. You can check your Python version by running `python --version` in your terminal.

## Installation

Nekoton-Python can be installed using pip. However, it is recommended to use a virtual environment to isolate your project dependencies. Here's how you can set up a virtual environment and install Nekoton-Python:

```bash
# Create a new virtual environment
python -m venv env

# Activate the virtual environment
# On Windows
env\Scripts\activate

# On Unix or MacOS
source env/bin/activate

# Now you can install Nekoton-Python in this isolated environment
pip install nekoton
```

## Quick Start

Once you have installed Nekoton-Python, you can verify the installation by running a simple script. The following script creates a connection to the blockchain using the `JrpcTransport` module, checks the connection, and then interacts with a smart contract.

```python
import asyncio
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


async def main():
    transport = nt.JrpcTransport('https://jrpc-broxustestnet.everwallet.net')
    await transport.check_connection()

    giver = Giver(transport, Address('-1:1111111111111111111111111111111111111111111111111111111111111111'))

    await giver.give(giver.address, nt.Tokens(10))


asyncio.run(main())
```
