# Nekoton-Python

Nekoton-Python is a Python binding for [Nekoton](https://github.com/broxus/nekoton), a Rust library that acts as the foundation for wallets interacting with TVM (Threaded Virtual Machine) compatible blockchains. This binding allows Python developers to utilize Nekoton's functionality in their applications, offering an abstraction over transport and support for [TIP-3.1](https://docs.everscale.network/standard/TIP-3) / Ever tokens. It can be used to develop Python applications that interact with TVM compatible blockchains, including creating and managing wallets, sending and receiving TIP-3.1 / Ever tokens, and interacting with smart contracts.

Nekoton-Python primarily consists of the following modules:

- **abi**: This module is responsible for handling the Application Binary Interface (ABI), which is used for interacting with smart contracts on the blockchain.
- **crypto**: This module provides cryptographic functionalities, including signature verification for public keys and key pairs.
- **models**: This module defines various data models used by Nekoton.
- **transport**: This module provides an abstraction over the transport layer, allowing Nekoton to send and receive messages from the blockchain.
- **util**: This module contains various utility functions used throughout Nekoton.

:::tip
Nekoton-Python is built "on top" of [**Everscale-jrpc**](https://github.com/broxus/everscale-jrpc), providing a robust and efficient foundation for blockchain interaction.
:::

## Core Features

- **Transport Abstraction**: Nekoton-Python provides an abstraction over the transport layer, simplifying the process of sending and receiving messages from the blockchain.
- **TIP-3.1 / Ever Token Support**: Nekoton-Python supports TIP-3.1 / Ever tokens, enabling developers to interact with these tokens in their Python applications.
- **Wallet Core**: Nekoton-Python serves as a core for wallets, providing essential functionalities for creating and managing wallets on the blockchain.

<!-- ## Table of Contents

- Introduction
- Installation & Setup
- Keys & Signatures
  - Seeds
    - Legacy & Bip39 Seeds
    - Generating Seeds
  - KeyPair Management
    - Generating a KeyPair
    - Deriving a KeyPair
  - Public Key Operations
    - Initialization
    - Encoding
    - Byte Conversion
  - Signature Operations
    - Signing Data
    - Verifying Signatures
- Working with Cells
  - Understanding Cells
    - Working with Tokens
  - Decoding & Encoding
  - Building & Unpacking
  - Cell Hashing & Comparison
  - Contract State
    - Building
    - Decoding & Encoding
    - Getting & Updating
    - Compute Address from State
- Interacting with Contracts
  - Contract ABI
    - Reading from File
    - Initializing & Decoding Contract ABI
    - Searching for Function & Event ABI
    - Encoding & Decoding Initial Contract Data
  - Working with Function ABI
    - Encoding & Decoding Calls
  - Working with Event ABI
    - Decoding Event Data
- Accounts, Messages & Transactions
  - Accounts
    - Account State
      - Reading Account State
      - Account Status
    - Blockchain Config
      - Reading Config
      - Checking Parameters
    - Interacting with Addresses
      - Creating & Validating
      - Address Parts
  - Messages
    - Message Types
      - Decoding & Encoding
      - Checking Types
    - Message Headers
      - Reading Headers
    - Signed & Unsigned External Messages
  - Transactions
    - Transaction Executor
      - Initializing & Executing Transactions
      - Context with Clock
    - Transaction Types
      - Checking Transaction Types
    - Transaction Phases
      - Storage Phase
      - Credit Phase
      - Compute Phase
      - Action Phase
      - Bounce Phase
    - Output
    - Transaction Tree
      - Decoding
      - Transaction Tree Iteration
    - Tracing
  - Asynchronous Iteration over Account States and Transactions
- Network Interaction
  - Transport
    - Checking Connection
    - Sending External Messages
    - Fetching Blockchain Config & Account State
    - Fetching Transactions & Account States
  - Working with GqlTransport
    - Querying Transactions, Messages, and Accounts
    - `path_for_account`: Returns a default derivation path for the specified account number.
    - `derive`: Derives a key pair using some derivation path. -->
