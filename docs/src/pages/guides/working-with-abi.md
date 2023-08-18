---
outline: deep
---

# Working with ABI

The Application Binary Interface (ABI) plays a fundamental role in interacting with smart contracts in Nekoton-Python. This section provides a comprehensive guide to the primary methods for working with the Contract ABI, Function ABI, and Event ABI, all of which are facilitated by `nekoton-python`.

The ABI is a specification that outlines the methods and structures of smart contracts, enabling interaction with them on the blockchain. It comprises:

- **Contract ABI**: This provides an exhaustive description of a smart contract, including its functions and events.
- **Function ABI**: This describes a function within a smart contract, detailing its name, inputs, and outputs.
- **Event ABI**: This outlines an event within a smart contract, specifying its name and inputs.

## Contract ABI

The Contract ABI is integral to interacting with smart contracts. It details the methods and structures that smart contracts employ, which are vital for interaction.

### Initialization

A `ContractAbi` object is initialized using its constructor, which accepts a string with a JSON ABI description as an argument. Here's an example of declaring the ABI of a contract:

:::details ABI Definition

```python
example_abi = ContractAbi("""{
    'ABI version': 2,
    version: '2.3',
    header: ['time'],
    functions: [
      {
        name: 'constructor',
        inputs: [
          { name: 'someParam', type: 'uint128' },
          { name: 'second', type: 'string' },
        ],
        outputs: [],
      },
      {
        name: 'getComplexState',
        inputs: [],
        outputs: [
          {
            components: [
              { name: 'first', type: 'uint32' },
              { name: 'second', type: 'string' },
            ],
            name: 'value0',
            type: 'tuple',
          },
        ],
      },
      {
        name: 'setVariable',
        inputs: [{ name: 'someParam', type: 'uint128' }],
        outputs: [{ name: 'value0', type: 'uint32' }],
      },
      {
        name: 'setVariableExternal',
        inputs: [{ name: 'someParam', type: 'uint128' }],
        outputs: [],
      },
      {
        name: 'getSecondElementWithPrefix',
        inputs: [{ name: 'prefix', type: 'string' }],
        outputs: [{ name: 'value0', type: 'string' }],
      },
      {
        name: 'computeSmth',
        inputs: [
          { name: 'answerId', type: 'uint32' },
          { name: 'offset', type: 'uint32' },
        ],
        outputs: [
          {
            components: [
              { name: 'first', type: 'uint32' },
              { name: 'second', type: 'string' },
            ],
            name: 'res',
            type: 'tuple',
          },
        ],
      },
      {
        name: 'simpleState',
        inputs: [],
        outputs: [{ name: 'simpleState', type: 'uint128' }],
      },
    ],
    data: [{ key: 1, name: 'nonce', type: 'uint32' }],
    events: [
      {
        name: 'StateChanged',
        inputs: [
          {
            components: [
              { name: 'first', type: 'uint32' },
              { name: 'second', type: 'string' },
            ],
            name: 'complexState',
            type: 'tuple',
          },
        ],
        outputs: [],
      },
    ],
    fields: [
      { name: '_pubkey', type: 'uint256' },
      { name: '_timestamp', type: 'uint64' },
      { name: '_constructorFlag', type: 'bool' },
      { name: 'nonce', type: 'uint32' },
      { name: 'simpleState', type: 'uint128' },
      {
        components: [
          { name: 'first', type: 'uint32' },
          { name: 'second', type: 'string' },
        ],
        name: 'complexState',
        type: 'tuple',
      },
    ],
}""")
```

:::

#### Reading from a File

The `ContractAbi` class provides a `from_file` method to read the ABI from a file. This method takes a file path as an argument and returns a `ContractAbi` object.

```python
abi = ContractAbi.from_file("/path/to/your/abi.json")
```

### Decoding Contract ABI

The `decode_init_data` method decodes initial contract data. It takes a `Cell` object as an argument and returns a tuple containing an optional `PublicKey` and a dictionary with initial data values.

```python
public_key, data = abi.decode_init_data(cell)
```

### Searching for Function ABI

The `get_function` method of the `ContractAbi` class searches for a function ABI by its name. It returns `FunctionAbi` objects, or `None` if no function with the specified name exists.

```python
function_abi = abi.get_function("computeSmth")
```

### Encoding Initial Contract Data

The `ContractAbi` class provides the `encode_init_data` method to encode initial contract data. It takes a dictionary with initial data values, an optional `PublicKey`, and an optional existing `Cell` object as arguments, and returns a `Cell` object.

```python
cell = abi.encode_init_data(data, public_key, existing_data)
```

## Function ABI

The `FunctionAbi` class facilitates interaction with the functions defined in the smart contract ABI.

### Calling ABI Functions

ABI functions can be invoked using the `call` method provided by the `FunctionAbi` class. This method takes two arguments: the current account state and a dictionary of input parameters. The account state can be obtained using the `getFullAccountState` method of the `TonClient` class.

#### Calling Simple Getters

Simple getters are functions that allow you to retrieve publicly visible data from the contract. They do not require user interaction and can be called without any parameters or with parameters, depending on the function definition in the ABI. Here's an example of how to call a simple getter:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("getComplexState")

# Call the function
result = function_abi.call(account_state, input={})

print(result)
print(result.output)

>> <ExecutionOutput exit_code=0, has_output=True>
>> {'value0': {'first': 42, 'second': 'test'}}
```

If the getter requires parameters, they can be provided in the `input` dictionary:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("getSecondElementWithPrefix")

# Call the function with parameters
result = function_abi.call(account_state, input={"prefix": "foo"})

print(result.output)

>> {'value0': 'footest'}
```

Note that the arguments must have the same type as described in the ABI, and they are merged into one object by `name`.

#### Calling Responsible Methods

Responsible methods are a special type of functions that can either be called via an internal message or locally as a getter via an external message. They differ from simple getters as they have an additional argument of type `uint32` which is usually called `answerId`.

When a responsible method is called on-chain, it returns the result in an outgoing internal message to the caller with `answerId` as a function id. When it is called locally, it behaves the same way as simple getters. Here's an example of how to call a responsible method:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("computeSmth")

# Call the function with parameters
result = function_abi.call(account_state, input={"offset": 999, "answerId": 42})
```

### Encoding External Messages & Input

#### External Messages

The `encode_external_message` and `encode_external_input` methods are used to prepare an external message for sending. External messages are used to call functions in smart contracts from off-chain applications. Here's an example of how to use these methods:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("setVariableExternal")

# Define the input parameters
input_params = {"someParam": 42}

# Define other necessary parameters
dst = "0:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
public_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
state_init = None
timeout = 0
clock = 0

# Encode the external message
external_message = function_abi.encode_external_message(dst, input_params, public_key, state_init, timeout, clock)

# Encode the external input
external_input = function_abi.encode_external_input(input_params, public_key, timeout, dst, clock)
```

In this example, we're preparing an external message to call the `setVariableExternal` function on the contract. The `input_params` dictionary contains the parameters for the function call.

#### Internal Messages

The `encode_internal_message` and `encode_internal_input` methods are used to prepare an internal message for sending. Internal messages are used for function calls between contracts on-chain. Here's an example of how to use these methods:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("setVariable")

# Define the input parameters
input_params = {"someParam": 1337}

# Define other necessary parameters
value = 1 * 10 ** 9  # 1 Native coin
bounce = True
dst = "0:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
src = "0:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"

# Encode the internal message
internal_message = function_abi.encode_internal_message(input_params, value, bounce, dst, src, state_init)

# Encode the internal input
internal_input = function_abi.encode_internal_input(input_params)
```

### Decoding Transactions as Function Calls

The `decode_transaction` method decodes a transaction as a function call. It takes a `Transaction` object as an argument and returns a `FunctionCall` object.

```python
function_call = function_abi.decode_transaction(transaction)
```

### Decoding Message Bodies as Input or Output

The `decode_input` and `decode_output` methods decode a message body as input or output. They take a `Cell` object and an optional boolean value as arguments, and return a dictionary with the decoded data.

```python
input_data = function_abi.decode_input(message_body, internal, allow_partial)
output_data = function_abi.decode_output(message_body, allow_partial)
```

## Event ABI

The `EventAbi` class is used to interact with the events defined in the smart contract ABI.

```json
{
  "name": "StateChanged",
  "inputs": [
    {
      "components": [
        { "name": "first", "type": "uint32" },
        { "name": "second", "type": "string" }
      ],
      "name": "complexState",
      "type": "tuple"
    }
  ],
  "outputs": []
}
```

### Searching for Event ABI

The `get_event` method of the `ContractAbi` class searches for an event ABI by its name. It returns `EventAbi` objects, or `None` if no event with the specified name exists.

```python
event_abi = abi.get_event("StateChanged")
```

### Decoding Event Data

The `decode_message` and `decode_message_body` methods decode event data from a message or a message body. They take a `Message` or `Cell` object as an argument respectively, and return a dictionary with the decoded data.

```python
event_data_from_message = event_abi.decode_message(message)
event_data_from_body = event_abi.decode_message_body(message_body)
```

## Working with Tokens

Tokens in Nekoton-Python are represented by the `Tokens` class. This class provides a convenient way to work with tokens.

### Creating Tokens

To create a `Tokens` object, you can use the constructor of the `Tokens` class. This constructor takes a decimal or integer value.

```python
from nekoton import Tokens

# Define the amount of tokens.
amount = 100

# Create a Tokens object.
tokens = nt.Tokens(amount)
print(tokens)

>> Tokens: 100
```

### Operations with Tokens

The `Tokens` class, as mentioned earlier, supports several arithmetic and comparison operations, making it flexible and convenient for various use cases.

#### **From Nano**

You can convert an amount in nano to a `Tokens` object using the statimethod `from_nano`.

```python
nano_amount = 1_000_000_000
tokens_from_nano = Tokens.from_nano(nano_amount)
print(tokens_from_nano)

>> Tokens: 1
```

#### **To Nano**

- `to_nano`: Convert the token amount back to its nano equivalent.

```python
tokens = nt.Tokens("0.001")
print(tokens.to_nano()) # 1000000
```

- Converting to integers or checking boolean value:

```python
tokens = nt.Tokens(1)
print(int(tokens))  # 1_000_000_000
print(bool(tokens)) # True for non-zero values
```

#### **Properties**

- `is_signed`: Helps determine if the token amount has a negative value.

```python
negative_tokens = nt.Tokens(-10)
print(negative_tokens.is_signed)  # True
```

- `is_zero`: Helps check if the token amount is zero.

```python
zero_tokens = nt.Tokens(0)
print(zero_tokens.is_zero)  # True
```

#### **Comparisons**

- `max`: Compare two `Tokens` objects and return the one with tmaximum value.

```python
tokens_a = nt.Tokens(50)
tokens_b = nt.Tokens(100)

print(tokens_a.max(tokens_b)) # 100
```

- `min`: Compare two `Tokens` objects and return the one with tminimum value.

```python
tokens_a = nt.Tokens(50)
tokens_b = nt.Tokens(100)

print(tokens_a.min(tokens_b)) # 50
```

#### **Arithmetic Operations**

The class supports standard arithmetic operations. You can addsubtract, multiply, and divide tokens, or get their absolute value.

```python
tokens_a = nt.Tokens(100)
tokens_b = nt.Tokens(50)

print(tokens_a + tokens_b)  # Tokens: 150
print(tokens_a - tokens_b)  # Tokens: 50
print(tokens_a * 2)         # Tokens: 200
print(tokens_a / 2)         # Tokens: 50
```

#### **Unary Operators**

These operators act on a single operand and return a result. Foinstance, you can get the positive or negative value of tokens.

```python
tokens = nt.Tokens(100)
print(+tokens)  # Tokens: 100
print(-tokens)  # Tokens: -100
```

#### **Comparisons**

You can directly compare two `Tokens` objects using standard comparison operators.

```python
tokens_a = nt.Tokens(100)
tokens_b = nt.Tokens(50)

print(tokens_a > tokens_b)   # True
print(tokens_a < tokens_b)   # False
print(tokens_a == tokens_b)  # False
```

#### **Absolute Value**

The `abs` function can be used to get the absolute value of the tokens.

```python
tokens = nt.Tokens(-100)
print(abs(tokens))  # Tokens: 100
```
