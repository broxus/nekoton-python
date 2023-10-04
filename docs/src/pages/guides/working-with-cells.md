---
outline: [2, 4]
---

# Working with Cells

In the context of TVM blockchain, [`cells`](./../concepts/data-representation.md#cells-the-fundamental-units-of-data) are the fundamental storage units that encapsulate and manipulate data in contracts. Cells are the basic building blocks of data in the blockchain. They are used to store and manipulate data in contracts. Cells can contain tokens, which are used to represent different types of data. This guide will walk you through the basic methods for working with cells provided by `nekoton-python`.

We'll use the following ABI definition in our examples:

```python
from nekoton import * as nt

ABI = [
    ("name", nt.AbiString()),
    ("age", nt.AbiUint(32)),
    ("city", nt.AbiString()),
]
```

## Building Cells

To build a cell, you can use the `build_cell` method. This method takes a Python object and an ABI structure, and builds a cell from the data.

```python
# Define the data to build into a cell.
data_to_build = {
    "name": "Alice",
    "age": 30,
    "city": "New York",
}

# Build the cell from the data.
cell = nt.Cell.build(abi=ABI, value=data_to_build)

print(cell)
```

##### Result

```python
<Cell repr_hash='445f574be946d8ebaac1368365e7c499b69fce42bae51a0fd3b925b1b4785f46', bits=32, refs=2>
```

### Build from Bytes

You can also build a cell from bytes using the `from_bytes` method. This method takes a `bytes` object and builds a cell from the data.

:::details Bytes Definition

```python
cell_bytes = b'\xb5\xee\x9cr\x01\x01\x03\x01\x00\x19\x00\x02\x08\x00\x00\x00\x1e\x02\x01\x00\x10New York\x00\nAlice'
```

:::

```python
cell = nt.Cell.from_bytes(cell_bytes)

print(cell)
```

##### Result

```python
<Cell repr_hash='445f574be946d8ebaac1368365e7c499b69fce42bae51a0fd3b925b1b4785f46', bits=32, refs=2>
```

## Encoding Cells

To encode data into a cell, you can use the `encode` method from the `Cell` class. This method encodes the data into a `base64` encoded `BOC` (Bag of Cells) string.

```python
# Build the cell from the data.
cell = Cell.build(abi=ABI, value=data_to_build)

# Encode the cell.
encoded_cell = cell.encode("base64")

print("Encoded cell:", encoded_cell)
```

##### Result

```python
Encoded cell: te6ccgEBAwEAGQACCAAAAB4CAQAQTmV3IFlvcmsACkFsaWNl
```

## Decoding Cells

To decode a cell, you can use the `decode` method from the `Cell` class. This method takes a `base64` encoded `BOC` (Bag of Cells) string and decodes the cell.

```python

encoded_cell = "te6ccgEBAwEAGQACCAAAAB4CAQAQTmV3IFlvcmsACkFsaWNl"

# Decode the data from BOC.
decoded = nt.Cell.decode(encoded_cell)

print(decoded)
```

##### Result

```python
<Cell repr_hash='445f574be946d8ebaac1368365e7c499b69fce42bae51a0fd3b925b1b4785f46', bits=32, refs=2>
```

## Cell Hashing & Comparison

Hashing and comparing cells are important operations when working with blockchain data. `nekoton-python` provides convenient methods for these operations as well.

### Cell Hashing

To compute the hash of a cell, you can use the `repr_hash` property of the `Cell` class. This property returns the representation hash of the cell.

```python
# Compute the hash of the cell.
cell_hash = cell.repr_hash

print(cell_hash)
```

##### Result

```python
b'D_WK\xe9F\xd8\xeb\xaa\xc16\x83e\xe7\xc4\x99\xb6\x9f\xceB\xba\xe5\x1a\x0f\xd3\xb9%\xb1\xb4x_F'
```

### Cell Comparison

To compare two cells, you can use the equality operator (`==`). This operator compares two cells for equality.

```python
# Define the cells to compare.
cell2 = nt.Cell.build(abi=ABI, value={
    "name": "Bob",
    "age": 27,
    "city": "Wroclaw",
})

# Compare the cells.
are_equal = cell == cell2 # False
```

## Unpacking Cells

To unpack a cell, you can use the `unpack` method from the `Cell` class. This method takes a cell and an ABI structure, and unpacks the data from the cell into a Python object.

```python
# Unpack the data from the cell.
unpacked = cell.unpack(ABI)

print(unpacked)
```

##### Result

```python
{'name': 'Alice', 'age': 30, 'city': 'New York'}
```

## Contract State (StateInit)

Interacting with the state of a contract is a common operation when working with the blockchain. In `nekoton-python`, the `StateInit` class provides ways to handle contract code and data, including encoding and decoding the state, computing an address for the state, and more.

:::tip Importance of Code Hash in TVM

The code hash plays a pivotal role:

- **Identification**: The code hash is a cryptographic representation of the contract's code, providing a unique identifier. Identical code hashes across contracts signal that their underlying codebases are the same.

- **Security**: Due to the cryptographic nature of the hash, knowing the hash doesn't allow one to deduce the original contract code, ensuring the confidentiality of the contract's content.

- **Indexing and Search**: The code hash is utilized as a key in various indexing systems, allowing for faster retrieval of contract information and efficient searches across the blockchain. It ensures that contracts with the same code can be grouped or identified quickly.

:::

### Decoding Contract State

The `StateInit` class provides several methods to decode the state of a contract.

#### From Raw Bytes

If you have the contract's state as raw bytes, you can decode it using the `StateInit.from_bytes` method:

:::details Raw Bytes Definition

```python
# Assume we have the contract's state as raw bytes
state_bytes = b'\xb5\xee\x9cr\x01\x02$\x01\x00\x05\x02\x00\x02\x014\x03\x01\x01\x01\xc0\x02\x00C\xd0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 \x04$\x8a\xedS \xe3\x03 \xc0\xff\xe3\x02 \xc0\xfe\xe3\x02\xf2\x0b!\x05\x04#\x02\xdc\xedD\xd0\xd7I\xc3\x01\xf8f\x8d\x08`\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\xf8i!\xdb<\xd3\x00\x01\x8e\x14\x83\x08\xd7\x18 \xf8(\xc8\xce\xce\xc9\xf9\x00X\xf8B\xf9\x10\xf2\xa8\xde\xd3?\x01\xf8C!\xb9\xf2\xb4 \xf8#\x81\x03\xe8\xa8\x82\x08\x1bw@\xa0\xb9\xf2\xb4\xf8c\xd3\x1f\x01\xdb<\xf2<\x0b\x06\x03z\xedD\xd0\xd7I\xc3\x01\xf8f"\xd0\xd3\x03\xfa@0\xf8i\xa98\x00\xf8D\x7foq\x82\x08\x98\x96\x80ormospot\xf8d\xdc!\xc7\x00\xe3\x02!\xd7\r\x1f\xf2\xbc!\xe3\x03\x01\xdb<\xf2<  \x06\x02( \x82\x10@\x02/r\xbb\xe3\x02 \x82\x10q\tkJ\xbb\xe3\x02\r\x07\x03< \x82\x10A>\xda\xcb\xba\xe3\x02 \x82\x10n\xbfV\xd2\xba\xe3\x02 \x82\x10q\tkJ\xba\xe3\x02\x0c\n\x08\x03r0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd1\xdb<!\x8e!#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xf1\tkJ\xcf\x0b\x81\x01o"\x02\xcb\x1f\xcc\xc9p\xfb\x00\x910\xe2\xe3\x00\xf2\x00\x1f\t\x1d\x00\x04\xf8L\x02H0\xf8Bn\xe3\x00\xf8F\xf2s\xd3\x7f\xd4\xd1\xf8\x00!\xf8k\x01\x81\x03\xe8\xa9\x08\xb5\x1f\x01o\x02\xf8l\xdb<\xf2\x00\x0b\x19\x02n\xedD\xd0\xd7I\xc2\x01\x8e\xacp\xedD\xd0\xf4\x05q!\x80@\xf4\x0eo\x91\x93\xd7\x0b\x1f\xdep \x88o\x02\xf8l\xf8k\xf8j\x80@\xf4\x0e\xf2\xbd\xd7\x0b\xff\xf8bp\xf8c\xe3\r#\x1f\x01P0\xd1\xdb<\xf8K!\x8e\x1c\x8d\x04p\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x000O\xb6\xb2\xe0\xc8\xce\xcb\x7f\xc9p\xfb\x00\xde\xf2\x00\x1f\x04P \x82\x10\x117\x0e\x00\xba\xe3\x02 \x82\x108\xda\xd0\xec\xba\xe3\x02 \x82\x10?\xe1h\x15\xba\xe3\x02 \x82\x10@\x02/r\xba\xe3\x02\x1c\x18\x10\x0e\x03(0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x7f\xd1\xdb<\xdb<\xf2\x00\x1f\x0f\x19\x01\n\xf8\x00\xdb<0\x1b\x03h0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd4\xd1\xdb<!\x8e\x1b#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xbf\xe1h\x15\xcf\x0b\x81\xcc\xc9p\xfb\x00\x910\xe2\xe3\x00\xf2\x00\x1f\x11\x1d\x01\x0c\xf8Lo\x11\xdb<\x12\x04<\x01\xdb<X\xd0_2\xdb<33\x94 q\xd7F\x8e\x88\xd51_2\xdb<33\xe80\xdb<\x16\x15\x15\x13\x01$\x96!o\x88\xc0\x00\xb3\x8e\x86!\xdb<3\xcf\x11\xe8\xc91\x14\x00\x1co\x8do\x8dY o\x88\x92o\x8c\x910\xe2\x01R!\xcf5\xa6\xf9!\xd7K \x96#p"\xd714\xde0!\xbb\x8e\x8d\\\xd7\x183#\xce3]\xdb<4\xc83\xdfS\x12\xcel1\x17\x010o\x00\x01\xd0\x95 \xd7J\xc3\x00\x8e\x89\xd5\x01\xc8\xceR \xdb<2\xe8\xc8\xce\x17\x008Q\x10o\x88\x9eo\x8d o\x88\x84\x07\xa1\x94o\x8co\x00\xdf\x92o\x00\xe2Xo\x8co\x8c\x03l0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x7f\xd1\xdb<!\x8e\x1c#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xb8\xda\xd0\xec\xcf\x0b\x81\xcb\x1f\xc9p\xfb\x00\x910\xe2\xdb<\xf2\x00\x1f\x1a\x19\x00>\xf8L\xf8K\xf8J\xf8C\xf8B\xc8\xcb\xff\xcb?\xcf\x83\xcb\x1f\xcb\x7f\x01o"\x02\xcb\x1f\xcc\xc9\xedT\x01B\xf8\'o\x10h\xa6\xfe`\xa1\xb5\x7fr\xfb\x02\xdb<\xf8I\xc8\xcf\x85\x88\xce\x80o\xcf@\xc9\x81\x00\x81\xfb\x00\x1b\x00n \xf8k\x81\x03\xe8\xa9\x08\xb5\x1f\xf8L\x01oP \xf8l\x8d\x04p\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x14\xcer)`\xc8\xce\x01o"\x02\xcb\x1f\xcc\xc9p\xfb\x00\xf8Lo\x10\x03\xf00\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x1f\xf8DXou\xf8d\xd3\x1f\xd1\xdb<!\x8e\x1f#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\x917\x0e\x00\xcf\x0b\x81\x01o"\x02\xcb\x1f\xcc\xc9p\x8e4\xf8D o\x13!o\x12\xf8IU\x02o\x11\xc8\xcf\x84\x80\xca\x00\xcf\x84@\xce\x01\xfa\x02\xf4\x00\x80j\xcf@\xf8Do\x15\xcf\x0b\x1f\x01o"\x02\xcb\x1f\xcc\xc9\xf8Do\x14\xe2\xfb\x00\xe3\x00\xf2\x00\x1f\x1e\x1d\x00(\xedD\xd0\xd3\xff\xd3?1\xf8CX\xc8\xcb\xff\xcb?\xce\xc9\xedT\x01Z \x81\x03\xe8\xb9\xf2\xe59\xf8Lo\x10\xa0\xb5\x1fp\x88o\x02\x01oP\xf8Lo\x11oQ0\xf8Dpor\x80Dotpoq\xf8d\xf8L#\x00@\xedD\xd0\xd3\xff\xd3?\xd3\x001\xd3\x1f\xd3\x7f\xd3\x1f\xd4Yo\x02\x01\xd1\xf8l\xf8k\xf8j\xf8c\xf8b\x00\n\xf8F\xf2\xe0L\x02\x10\xf4\xa4 \xf4\xbd\xf2\xc0N#"\x00\x14sol 0.66.0\x00\x00'
```

:::

```python
# Decode the contract's state
state_init = StateInit.from_bytes(state_bytes)

print(state_init)
```

##### Result

```python
<StateInit code_hash=''1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554'', data_hash=''55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d''>
```

#### From a Cell

If you have the contract's state as a cell, you can decode it using the `StateInit.from_cell` method:

```python
# Assume we have the contract's state as a cell
state_cell = Cell.from_bytes(state_bytes)

# Decode the contract's state
state_init = StateInit.from_cell(state_cell)

print(state_init)
```

##### Result

```python
<StateInit code_hash='1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554', data_hash='55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d'>
```

#### From an Encoded BOC

If you have the contract's BOC (Bag of Cells) encoded as base64 or hex, you can decode it using the `StateInit.decode` method:

::: details Encoded BOC as base64

```python
boc_base64 = 'te6ccgECJAEABQIAAgE0AwEBAcACAEPQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgBCSK7VMg4wMgwP/jAiDA/uMC8gshBQQjAtztRNDXScMB+GaNCGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT4aSHbPNMAAY4UgwjXGCD4KMjOzsn5AFj4QvkQ8qje0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B2zzyPAsGA3rtRNDXScMB+GYi0NMD+kAw+GmpOAD4RH9vcYIImJaAb3Jtb3Nwb3T4ZNwhxwDjAiHXDR/yvCHjAwHbPPI8ICAGAiggghBAAi9yu+MCIIIQcQlrSrvjAg0HAzwgghBBPtrLuuMCIIIQbr9W0rrjAiCCEHEJa0q64wIMCggDcjD4RvLgTPhCbuMA0ds8IY4hI9DTAfpAMDHIz4cgzoIQ8QlrSs8LgQFvIgLLH8zJcPsAkTDi4wDyAB8JHQAE+EwCSDD4Qm7jAPhG8nPTf9TR+AAh+GsBgQPoqQi1HwFvAvhs2zzyAAsZAm7tRNDXScIBjqxw7UTQ9AVxIYBA9A5vkZPXCx/ecCCIbwL4bPhr+GqAQPQO8r3XC//4YnD4Y+MNIx8BUDDR2zz4SyGOHI0EcAAAAAAAAAAAAAAAADBPtrLgyM7Lf8lw+wDe8gAfBFAgghARNw4AuuMCIIIQONrQ7LrjAiCCED/haBW64wIgghBAAi9yuuMCHBgQDgMoMPhG8uBM+EJu4wDTf9HbPNs88gAfDxkBCvgA2zwwGwNoMPhG8uBM+EJu4wDU0ds8IY4bI9DTAfpAMDHIz4cgzoIQv+FoFc8LgczJcPsAkTDi4wDyAB8RHQEM+ExvEds8EgQ8Ads8WNBfMts8MzOUIHHXRo6I1TFfMts8MzPoMNs8FhUVEwEkliFviMAAs46GIds8M88R6MkxFAAcb41vjVkgb4iSb4yRMOIBUiHPNab5IddLIJYjcCLXMTTeMCG7jo1c1xgzI84zXds8NMgz31MSzmwxFwEwbwAB0JUg10rDAI6J1QHIzlIg2zwy6MjOFwA4URBviJ5vjSBviIQHoZRvjG8A35JvAOJYb4xvjANsMPhG8uBM+EJu4wDTf9HbPCGOHCPQ0wH6QDAxyM+HIM6CELja0OzPC4HLH8lw+wCRMOLbPPIAHxoZAD74TPhL+Er4Q/hCyMv/yz/Pg8sfy38BbyICyx/Mye1UAUL4J28QaKb+YKG1f3L7Ats8+EnIz4WIzoBvz0DJgQCB+wAbAG4g+GuBA+ipCLUf+EwBb1Ag+GyNBHAAAAAAAAAAAAAAAAAUznIpYMjOAW8iAssfzMlw+wD4TG8QA/Aw+Eby4Ez4Qm7jANMf+ERYb3X4ZNMf0ds8IY4fI9DTAfpAMDHIz4cgzoIQkTcOAM8LgQFvIgLLH8zJcI40+EQgbxMhbxL4SVUCbxHIz4SAygDPhEDOAfoC9ACAas9A+ERvFc8LHwFvIgLLH8zJ+ERvFOL7AOMA8gAfHh0AKO1E0NP/0z8x+ENYyMv/yz/Oye1UAVoggQPoufLlOfhMbxCgtR9wiG8CAW9Q+ExvEW9RMPhEcG9ygERvdHBvcfhk+EwjAEDtRNDT/9M/0wAx0x/Tf9Mf1FlvAgHR+Gz4a/hq+GP4YgAK+Eby4EwCEPSkIPS98sBOIyIAFHNvbCAwLjY2LjAAAA=='
```

:::

```python
# Decode the contract's state
state_init = StateInit.decode(boc_base64, encoding='base64')

print(state_init)
```

##### Result

```python
<StateInit code_hash='1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554', data_hash='55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d'>
```

### Accessing and Modifying Contract Code and Data

You can access the contract's code and data:

```python
# Access the contract's code and data
code = state_init.code
data = state_init.data

print("Code:", code)
print("Data:", data)
```

##### Result

```python
Code: <Cell repr_hash='1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554', bits=144, refs=4>
Data: <Cell repr_hash='55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d', bits=1, refs=1>
```

#### Modifying the Contract's Code

Code Salt is a random data added to the code before hashing. This ensures that even if two contracts have identical code, their combined salted content results in distinct hashes, thereby generating different contract addresses.

:::tip

The salt holds significant value:

- **Uniqueness**: A salt is random data added to the code before hashing. This ensures that even if two contracts have identical code, their combined salted content results in distinct hashes, thereby generating different contract addresses.

- **Enhanced Security**: Salts deter dictionary attacks and precomputed hash (rainbow table) attacks. By ensuring every contract, even with similar code, can possess a unique hash when combined with its salt, salts render these attack methods ineffective.

- **Adjustable Properties**: In specific scenarios, a salt can be employed to tweak the behavior or properties of a contract without modifying the actual code.

:::

You can update the code salt:

```python
# Define ABI for the code salt cell
ABI = [
    ("name", nt.AbiString()),
    ("secret", nt.AbiString()),
]

# Define the data to build into a cell.
data_to_build = {
    "name": "Alice",
    "secret": "9c1b512d6296870f78d145713bf",
}

# Build the cell from the data.
new_salt = nt.Cell.build(abi=ABI, value=data_to_build)

print("Old StateInit: ", state_init)

# Update the code salt
state_init.set_code_salt(new_salt)

print("New StateInit: ", state_init)
```

##### Result

```python
Old StateInit: <StateInit code_hash='1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554', data_hash='55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d'>

New StateInit: <StateInit code_hash='a1b1f194845bad7c5085140cdf7aac422756002d7da2bfd1dc8e2f8cb47faac8', data_hash='55a703465a160dce20481375de2e5b830c841c2787303835eb5821d62d65ca9d'>
```

Now, the code hash has changed, but the data hash remains the same.

#### Reading the Code Salt

When you need to read the code salt, you can use the `get_code_salt` method:

```python
# Extract the code salt
code_salt = state_init.get_code_salt()

print(code_salt)
```

##### Result

```python
<Cell repr_hash='445f574be946d8ebaac1368365e7c499b69fce42bae51a0fd3b925b1b4785f46', bits=32, refs=2>
```

### Computing the Contract's Address

You can compute an address for the `StateInit`:

```python
# Compute an address for the StateInit
address = state_init.compute_address(workchain=0)

print(address)
```

##### Result

```python
0:6fa47a67904d11c76c9ebffb2750bfd2042d38bd689741752a146c9d4c063c84
```

### Building a Cell with StateInit

You can create a new cell with `StateInit`:

```python
# Create a new cell with StateInit
cell = state_init.build_cell()

print(cell)
```

##### Result

```python
<Cell repr_hash='6fa47a67904d11c76c9ebffb2750bfd2042d38bd689741752a146c9d4c063c84', bits=5, refs=2>
```

### Encoding Contract State

To encode the state of a contract into BOC or raw bytes, you can use the following methods:

```python
# Encode into BOC (base64 or hex)
encoded_state = state_init.encode(encoding='base64')
```

:::details Encoded State as base64

```
te6ccgECJAEABQIAAgE0AwEBAcACAEPQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgBCSK7VMg4wMgwP/jAiDA/uMC8gshBQQjAtztRNDXScMB+GaNCGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT4aSHbPNMAAY4UgwjXGCD4KMjOzsn5AFj4QvkQ8qje0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B2zzyPAsGA3rtRNDXScMB+GYi0NMD+kAw+GmpOAD4RH9vcYIImJaAb3Jtb3Nwb3T4ZNwhxwDjAiHXDR/yvCHjAwHbPPI8ICAGAiggghBAAi9yu+MCIIIQcQlrSrvjAg0HAzwgghBBPtrLuuMCIIIQbr9W0rrjAiCCEHEJa0q64wIMCggDcjD4RvLgTPhCbuMA0ds8IY4hI9DTAfpAMDHIz4cgzoIQ8QlrSs8LgQFvIgLLH8zJcPsAkTDi4wDyAB8JHQAE+EwCSDD4Qm7jAPhG8nPTf9TR+AAh+GsBgQPoqQi1HwFvAvhs2zzyAAsZAm7tRNDXScIBjqxw7UTQ9AVxIYBA9A5vkZPXCx/ecCCIbwL4bPhr+GqAQPQO8r3XC//4YnD4Y+MNIx8BUDDR2zz4SyGOHI0EcAAAAAAAAAAAAAAAADBPtrLgyM7Lf8lw+wDe8gAfBFAgghARNw4AuuMCIIIQONrQ7LrjAiCCED/haBW64wIgghBAAi9yuuMCHBgQDgMoMPhG8uBM+EJu4wDTf9HbPNs88gAfDxkBCvgA2zwwGwNoMPhG8uBM+EJu4wDU0ds8IY4bI9DTAfpAMDHIz4cgzoIQv+FoFc8LgczJcPsAkTDi4wDyAB8RHQEM+ExvEds8EgQ8Ads8WNBfMts8MzOUIHHXRo6I1TFfMts8MzPoMNs8FhUVEwEkliFviMAAs46GIds8M88R6MkxFAAcb41vjVkgb4iSb4yRMOIBUiHPNab5IddLIJYjcCLXMTTeMCG7jo1c1xgzI84zXds8NMgz31MSzmwxFwEwbwAB0JUg10rDAI6J1QHIzlIg2zwy6MjOFwA4URBviJ5vjSBviIQHoZRvjG8A35JvAOJYb4xvjANsMPhG8uBM+EJu4wDTf9HbPCGOHCPQ0wH6QDAxyM+HIM6CELja0OzPC4HLH8lw+wCRMOLbPPIAHxoZAD74TPhL+Er4Q/hCyMv/yz/Pg8sfy38BbyICyx/Mye1UAUL4J28QaKb+YKG1f3L7Ats8+EnIz4WIzoBvz0DJgQCB+wAbAG4g+GuBA+ipCLUf+EwBb1Ag+GyNBHAAAAAAAAAAAAAAAAAUznIpYMjOAW8iAssfzMlw+wD4TG8QA/Aw+Eby4Ez4Qm7jANMf+ERYb3X4ZNMf0ds8IY4fI9DTAfpAMDHIz4cgzoIQkTcOAM8LgQFvIgLLH8zJcI40+EQgbxMhbxL4SVUCbxHIz4SAygDPhEDOAfoC9ACAas9A+ERvFc8LHwFvIgLLH8zJ+ERvFOL7AOMA8gAfHh0AKO1E0NP/0z8x+ENYyMv/yz/Oye1UAVoggQPoufLlOfhMbxCgtR9wiG8CAW9Q+ExvEW9RMPhEcG9ygERvdHBvcfhk+EwjAEDtRNDT/9M/0wAx0x/Tf9Mf1FlvAgHR+Gz4a/hq+GP4YgAK+Eby4EwCEPSkIPS98sBOIyIAFHNvbCAwLjY2LjAAAA==
```

:::

:::details Encoded State as hex

```
b5ee9c72010224010005020002013403010101c0020043d000000000000000000000000000000000000000000000000000000000000000002004248aed5320e30320c0ffe30220c0fee302f20b2105042302dced44d0d749c301f8668d0860000000000000000000000000000000000000000000000000000000000000000004f86921db3cd300018e148308d71820f828c8cecec9f90058f842f910f2a8ded33f01f84321b9f2b420f8238103e8a882081b7740a0b9f2b4f863d31f01db3cf23c0b06037aed44d0d749c301f86622d0d303fa4030f869a93800f8447f6f7182089896806f726d6f73706f74f864dc21c700e30221d70d1ff2bc21e30301db3cf23c202006022820821040022f72bbe30220821071096b4abbe3020d07033c208210413edacbbae3022082106ebf56d2bae30220821071096b4abae3020c0a08037230f846f2e04cf8426ee300d1db3c218e2123d0d301fa403031c8cf8720ce8210f1096b4acf0b81016f2202cb1fccc970fb009130e2e300f2001f091d0004f84c024830f8426ee300f846f273d37fd4d1f80021f86b018103e8a908b51f016f02f86cdb3cf2000b19026eed44d0d749c2018eac70ed44d0f40571218040f40e6f9193d70b1fde7020886f02f86cf86bf86a8040f40ef2bdd70bfff86270f863e30d231f015030d1db3cf84b218e1c8d0470000000000000000000000000304fb6b2e0c8cecb7fc970fb00def2001f045020821011370e00bae30220821038dad0ecbae3022082103fe16815bae30220821040022f72bae3021c18100e032830f846f2e04cf8426ee300d37fd1db3cdb3cf2001f0f19010af800db3c301b036830f846f2e04cf8426ee300d4d1db3c218e1b23d0d301fa403031c8cf8720ce8210bfe16815cf0b81ccc970fb009130e2e300f2001f111d010cf84c6f11db3c12043c01db3c58d05f32db3c3333942071d7468e88d5315f32db3c3333e830db3c16151513012496216f88c000b38e8621db3c33cf11e8c93114001c6f8d6f8d59206f88926f8c9130e2015221cf35a6f921d74b2096237022d73134de3021bb8e8d5cd7183323ce335ddb3c34c833df5312ce6c311701306f0001d09520d74ac3008e89d501c8ce5220db3c32e8c8ce17003851106f889e6f8d206f888407a1946f8c6f00df926f00e2586f8c6f8c036c30f846f2e04cf8426ee300d37fd1db3c218e1c23d0d301fa403031c8cf8720ce8210b8dad0eccf0b81cb1fc970fb009130e2db3cf2001f1a19003ef84cf84bf84af843f842c8cbffcb3fcf83cb1fcb7f016f2202cb1fccc9ed540142f8276f1068a6fe60a1b57f72fb02db3cf849c8cf8588ce806fcf40c9810081fb001b006e20f86b8103e8a908b51ff84c016f5020f86c8d047000000000000000000000000014ce722960c8ce016f2202cb1fccc970fb00f84c6f1003f030f846f2e04cf8426ee300d31ff844586f75f864d31fd1db3c218e1f23d0d301fa403031c8cf8720ce821091370e00cf0b81016f2202cb1fccc9708e34f844206f13216f12f84955026f11c8cf8480ca00cf8440ce01fa02f400806acf40f8446f15cf0b1f016f2202cb1fccc9f8446f14e2fb00e300f2001f1e1d0028ed44d0d3ffd33f31f84358c8cbffcb3fcec9ed54015a208103e8b9f2e539f84c6f10a0b51f70886f02016f50f84c6f116f5130f844706f7280446f74706f71f864f84c230040ed44d0d3ffd33fd30031d31fd37fd31fd4596f0201d1f86cf86bf86af863f862000af846f2e04c0210f4a420f4bdf2c04e23220014736f6c20302e36362e300000
```

:::

### Encode into raw bytes

```python
raw_bytes = state_init.to_bytes()
```

:::details Raw Bytes

```python
b'\xb5\xee\x9cr\x01\x02$\x01\x00\x05\x02\x00\x02\x014\x03\x01\x01\x01\xc0\x02\x00C\xd0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 \x04$\x8a\xedS \xe3\x03 \xc0\xff\xe3\x02 \xc0\xfe\xe3\x02\xf2\x0b!\x05\x04#\x02\xdc\xedD\xd0\xd7I\xc3\x01\xf8f\x8d\x08`\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\xf8i!\xdb<\xd3\x00\x01\x8e\x14\x83\x08\xd7\x18 \xf8(\xc8\xce\xce\xc9\xf9\x00X\xf8B\xf9\x10\xf2\xa8\xde\xd3?\x01\xf8C!\xb9\xf2\xb4 \xf8#\x81\x03\xe8\xa8\x82\x08\x1bw@\xa0\xb9\xf2\xb4\xf8c\xd3\x1f\x01\xdb<\xf2<\x0b\x06\x03z\xedD\xd0\xd7I\xc3\x01\xf8f"\xd0\xd3\x03\xfa@0\xf8i\xa98\x00\xf8D\x7foq\x82\x08\x98\x96\x80ormospot\xf8d\xdc!\xc7\x00\xe3\x02!\xd7\r\x1f\xf2\xbc!\xe3\x03\x01\xdb<\xf2<  \x06\x02( \x82\x10@\x02/r\xbb\xe3\x02 \x82\x10q\tkJ\xbb\xe3\x02\r\x07\x03< \x82\x10A>\xda\xcb\xba\xe3\x02 \x82\x10n\xbfV\xd2\xba\xe3\x02 \x82\x10q\tkJ\xba\xe3\x02\x0c\n\x08\x03r0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd1\xdb<!\x8e!#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xf1\tkJ\xcf\x0b\x81\x01o"\x02\xcb\x1f\xcc\xc9p\xfb\x00\x910\xe2\xe3\x00\xf2\x00\x1f\t\x1d\x00\x04\xf8L\x02H0\xf8Bn\xe3\x00\xf8F\xf2s\xd3\x7f\xd4\xd1\xf8\x00!\xf8k\x01\x81\x03\xe8\xa9\x08\xb5\x1f\x01o\x02\xf8l\xdb<\xf2\x00\x0b\x19\x02n\xedD\xd0\xd7I\xc2\x01\x8e\xacp\xedD\xd0\xf4\x05q!\x80@\xf4\x0eo\x91\x93\xd7\x0b\x1f\xdep \x88o\x02\xf8l\xf8k\xf8j\x80@\xf4\x0e\xf2\xbd\xd7\x0b\xff\xf8bp\xf8c\xe3\r#\x1f\x01P0\xd1\xdb<\xf8K!\x8e\x1c\x8d\x04p\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x000O\xb6\xb2\xe0\xc8\xce\xcb\x7f\xc9p\xfb\x00\xde\xf2\x00\x1f\x04P \x82\x10\x117\x0e\x00\xba\xe3\x02 \x82\x108\xda\xd0\xec\xba\xe3\x02 \x82\x10?\xe1h\x15\xba\xe3\x02 \x82\x10@\x02/r\xba\xe3\x02\x1c\x18\x10\x0e\x03(0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x7f\xd1\xdb<\xdb<\xf2\x00\x1f\x0f\x19\x01\n\xf8\x00\xdb<0\x1b\x03h0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd4\xd1\xdb<!\x8e\x1b#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xbf\xe1h\x15\xcf\x0b\x81\xcc\xc9p\xfb\x00\x910\xe2\xe3\x00\xf2\x00\x1f\x11\x1d\x01\x0c\xf8Lo\x11\xdb<\x12\x04<\x01\xdb<X\xd0_2\xdb<33\x94 q\xd7F\x8e\x88\xd51_2\xdb<33\xe80\xdb<\x16\x15\x15\x13\x01$\x96!o\x88\xc0\x00\xb3\x8e\x86!\xdb<3\xcf\x11\xe8\xc91\x14\x00\x1co\x8do\x8dY o\x88\x92o\x8c\x910\xe2\x01R!\xcf5\xa6\xf9!\xd7K \x96#p"\xd714\xde0!\xbb\x8e\x8d\\\xd7\x183#\xce3]\xdb<4\xc83\xdfS\x12\xcel1\x17\x010o\x00\x01\xd0\x95 \xd7J\xc3\x00\x8e\x89\xd5\x01\xc8\xceR \xdb<2\xe8\xc8\xce\x17\x008Q\x10o\x88\x9eo\x8d o\x88\x84\x07\xa1\x94o\x8co\x00\xdf\x92o\x00\xe2Xo\x8co\x8c\x03l0\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x7f\xd1\xdb<!\x8e\x1c#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\xb8\xda\xd0\xec\xcf\x0b\x81\xcb\x1f\xc9p\xfb\x00\x910\xe2\xdb<\xf2\x00\x1f\x1a\x19\x00>\xf8L\xf8K\xf8J\xf8C\xf8B\xc8\xcb\xff\xcb?\xcf\x83\xcb\x1f\xcb\x7f\x01o"\x02\xcb\x1f\xcc\xc9\xedT\x01B\xf8\'o\x10h\xa6\xfe`\xa1\xb5\x7fr\xfb\x02\xdb<\xf8I\xc8\xcf\x85\x88\xce\x80o\xcf@\xc9\x81\x00\x81\xfb\x00\x1b\x00n \xf8k\x81\x03\xe8\xa9\x08\xb5\x1f\xf8L\x01oP \xf8l\x8d\x04p\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x14\xcer)`\xc8\xce\x01o"\x02\xcb\x1f\xcc\xc9p\xfb\x00\xf8Lo\x10\x03\xf00\xf8F\xf2\xe0L\xf8Bn\xe3\x00\xd3\x1f\xf8DXou\xf8d\xd3\x1f\xd1\xdb<!\x8e\x1f#\xd0\xd3\x01\xfa@01\xc8\xcf\x87 \xce\x82\x10\x917\x0e\x00\xcf\x0b\x81\x01o"\x02\xcb\x1f\xcc\xc9p\x8e4\xf8D o\x13!o\x12\xf8IU\x02o\x11\xc8\xcf\x84\x80\xca\x00\xcf\x84@\xce\x01\xfa\x02\xf4\x00\x80j\xcf@\xf8Do\x15\xcf\x0b\x1f\x01o"\x02\xcb\x1f\xcc\xc9\xf8Do\x14\xe2\xfb\x00\xe3\x00\xf2\x00\x1f\x1e\x1d\x00(\xedD\xd0\xd3\xff\xd3?1\xf8CX\xc8\xcb\xff\xcb?\xce\xc9\xedT\x01Z \x81\x03\xe8\xb9\xf2\xe59\xf8Lo\x10\xa0\xb5\x1fp\x88o\x02\x01oP\xf8Lo\x11oQ0\xf8Dpor\x80Dotpoq\xf8d\xf8L#\x00@\xedD\xd0\xd3\xff\xd3?\xd3\x001\xd3\x1f\xd3\x7f\xd3\x1f\xd4Yo\x02\x01\xd1\xf8l\xf8k\xf8j\xf8c\xf8b\x00\n\xf8F\xf2\xe0L\x02\x10\xf4\xa4 \xf4\xbd\xf2\xc0N#"\x00\x14sol 0.66.0\x00\x00'
```

:::
