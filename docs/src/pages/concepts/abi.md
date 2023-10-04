---
outline: deep
---

# Application Binary Interface (ABI)

Application Binary Interface (ABI) is a critical component in the interaction between smart contracts and the outside world within the TVM blockchains. It is responsible for defining how functions in a contract are called, including how their data is encoded and decoded. The ABI is a scheme that specifies how to transform high-level data types into a binary representation in TVM cells and vice versa.

### ABI Structure

The ABI is described in a JSON file and includes the following components:

- **Version**: The version of the ABI.
- **Header**: Additional parameters of functions within the contract.
- **Functions**: The signatures of each interface function, including its name, input, and output parameters.
- **Events**: The events used in the contract.
- **Data**: Used for state init formation.
- **Fields**: The internal structure of the smart contract's data.

:::details ABI Example

```json
{
  "ABI version": 2,
  "version": "2.2",
  "header": ["time"],
  "functions": [
    {
      "name": "sendTransaction",
      "inputs": [
        { "name": "dest", "type": "address" },
        { "name": "value", "type": "uint128" },
        { "name": "bounce", "type": "bool" },
        { "name": "flags", "type": "uint8" },
        { "name": "payload", "type": "cell" }
      ],
      "outputs": []
    },
    {
      "name": "transferOwnership",
      "inputs": [{ "name": "newOwner", "type": "uint256" }],
      "outputs": []
    },
    {
      "name": "constructor",
      "inputs": [],
      "outputs": []
    },
    {
      "name": "owner",
      "inputs": [],
      "outputs": [{ "name": "owner", "type": "uint256" }]
    },
    {
      "name": "_randomNonce",
      "inputs": [],
      "outputs": [{ "name": "_randomNonce", "type": "uint256" }]
    }
  ],
  "data": [{ "key": 1, "name": "_randomNonce", "type": "uint256" }],
  "events": [
    {
      "name": "OwnershipTransferred",
      "inputs": [
        { "name": "previousOwner", "type": "uint256" },
        { "name": "newOwner", "type": "uint256" }
      ],
      "outputs": []
    }
  ],
  "fields": [
    { "name": "_pubkey", "type": "uint256" },
    { "name": "_timestamp", "type": "uint64" },
    { "name": "_constructorFlag", "type": "bool" },
    { "name": "owner", "type": "uint256" },
    { "name": "_randomNonce", "type": "uint256" }
  ]
}
```

:::

Each of these components is defined in a specific way, and they all contribute to the overall functionality of the ABI.

### Header

The header describes additional parameters of functions within the contract. Header-specific types are specified as strings with the type name. Other types are specified as function parameter types.

### Functions

Functions specify each interface function signature, including its name, input, and output parameters. Functions specified in the contract interface can be called from other contracts or from outside the blockchain via an ABI call.

### Events

Events specify the events used in the contract. An event is an external outbound message with ABI-encoded parameters in the body.

### Data

Data covers the contract's global public variables. It's typically used when deploying multiple identical contracts with the same deployer keys. It affects the contract address, and thus varying data results in unique addresses for identical contracts.

### Fields

Fields describe the internal structure of the smart contracts data. They include contract state variables and some internal contract-specific hidden variables. They are listed in the order in the contract persistent data.

### ABI Types

ABI supports various data types, including integers, booleans, tuples, maps, cells, addresses, bytes, strings, optional types, and arrays. Each type has its specific usage, value examples, maximum bit size, and maximum reference size.

### ABI in Action

When a function call is made, the ABI encodes the function name and arguments into a format that can be recognized and processed by the smart contract. When the function execution is complete, the ABI then decodes the data returned by the function into a format that can be understood outside the blockchain.

This process of encoding and decoding allows for seamless interaction between the blockchain and external entities, making the ABI a crucial part of any blockchain system.
