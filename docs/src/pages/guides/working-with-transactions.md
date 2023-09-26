---
outline: [2, 3]
---

# Working with Transactions

A transaction stands as the testament to the execution of a contract within the framework of TVM (TON Virtual Machine) compatible blockchains.

Typically, a single incoming message, whether external or internal, gives rise to a transaction. This transaction, in turn, has the potential to produce multiple outgoing messages, which can also be either external or internal.

It's essential to understand that transactions can either reach successful completion or get aborted.

### Preliminary Step

Before diving into the specifics of transactions, we need a transaction instance for our examples. For the purpose of this guide, we'll retrieve a transaction using the `get_transaction` method from the transport and assign it to the variable `tx`:

```python
tx = await transport.get_transaction(
  bytes.fromHex("2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552")
)

print(tx)
```

#### Result

```python
<Transaction hash='2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552', Ordinary>
```

:::tip
For a detailed guide on working with the transport, including sending transactions, please refer to [**`Working with Transport`**](./working-with-transport.md).

For a visual representation of the transaction, you can follow this [**`link`**](https://testnet.everscan.io/transactions/2a7c997e25c5849460057e0e066525647d0b0f657195c81d5f7ffa522ab1d552) which leads to the blockchain explorer.
:::

## Transaction Types

In the TVM-compatible blockchains, there are three types of transactions:

- Ordinary Transactions: These are the regular transactions that occur between accounts.
- Tick Transactions: These are special transactions that occur at the beginning of a block, without an incoming message.
- Tock Transactions: These are special transactions that occur at the end of a block, also without an incoming message.

These transaction types are represented by the `TransactionType` enum in the `nekoton-python` library.

### Checking Transaction Types

The `TransactionType` class provides a property called `type` to determine the nature of a transaction. Here's how you can use it:

```python
print(tx.type) # Ordinary
```

## Transaction Phases

Each transaction in a TVM-compatible blockchain consists of up to five phases: the Storage Phase, the Credit Phase, the Compute Phase, the Action Phase, and the Bounce Phase. Each of these phases is represented by a separate class in the `nekoton-python` library.

### Storage Phase

The `TransactionStoragePhase` class represents the storage phase of a transaction. In this phase, the storage fees accrued by the contract due to the occupation of some space in the chain state are calculated.

You can access the amount of collected storage fees and the status change during this phase through the `storage_fees_collected`, `storage_fees_due` and `status_change` properties, respectively.

```python
storage_phase = tx.storage_phase

print(f"Storage fees collected: {storage_phase.storage_fees_collected}")
print(f"Storage fees due: {storage_phase.storage_fees_due}")
print(f"Status change: {storage_phase.status_change}")
```

#### Result

```python
Storage fees collected: 0.002573887
Storage fees due: None
Status change: Unchanged
```

### Credit Phase

The `TransactionCreditPhase` class represents the credit phase of a transaction. In this phase, the balance of the contract with respect to a possible incoming message value and the collected storage fee are calculated. You can access the amount of collected due fees and the increased balance through the `due_fees_collected` and `credit` properties, respectively.

```python
credit_phase = tx.credit_phase

print(f"Due fees collected: {credit_phase.due_fees_collected}")
print(f"Credit: {credit_phase.credit}")
```

#### Result

```python
Due fees collected: None
Credit: 1
```

### Compute Phase

The `TransactionComputePhase` class encapsulates the compute phase of a transaction. During this phase, the TON Virtual Machine (TVM) runs, producing a result that aggregates several elements:

- **Exit Code:** `exit_code`
- **Actions:** Serialized list of `actions`
- **Gas Details:** `gas_details`
- **New Storage:** `new_storage`

This phase also offers insights into:

- **Success:** Indicates if the compute phase was successful (`success`).
- **Message State Used:** Specifies if the message state was utilized (`msg_state_used`).
- **Account State:** Whether the account is activated (`account_activated`).
- **Gas Fees:** The total gas fees incurred (`gas_fees`).
- **Gas Used:** The amount of gas consumed (`gas_used`).
- **Gas Limit:** The maximum gas that can be used (`gas_limit`).
- **Gas Credit:** The gas credit available (`gas_credit`).
- **Transaction Mode:** The mode of the transaction (`mode`).
- **Exit Argument:** Additional information or context for the exit code (`exit_arg`).
- **VM Steps:** Number of steps taken by the VM (`vm_steps`).
- **VM State Hashes:** Initial (`vm_init_state_hash`) and final (`vm_final_state_hash`) state hashes of the VM.

```python
compute_phase = tx.compute_phase

print(f"Success: {compute_phase.success}")
print(f"Message state used: {compute_phase.msg_state_used}")
print(f"Account activated: {compute_phase.account_activated}")
print(f"Gas fees: {compute_phase.gas_fees}")
print(f"Gas used: {compute_phase.gas_used}")
print(f"Gas limit: {compute_phase.gas_limit}")
print(f"Gas credit: {compute_phase.gas_credit}")
print(f"Mode: {compute_phase.mode}")
print(f"Exit code: {compute_phase.exit_code}")
print(f"Exit argument: {compute_phase.exit_arg}")
print(f"VM steps: {compute_phase.vm_steps}")
print(f"VM initial state hash: {compute_phase.vm_init_state_hash}")
print(f"VM final state hash: {compute_phase.vm_final_state_hash}")
```

#### Result

```python
Success: True
Message state used: False
Account activated: False
Gas fees: 0.006042
Gas used: 6042
Gas limit: 0
Gas credit: 10000
Mode: 0
Exit code: 0
Exit argument: None
VM steps: 143
VM initial state hash: b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
VM final state hash: b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
```

### Action Phase

The `TransactionActionPhase` class handles the action phase of a transaction. During this phase, output messages from the Compute Phase are dispatched. The main actions include:

- Internal Outbound Messages
- Event Messages
- RawReserve
- SetCode

### Key Properties

Key properties of the action phase include:

- **Success Status:** Indicates whether the action phase was successful.
- **Valid:** Specifies if the action phase is valid.
- **No Funds:** Indicates if there are insufficient funds for the action.
- **Status Change:** Reflects any status changes during the action phase.
- **Total Forward Fees:** The total fees forwarded during this phase.
- **Total Action Fees:** The total fees associated with the actions.
- **Result Code:** The resulting code after the action phase.
- **Result Argument:** Any arguments related to the result code.
- **Total Actions:** The total number of actions dispatched during this phase.
- **Special Actions:** The number of unique or special actions taken.
- **Skipped Actions:** Actions that were skipped during this phase.
- **Messages Created:** The number of output messages created during the Compute Phase that are dispatched in this phase.
- **Action List Hash:** The hash of the list of actions.

```python
action_phase = tx.action_phase

print(f"Success: {action_phase.success}")
print(f"Valid: {action_phase.valid}")
print(f"No funds: {action_phase.no_funds}")
print(f"Status change: {action_phase.status_change}")
print(f"Total forward fees: {action_phase.total_fwd_fees}")
print(f"Total action fees: {action_phase.total_action_fees}")
print(f"Result code: {action_phase.result_code}")
print(f"Result argument: {action_phase.result_arg}")
print(f"Total actions: {action_phase.total_actions}")
print(f"Special actions: {action_phase.special_actions}")
print(f"Skipped actions: {action_phase.skipped_actions}")
print(f"Messages created: {action_phase.messages_created}")
print(f"Action list hash: {action_phase.action_list_hash}")
```

#### Result

```python
Success: True
Valid: True
No funds: False
Status change: Unchanged
Total forward fees: 0.000566
Total action fees: 0.000566
Result code: 0
Result argument: None
Total actions: 1
Special actions: 0
Skipped actions: 0
Messages created: 1
Action list hash: b']s\x17`\xee\xd8\x89\xd7=v(\x8d\xd5i\xd8\xb3,\xe6\xf6N"\xdc\xbb\xf3\x8e\x1ai\xdf\xbf-F\r'
```

:::warning Caution
A maximum of 255 actions can be dispatched during this phase. Exceeding this limit aborts the transaction.
:::

### Bounce Phase

The `TransactionBouncePhase` class represents the bounce phase of a transaction. If the compute phase failed (it returned exit_code >= 2), in this phase, a bounce message is formed for transactions initiated by an incoming message. You can access the message fees and forward fees through the `msg_fees` and `fwd_fees` properties, respectively.

```python
bounce_phase = tx.bounce_phase

print(f"Message fees: {bounce_phase.msg_fees}")
print(f"Forward fees: {bounce_phase.fwd_fees}")
```

#### Result

```python
Message fees: 0.000000000
Forward fees: 0.000000000
```

## Transaction Executor

The `TransactionExecutor` class in the `nekoton-python` is a local transaction executor. It allows you to simulate the execution of a transaction in a local environment, which can be highly useful for testing and debugging purposes.

This class requires a `BlockchainConfig` object as a parameter during initialization.
This object contains the blockchain's configuration used during the execution of the transaction.

It can also optionally take a `Clock` object to modify the timestamp used during execution and a boolean value for `check_signature` to determine whether to check for valid signatures.

### Initializing

To initialize a `TransactionExecutor`, you need to provide a `BlockchainConfig`:

```python
# We can use the transport to get the blockchain config
config = await transport.get_blockchain_config()

executor = nt.TransactionExecutor(config, check_signature=False)

print(executor)
```

#### Result

```python
<TransactionExecutor check_signature=False>
```

:::tip Note
For a detailed understanding of the `BlockchainConfig` and how to retrieve it using the transport, please refer to our comprehensive guide on [**`Working with Transport`**](./working-with-transport.md#blockchain-config).
:::

### Executing Transaction

Once initialized, you can use the `execute` method to execute a message on an account state. This method takes a `Message` object and optionally an `AccountState` object. If no `AccountState` is provided, it assumes that the account does not exist.

#### External Message

Here's an example of how to use the `execute` method with an external message:

```python
tx, new_state = executor.execute(
    signed_external_message, account)

print(tx)
print(new_state)
```

##### Result

```python
<Transaction hash='bdfafac4450b2689886eb8c5c13cbed3ad18493c04907dfffb21d5ec2c681770', Ordinary>
<AccountState balance=0.907201392, Active>
```

:::tip Note
In the example above, we're using a message that was created in our [**Guide on Working with Messages**](./working-with-messages.md#creating-a-message-without-a-signature).
:::

#### Internal Message

You can also execute internal messages in a similar manner. Instead of passing a `Message` object, you directly pass the cell containing the message:

```python
tx, new_state = executor.execute(unsigned_internal_message, account_state)

print(tx)
print(new_state)
```

##### Result

```python
<Transaction hash='7abd9c98bdf9da2f5060f7610f030ebeb8f2b7b363b865103b696917f508f0dc', Ordinary>
<AccountState balance=0.921467796, Active>
```

### Context with Clock

If you want to simulate a transaction at a specific time, you can provide a `Clock` object during the initialization of the `TransactionExecutor`. This object will modify the timestamp used during the execution of the transaction.

```python
clock = nt.Clock(1337)
executor = nt.TransactionExecutor(config, clock=clock)
```

### Output

The `execute` method returns a tuple containing the executed transaction and the new state of the account.
If the transaction is successfully executed, the `aborted` attribute of the transaction will be `False`, and the `exit_code` of the `compute_phase` will be `0`. If the account state changes as a result of the transaction, the new state will be returned. Otherwise, `None` will be returned.

Here's how to check if a transaction was successful:

```python
print("Transaction aborted:", tx.aborted)
print("Exit code of Compute Phase:", tx.compute_phase.exit_code)

print(new_state)

```

#### Result

```python
Transaction aborted: False
Exit code of Compute Phase: 0

<AccountState balance=0.907993575, Active>
```

### Transaction Tree

The `TransactionTree` class offers a structured representation of a series of interconnected transactions, facilitating easy traversal and analysis. This class is instrumental for comprehending the relationships between transactions, especially when a single transaction might lead to multiple subsequent ones.

#### Decoding from Bytes

The `from_bytes` method allows for the direct decoding of a transaction tree from raw bytes:

```python
tree = TransactionTree.from_bytes(raw_byte_data)
```

This method is particularly beneficial when working with binary transaction data.

#### Decoding from Encoded BOC

With the `decode` method, you can decode a transaction tree from a `Base64` or `Hex` encoded BOC:

```python
base64_string = "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"


tree = nt.TransactionTree.decode(base64_string)

print(tree)
```

#### Result

```python
<builtins.TransactionTree object at 0x105014070>
```

This functionality is crucial when handling transactions serialized in specific encoding formats.

#### Accessing Root Transaction

The `root` property provides direct access to the tree's root transaction:

```python
root_transaction = tree.root

print(root_transaction)
```

#### Result

```python
<Transaction hash='8be545e0cfbdc421a1c79dfd074ec351ba6a36d89aee927efda2b879dc3be323', Ordinary>
```

This root transaction often serves as the starting point for any traversal or analysis.

#### Listing Child Transactions

By using the `children` property, you can retrieve a list of the immediate child transactions that stem from the root:

```python
child_transactions = tree.children

print(child_transactions)
```

#### Result

```python
[<builtins.TransactionTree object at 0x103a6fef0>]
```

#### Iterating Over Transactions

The `TransactionTree` class is inherently iterable. This means you can effortlessly loop over all transactions in the tree:

```python
for tx in tree:
    print(tx)
```

#### Result

```python
<Transaction hash='8be545e0cfbdc421a1c79dfd074ec351ba6a36d89aee927efda2b879dc3be323', Ordinary>
<Transaction hash='4c19d256001246e0bcdd167a94100282dbc73f5aa4ef1f56593f68c8acbc20bf', Ordinary>
<Transaction hash='0f3af431a6bd7325318abff024d23ef661e07f80c099d3f41838f4d7af078fc8', Ordinary>
<Transaction hash='994e9750c334245c1c0850ed339b738d394f5e3784191ed52ea193815a87cd41', Ordinary>
<Transaction hash='f0d4e5dfcdfdfe66b3b25e67b15c5dd15cddf6891693b8d10d091dfdacd943d0', Ordinary>
<Transaction hash='7c57cf3af3ec7e10c5366b46f3e71b513ba865f63f145cad0bb49b1d07c1b73e', Ordinary>
<Transaction hash='e506558308a4eee1ead003d52e0b14f0fb81eccdd5477e5fc5b6a07806216fc1', Ordinary>
<Transaction hash='3666f1e674c959c4ecd31ec0d0b2a317c4b7c45d5b1762392a230a6892ea8e85', Ordinary>
```

## Transaction Trace

The `TraceTransaction` class provides an asynchronous iterator for traversing transactions, making it ideal for non-blocking transaction analysis in asynchronous environments.

### Obtaining the Trace

To start, you need to obtain the trace using the transport. You can fetch the trace for a specific transaction either by its hash in bytes or by using an instance of the `Transaction`:

```python
trace = transport.trace_transaction(tx, yield_root=True)

print(trace)
```

#### Result

```python
<builtins.TraceTransaction object at 0x100d83310>
```

### Asynchronous Context Management

Once initialized, the `TraceTransaction` class can be utilized within an asynchronous context manager (`async with`) due to its `__aenter__` and `__aexit__` methods. This ensures the proper setup and teardown of the iterator.

```python
async with transport.trace_transaction(
  bytes.fromhex("a82e6603dec13499dd43486203b3304122ae89c13a80b189eef8976834cba413"),
  yield_root=True) as trace:
    # Use the trace iterator here
```

### Asynchronous Iteration

With the `__aiter__` and `__anext__` methods, you can asynchronously iterate over the transactions.

```python
async for transaction in trace:
    print(transaction)
```

### Waiting for Completion

After starting the iteration, you might want to ensure that all transactions are fetched. The `wait` method allows you to asynchronously pause until the last transaction is retrieved.

```python
await trace.wait()
```

### Closing the Iterator

After you're done with the iterator, especially if you're not using the context manager approach, it's a good practice to close it to free up resources. The `close` method allows for the explicit termination of the asynchronous iterator.

```python
await trace.close()
```
