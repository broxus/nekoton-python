---
outline: [2, 4]
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

ABI functions can be invoked using the `call` method provided by the `FunctionAbi` class.
This method takes two arguments: the current account state and a dictionary of input parameters.
The account state can be obtained using the `get_account_state` method of the `Transport` class.

#### Calling Simple Getters

Simple getters are functions that allow you to retrieve publicly visible data from the contract.
They do not require user interaction and can be called without any parameters or with parameters, depending on the function definition in the ABI. Here's an example of how to call a simple getter:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("getComplexState")

# Call the function
result = function_abi.call(account_state, input={})

print(result)
print(result.output)
```

##### Result

```python
<ExecutionOutput exit_code=0, has_output=True>
{'value0': {'first': 42, 'second': 'test'}}
```

If the getter requires parameters, they can be provided in the `input` dictionary:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("getSecondElementWithPrefix")

# Call the function with parameters
result = function_abi.call(account_state, input={"prefix": "foo"})

print(result.output)
```

##### Result

```python
{'value0': 'foo'}
```

:::info
Note that the arguments must have the same type as described in the ABI, and they are merged into one object by `name`.
:::

#### Calling Responsible Methods

Responsible methods are a special type of functions that can either be called via an internal message or locally as a getter via an external message. They differ from simple getters as they have an additional argument of type `uint32` which is usually called `answerId`.

When a responsible method is called on-chain, it returns the result in an outgoing internal message to the caller with `answerId` as a function id. When it is called locally, it behaves the same way as simple getters. Here's an example of how to call a responsible method:

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("computeSmth")

# Call the function with parameters
result = function_abi.call(account_state, input={"offset": 999, "answerId": 42})

print(result)
print(result.output)
```

##### Result

```python
{'res': {'first': 42, 'second': 'test'}}
```

### Encoding Messages

#### External Messages

The `encode_external_message` method is utilized to prepare an external message for transmission. External messages facilitate the invocation of functions in smart contracts from off-chain applications.

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("setVariableExternal")

# Define the input parameters
input_params = {"someParam": 66}

# Define other necessary parameters
dst = nt.Address("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")
timeout = 0
clock = nt.Clock()
state_init = nt.StateInit.decode(base64, encoding="base64")

# Encode the unsigned external message
ext_unsigned_message = function_abi.encode_external_message(
  dst,
  input_params,
  public_key,
  state_init,
  timeout,
  clock)

print(ext_unsigned_message)
```

##### Result

```python
<UnsignedExternalMessage hash='c07407d60d09753fc41d32b1124264df0d6033100bd36fd7c77211e47297f38e', expire_at=1694215298>

```

#### Internal Messages

The `encode_internal_message` method is employed to prepare an internal message for transmission. Internal messages are designated for function calls between on-chain contracts.

```python
# Initialize the ABI and get the function
function_abi = abi.get_function("setVariable")

# Define the input parameters
input_params = {"someParam": 1337}

# Define other necessary parameters
value = nt.Tokens(1 * 10**9) # 1 Native coin
bounce = True
dst = nt.Address("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")
account = nt.Address("your_account_address")

# Encode the internal message
internal_message = function_abi.encode_internal_message(
  input_params,
  value,
  bounce,
  dst,
  account,
  state_init)

print(internal_message)
```

##### Result

```python
<Message hash='b25915ce08b5ba4ade9323a0011d155f6cfe2bc9439d923d67d9dd0501113f03', Internal>
```

### Encoding Message Body

#### External Message Body

The `encode_external_body` method is used to prepare the body of an external message. This body contains the necessary data for the external message to be processed by the smart contract.

```python
# Encode the external message body
external_body = function_abi.encode_external_body(input_params, public_key, timeout, dst, clock)

print(external_body)
```

##### Result

```python
<UnsignedBody hash='a14be6da67c3395f57c48eabecd443549316933e94cbca929f6224c16e7dd7aa', expire_at=1694215298>
```

#### Internal Message Bodies

The `encode_internal_body` method is used to prepare the body of an internal message. This body contains the necessary data for the internal message to be processed by the smart contract.

```python
# Define the input parameters
input_params = {"someParam": 1337}

# Encode the internal input
internal_input = function_abi.encode_internal_input(input_params)

print(internal_input)
```

##### Result

```python
 <Cell repr_hash='13e1b0dc2a0f092c40a99ccbdd3022d8660c818651aaecff9932333cfb09ca36', bits=160, refs=0>
```

### Decoding Transactions as Function Calls

The `decode_transaction` method decodes a transaction as a function call.
It takes a `Transaction` object as an argument and returns a `FunctionCall` object.

:::info
Please note that we have not yet covered the Transport aspect.

For information on how to set it up and its various functions, please refer to [Working with Transport](working-with-transport.md).
:::

```python
set_variable_tx = await transport.get_transaction(
    bytes.fromhex(
        "b0e21d98e2536491a9cd4b56a72a38a8a41e7c25cd7163c95aba186f54700ec1"
    )
)

function_call = function_abi.decode_transaction(set_variable_tx)

print(function_call)
print(function_call.input, function_call.output)
```

##### Result

```python
<builtins.FunctionCall object at 0x101d23600>
{'someParam': 42} {}
```

### Decoding Message Bodies as Input or Output

The `decode_input` and `decode_output` methods decode a message body as input or output.
They take a `Cell` object and an optional boolean value as arguments, and return a dictionary with the decoded data.

```python

# Setting up the ABI for a specific function.
function_abi = abi.get_function("setVariable")

# Decoding a message body as an input using `decode_input` method from `function_abi` object.
message_body_cell = nt.Cell.decode("te6ccgEBAQEAFgAAKDja0OwAAAAAAAAAAAAAAAAAAAU5")

input_data = function_abi.decode_input(message_body_cell, True)

print(input_data)
```

##### Result

```python
{'someParam': 42}
```

```python
# Setting up the ABI for a specific function.
function_abi = abi.get_function("setVariable")

# Decoding a message body as an output using `decode_output` method from `function_abi` object.
message_body_cell = nt.Cell.decode("te6ccgEBAQEAFgAAKDja0OwAAAAAAAAAAAAAAAAAAAU5")

output_data = function_abi.decode_output(message_body_cell)

print(output_data) # {}
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

The `get_event` method of the `ContractAbi` class searches for an event ABI by its name.
It returns `EventAbi` objects, or `None` if no event with the specified name exists.

```python
# Searching for an event ABI by its name using `get_event` method of `abi` object.
event_abi = abi.get_event("StateChanged")

print(event_abi)
```

##### Result

```python
<EventAbi name='StateChanged', id=0x5339c8a5>
```

### Decoding Event Data

The `decode_message` and `decode_message_body` methods decode event data from a message or a message body. They take a `Message` or `Cell` object as an argument respectively, and return a dictionary with the decoded data.

```python
# Firstly, we retrieve the ABI for a specific event by calling the `get_event` method.
event_abi = abi.get_event("StateChanged")

# Here, we decode two different message bodies (as 'Cell' objects) to extract the data they contain.
message_body = nt.Cell.decode("te6ccgEBAgEAEQABEFM5yKUAAAFRAQAIdGVzdA==")
message_boc = nt.Cell.decode(
    "te6ccgEBAgEAQAABbeAANiAkzF2lN65/Iyyc8fGrF2q58T+8gxqtjU8Oew+t0XAAACbh/GVjCMn3j2wpnORSgAAAqMABAAh0ZXN0"
)

# Next, we create a 'Message' object from one of the previously decoded 'Cell' objects.
message = nt.Message.from_cell(message_boc)

# Using the `event_abi` object, we call `decode_message` and `decode_message_body` methods
# to decode the event data from the message and the message body, respectively.
event_data_from_message = event_abi.decode_message(message)
event_data_from_body = event_abi.decode_message_body(message_body)


print(event_data_from_message)
print(event_data_from_body)
```

##### Result

```python
{'complexState': {'first': 337, 'second': 'test'}}
{'complexState': {'first': 337, 'second': 'test'}}
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
```

##### Result

```python
Tokens: 100
```

### Operations with Tokens

The `Tokens` class, as mentioned earlier, supports several arithmetic and comparison operations, making it flexible and convenient for various use cases.

#### **From Nano**

You can convert an amount in nano to a `Tokens` object using the statimethod `from_nano`.

```python
nano_amount = 1_000_000_000
tokens_from_nano = Tokens.from_nano(nano_amount)

print(tokens_from_nano)
```

##### Result

```python
Tokens: 1
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
