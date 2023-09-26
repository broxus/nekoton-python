---
outline: [2, 4]
---

# Working with Transport

The `nekoton-python` library offers interaction with the TVM (TON Virtual Machine) blockchain network through its transport mechanisms. Specifically, there are two primary transport classes available:

- `GqlTransport`: Implements transport using GraphQL.
- `JrpcTransport`: Implements transport using JRPC.

Both of these classes are subclasses of the base `Transport` class, providing specialized methods for their respective transport protocols.

## Initialization

### GqlTransport

To initialize a `GqlTransport` object, you need to provide a list of endpoints. Optionally, you can also provide a `Clock` object and a boolean indicating whether the connection is with a local node.

```python
transport = nt.GqlTransport(
    endpoints=["https://devnet.evercloud.dev/89a3b8f46a484f2ea3bdd364ddaee3a3"]
)

print(transport)
```

##### Result

```python
<builtins.GqlTransport object at 0x104f4c2a0>
```

### JrpcTransport

To initialize a `JrpcTransport` object, you need to provide a JRPC endpoint. Optionally, you can also provide a `Clock` object.

```python
transport = nt.JrpcTransport(endpoint="https://jrpc.everwallet.net")

print(transport)
```

##### Result

```python
<builtins.JrpcTransport object at 0x104e3b0d0>
```

## Checking Connection

You can check the connection by calling the `check_connection` method. This method does not take any parameters.

```python
await transport.check_connection()
```

## Blockchain Config

The `nekoton-python` library provides an easy way to fetch the latest blockchain configuration and work with it. The blockchain configuration provides crucial information about the blockchain parameters.

### Fetching Blockchain Config

You can fetch the latest blockchain configuration by calling the `get_blockchain_config` method. This method does not take any parameters.

```python
config = await transport.get_blockchain_config()

print(config)
```

##### Result

```python
<BlockchainConfig global_id=42 capabilities=0x00000000880737ae, global_version=0x36>
```

The `get_blockchain_config` method returns a `BlockchainConfig` object, which provides a partially parsed blockchain configuration. This object contains various properties that offer insights into different blockchain parameters.

### Accessing Properties

Here's how you can access and print the properties of the `BlockchainConfig`:

```python
config = await transport.get_blockchain_config()

# Accessing and printing properties
print("Capabilities:", config.capabilities)
print("Global Version:", config.global_version)
print("Config Address:", config.config_address)
print("Elector Address:", config.elector_address)
print("Minter Address:", config.minter_address)
print("Fee Collector Address:", config.fee_collector_address)
```

##### Result

```
Capabilities: 2281903790
Global Version: 32
Config Address: -1:5555555555555555555555555555555555555555555555555555555555555555
Elector Address: -1:3333333333333333333333333333333333333333333333333333333333333333
Minter Address: -1:0000000000000000000000000000000000000000000000000000000000000000
Fee Collector Address: -1:3333333333333333333333333333333333333333333333333333333333333333
```

### Checking Parameters

This method returns `True` if the config contains the specified parameter.

```python
param_exists = config.contains_param(0)

print(param_exists) # True
```

### Getting Parameters

Returns the corresponding config value as a cell.

```python
raw_param = config.get_raw_param(0)

print(raw_param)
```

##### Result

```
<Cell repr_hash='e6025a4b06943baa939e0497bf474bf8b946938d5a4d70bd2fae2b7d481b3cb9', bits=256, refs=0>
```

## Fetching Signature ID

You can fetch the signature ID for the selected network using the `get_signature_id` method. This method does not take any parameters.

```python
signature_id = await transport.get_signature_id()

print(signature_id)
```

##### Result

```python
None
```

## Account Interaction

### Account State

You can fetch the state of an account by calling the `get_account_state` method and providing an `Address` object.

```python
example_address = nt.Address("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")

account_state = await transport.get_account_state(example_address)

print(account_state)
```

##### Result

```python
<AccountState balance=0.922934241, Active>
```

### Accounts by Code Hash

You can fetch a list of addresses of accounts with the specified code hash using the `get_accounts_by_code_hash` method. This method requires a `code_hash` as a parameter, and optionally a `continuation` address from the previous batch and a `limit` for the max number of items in the response.

```python
code_hash = bytes.fromhex("1583b2bc6a3b8acc01ac653e2255407a140df286141e8bb77dd97419e6258554")

addresses = await transport.get_accounts_by_code_hash(
    code_hash=code_hash,
    continuation=None,
    limit=5
)

for address in addresses:
    print(address)
```

##### Result

```python
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e
0:31eea318369c9019f9c65aa6aa2c3663ac8b5b853283f5af4e4b09deaefea1d6
0:4997f134bfa9d659ea2095223d59253931ca26a71460108ef5e19b7327570cc4
```

### Account States Iterator

To iterate over the account states, you can use the `account_states` method. This method requires an `address` as a parameter. The iterator will always have at least one iteration with the current state. If the state is `None`, it means the account does not exist.

Here's how you can use it:

```python
async with transport.account_states(example_address) as states:
    async for state in states:
        if state is not None:
            print(state)
        else:
            print("Account does not exist.")
        break  # You can remove this line if you want to iterate over all states
```

##### Result

```python
<AccountState balance=0.922934241, Active>
```

### Account Transactions Iterator

You can get an asynchronous iterator over the account transactions using the `account_transactions` method. This method requires an `address` as a parameter.

```python
async with transport.account_transactions(config.elector_address) as batches:
    async for batch, batch_info in batches:
        print(batch)
        # Additional processing can be done here if needed
        break  # You can remove this line if you want to iterate over all batche
```

##### Result

```python
[<Transaction hash='217ebed261828c71298ef2fe46a8ea52dc71335653c4760ddcd4de9446e561e0', Tick>, <Transaction hash='dfb8247a1cad5b1fc68210b57ba5ae55b3640da5fad4998209a60e33462a8b9a', Ordinary>, <Transaction hash='faf56e6d848a5a9a6625a26a547d1bc7aeec1f9c20fd90e666043d872a439526', Tick>, <Transaction hash='44d9239ee3a636b5e3c1b28522135df75e6a61ef3b6ae1382664478df4604ea6', Ordinary>]
```

## Transactions

### Sending External Message

You can send an external message to the network and wait for the transaction using the `send_external_message` method. This method requires a `SignedExternalMessage` as a parameter.

```python
transaction = await transport.send_external_message(signed_external_message)

print(transaction)
```

##### Result

```python
<Transaction hash='74a5de834a2b7b043c4d993c78cfb15fdb46c3f2346e2aee53834ab0c1bd28d1', Ordinary>
```

### Fetching Transaction

You can fetch a transaction by calling the `get_transaction` method and providing a transaction hash.

```python
tx = await transport.get_transaction(
    bytes.fromhex(
        "2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552"
    )
)

print(tx)
```

##### Result

```python
<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>
```

#### Destination Transaction

You can search for a transaction by the hash of the incoming message using the `get_dst_transaction` method. This method requires a `message_hash` as a parameter, which can be a hash of the incoming message or the message itself.

```python
transaction = await transport.get_dst_transaction(message_hash=tx.in_msg_hash)

print(transaction)
```

##### Result

```python
<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>
```

#### Transactions Batch

You can fetch a transactions batch for a specific account using the `get_transactions` method. This method requires an `address` as a parameter, and optionally a `lt` (logical time of the latest transaction) and a `limit` (max number of items in the response).

```python
transactions = await transport.get_transactions(
    address=example_address,
    lt=None,
    limit=5
)

for transaction in transactions:
    print(transaction)
```

##### Result

```python
<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>
<Transaction hash='1ee4e54aae183b683d65149a4d34a822942b49e391fe33b7b8c80f7b0898f7b8', Ordinary>
<Transaction hash='2316322ca16010560c4146180415366c6a3a64ce62338866bd4f198cfeb660eb', Ordinary>
<Transaction hash='b6d560ffab7466e2f3cb4fc9e89130d69843a453cb1eaa8693ea430ba3abc953', Ordinary>
<Transaction hash='571e84432d76e6f297a4a540147ebfd17992c7e1d7da867daa6f1e82c3a2fd9e', Ordinary>
```

### Transactions Tree Iterator

You can get an asynchronous iterator over the transactions tree using the `trace_transaction` method. This method requires a `transaction_hash` as a parameter, which can be a hash of the root transaction or the root transaction itself, and optionally a `yield_root` boolean to specify whether to emit the root transaction.

```python
async for transaction in transport.trace_transaction(transaction_hash=tx.hash, yield_root=True):
    print(transaction)
```

##### Result

```python
<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>
<Transaction hash='c9d0ecc9ebcf2ebdd077eae5168249f477bacdb3105b86145f0521291131668a', Ordinary>
```

## GqlTransport Queries

### Transactions

You can run a transactions GQL query using the `query_transactions` method. This method requires a `filter` as a parameter, and optionally an `order_by` and a `limit`.

```python
transactions = await transport.query_transactions(
    [
        nt.gql.tx.AccountAddr() == example_address,
        nt.gql.tx.TrType() == nt.TransactionType.Ordinary,
    ],
    order_by=[
        nt.gql.tx.Lt().desc(),
    ],
    limit=5,
)

print(transactions)
```

##### Result

```python
[<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>, <Transaction hash='1ee4e54aae183b683d65149a4d34a822942b49e391fe33b7b8c80f7b0898f7b8', Ordinary>, <Transaction hash='2316322ca16010560c4146180415366c6a3a64ce62338866bd4f198cfeb660eb', Ordinary>, <Transaction hash='b6d560ffab7466e2f3cb4fc9e89130d69843a453cb1eaa8693ea430ba3abc953', Ordinary>, <Transaction hash='571e84432d76e6f297a4a540147ebfd17992c7e1d7da867daa6f1e82c3a2fd9e', Ordinary>]
```

### Messages

You can run a messages GQL query using the `query_messages` method. This method requires a `filter` as a parameter, and optionally an `order_by` and a `limit`.

```python
messages = await transport.query_messages(
    nt.gql.or_(
        [
            nt.gql.msg.Src() == example_address,
            nt.gql.msg.Dst() == example_address,
        ]
    ),
    order_by=[
        nt.gql.msg.CreatedLt().desc(),
    ],
    limit=5,
)

print(messages)
```

##### Result

```python
[<Message hash='11b73e293fcbf34b2349dcbd0997520261a20325c72b03eee72771c60099ecf8', Internal>, <Message hash='c22a995050237f4e0fc88a25e187d2ede99199d3e6a7dc9ef8ac23bf8b2ac41e', ExternalOut>, <Message hash='44c4fe9d74053964b795c9be6af384a670c03dd6b90bb0b0b3b6b0e48c0304c5', Internal>, <Message hash='3a6054696fcb3c0d10e18d2ddca4d545e9e35b6d6b647992535e8f5594698630', ExternalOut>, <Message hash='a2a99a9f53031729ccb1f9b9a1c96ff23d855b3974c3ac44d034582f72925e62', ExternalOut>]
```

### Accounts

You can run an accounts GQL query using the `query_accounts` method. This method requires a `filter` as a parameter, and optionally an `order_by` and a `limit`.

```python
accounts = await transport.query_accounts(
    nt.gql.or_(
        [
            nt.gql.acc.Id() == example_address,
            nt.gql.acc.CodeHash() == code_hash,
        ]
    ),
    order_by=[nt.gql.acc.LastTransLt().asc()],
)
for addr, state in accounts:
    print(addr, state)
```

##### Result

```python
0:4997f134bfa9d659ea2095223d59253931ca26a71460108ef5e19b7327570cc4 <AccountState balance=0.987386499, Active>
0:31eea318369c9019f9c65aa6aa2c3663ac8b5b853283f5af4e4b09deaefea1d6 <AccountState balance=0.987386499, Active>
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e <AccountState balance=0.922934241, Active>
```
