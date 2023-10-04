---
outline: [2, 4]
---

# Working with Accounts

Accounts form the basic unit of information storage in TVM-compatible blockchains. They are essentially smart contracts, with each one uniquely identified by a full address. This guide will walk you through the basic methods for interacting with accounts provided by the given source code.

## Account State

The state of an account is a crucial aspect as it determines the operation mode of the account while a message is being executed for that account. The account state can be obtained using a transport and provides several useful properties.

:::tip
Before delving deeper, be aware that this guide provides a brief insight into working with a transport, essential for understanding how to retrieve the `AccountState`. A comprehensive guide on working with transport can be found in the subsequent section, [**Working with Transport guide**](./working-with-transport.md).
:::

### Reading Account State

The account state can be obtained by using the `get_account_state` method of a transport. For example:

```python
# Initialize a transport
transport = nt.GqlTransport(
    endpoints=["https://devnet.evercloud.dev/89a3b8f46a484f2ea3bdd364ddaee3a3"]
)

# Get the account state
account_state = await transport.get_account_state(
    nt.Address("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")
)

print(account_state)
```

##### Result

```python
<AccountState balance=0.922934241, Active>
```

In this case, the `account_state` is an instance of `AccountState`, which is obtained from the blockchain.

### Account Status

The account status can be one of three types: `Nonexisten`, `Uninit`, `Active`, or `Frozen`. The status can be accessed as follows:

```python
status = account_state.status

print(status) # Active
```

## Interacting with Addresses

Each account is uniquely identified by its address. You can initialize and validate addresses using the `Address` class.

### Initializing & Validating

An address can be initialized using the `Address` class. For instance:

```python
address = nt.Address("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")
```

To validate an address, you can use the `validate` method:

```python
is_valid = nt.Address.validate("0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e")

print(is_valid) # True
```

### Address Parts

The parts of an address can be accessed using the `workchain` and `account` properties:

```python
workchain = address.workchain
account = address.account

print(workchain)
print(account)
```

##### Result

```python
0
b"\x06\xc4\x04\x99\x8b\xb4\xa6\xf5\xcf\xe4e\x93\x9e>5b\xedW>'\xf7\x90cU\xb1\xa9\xe1\xcfa\xf5\xba."
```

#### From Parts

An address can also be initialized from its parts using the `from_parts` method:

```python
address = nt.Address.from_parts(workchain_id=0, account=account)

print(address)
```

##### Result

```python
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e
```
