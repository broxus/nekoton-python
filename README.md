<p align="center">
  <a href="https://github.com/venom-blockchain/developer-program">
    <img src="https://raw.githubusercontent.com/venom-blockchain/developer-program/main/vf-dev-program.png" alt="Logo" width="366.8" height="146.4">
  </a>
</p>

# nekoton-python &emsp;  [![Latest Version]][pypi.org]

Python bindings for Nekoton

## Usage

### Install

```
pip install nekoton
```

### Example

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

### Local development

* Install [`maturin`](https://www.maturin.rs/installation.html).
* Configure virtual env:
  ```bash
  python -m venv .env
  source .env/bin/activate
  ```
* Dev build:
  ```bash
  maturin develop
  ```
* Publish
  ```bash
  maturin build --release --zig --strip
  maturin upload path/to/generated/file.whl
  ```

## Contributing

We welcome contributions to the project! If you notice any issues or errors, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [License Apache].

[latest version]: https://img.shields.io/pypi/v/nekoton
[pypi.org]: https://pypi.org/project/nekoton/
