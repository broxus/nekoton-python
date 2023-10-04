# Concept of Accounts

In the context of TVM-compatible blockchains, an account is the fundamental unit of information storage. Each account is uniquely identified by its full address, which is the combination of the workchain identifier and the account identifier. In essence, every account is a smart contract, even a wallet is a simple actor (and smart contract).

## Account Structure

An account is characterized by its address, which is a deterministic value derived from its code and initial data, specifically `cell_hash(cell_repr(StateInit))`. There are no special types of accounts for user wallets initiating transactions in the TVM blockchain. Wallets are typical smart contracts, and there are many different kinds. Any smart contract that allows the reception of external messages can initiate a transaction.

The comprehensive account record is represented by a series of nested data structures, including Account Storage, Account State, and Smart Contract Storage (also known as `StateInit`).

### Account Storage Structure

The outermost structure is the account storage, which encompasses the account balance and the account state. The account state might include the smart contract code and data, which are detailed in the `StateInit` structure.

```python
class AccountStorage:
  last_trans_lt: int
  balance: Tokens
  state: AccountState
```

AccountStorage fields:

- `last_trans_lt`: Logical time of the last transaction
- `balance`: The quantity of cryptocurrency tokens available for the account
- `state`: The present account state

### Account State

The account state determines the operation mode of the account while a message is being executed for that account. The Transaction Executor logic significantly varies depending on the current account state.

```python
class AccountState:
  status: AccountStatus
  state_init: Optional[StateInit]
  frozen_state_hash: Optional[bytes]
```

AccountState fields:

- `status`: Account status, which can be `Uninit`, `Active`, or `Frozen`
- `state_init`: StateInit for the active account
- `frozen_state_hash`: A hash of the last known state for the frozen account

### Smart Contract Storage (StateInit)

The contract's byte-code and data are stored within a structure called `StateInit`. This structure is also used for the initialization of an account when it is uninitialized.

```python
class StateInit:
  code: Optional[Cell]
  data: Optional[Cell]
```

StateInit fields:

- `code`: Optional contract code
- `data`: Optional contract data

The `code` and `data` fields encode the current byte-code and data of a contract, where `data` denotes the values of the contract's variables.

:::info
The representation provided here is a simplified Python representation. In the actual implementation, there are additional fields that are omitted in this library for simplicity.
:::

## Account Lifecycle

### Creation

Initially, an account does not exist in the blockchain. To create a record of it, we first calculate the address of the future contract (i.e., `cell_hash(cell_repr(StateInit))`) and send the required amount of native coins to this address with a special flag `bounce = false`. This flag indicates that if the recipient account does not exist, or if an error occurred during message processing, the coins should remain at this address rather than being sent back with a special error message.

After this process, we get an account in the blockchain with the status `Uninitialized`. This means that we have a record of the account in the blockchain, but no data and code.

### Activation

To transition an account to the `Active` status, we need to send a specially formulated message containing the data and code of this contract. Anyone can send such a message. Validators will verify that the contract address equals `cell_hash(cell_repr(StateInit))`, and if everything matches, the account will be initialized. This message can also include a function to be called immediately after account activation, along with its arguments. By default, the constructor is called.

Once the account becomes active, it can accept incoming internal and external messages. Every time an account receives a message, a transaction begins, during which the account can create up to 255 actions such as `Internal Messages`, `External Messages`, `rawReserve`, `setCode`.

### Utilization

To create a wallet in the network, we simply create a private/public key pair, take the code of our wallet, calculate the address from the code + public key, and send native coins to this address. Once there are coins, we can initialize the wallet and start using it.

When an account receives a message, the Transaction Executor is launched. Before the contract code execution begins, a storage fee is deducted from the account for all the time that has passed since the previous transaction. The storage fee depends linearly on the size of the contract's data and code. If the contract's balance becomes negative after deducting the storage fee, the transaction does not occur, and the account transitions to the `Frozen` state. In this state, the contract's data and code are deleted, leaving only the state hash. The contract will remain in the Frozen state until the debt for its storage reaches the deletion threshold. This is a network parameter, currently -0.1. After that, the contract will be permanently deleted without the possibility of recovery.

Also, during a transaction, a contract can create an outgoing message with a special flag, indicating that all remaining money should be sent with this message, after which the account should be deleted.

:::warning Caution
Please note that the storage fee, which depends linearly on the size of the contract's data and code, is not a one-time payment. This fee is essentially a rental cost for the storage space used by the contract. The fee is charged for the size of the code multiplied by the time elapsed since the last payment.
::::

## Account Abstraction

Account Abstraction (AA) is a fundamental concept in TVM blockchains. Unlike certain other blockchain (like a EVM) architectures, where AA is complex and requires changes to core protocol, TVM blockchains have a natively built-in, more user-friendly AA design. This approach is aimed at facilitating extended wallet functionality, enhancing security, and improving user experience.

### Overview

In this framework, whether it's a native coin or a TIP-3 token, value can only move as an outcome of smart contract code execution. There are no Externally Owned Accounts (EOA), just Accounts, which are "abstract".

An account is deployed with some initial state (code + data), and the TVM includes instructions to access and modify accounts' code, state, send messages, deploy new accounts, and more. Executing the code in the TVM is initiated by an inbound message.

### Interaction

Users communicate with accounts by sending External Messages, using key pairs. With no EOAs, these External Messages do not carry any value. On receipt of an External Message, the TVM provides a small portion of "credit gas" to perform specific logic before accepting the message. Each Account can be seen as its own EntryPoint.

### Account Address

Each account is content-addressable, with the account address derived from its initial code and state. This approach enables developers to build secure systems with address-based access control rules, without the need to maintain explicit access control lists.

### Upgrading

On TVM, the `SETCODE` instruction allows an account to self-upgrade using any code it can obtain from an inbound message or its own storage. This way, accounts' addresses remain unchanged, and the upgrade requires no additional deployments.

## Smart Contract Wallets

The driving force behind AA is to give developers the power to implement the ownership model, based on the concept that "code is law".

For TVM blockchains, formally verified implementations for singlesig/multisig are available, along with an upgradable version. This facilitates the creation of any logic and integration on top, keeping the address unchanged.
