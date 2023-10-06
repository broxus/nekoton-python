# Nekoton-Python

Nekoton-Python is a Python binding for [Nekoton](https://github.com/broxus/nekoton), a Rust library that acts as the foundation for wallets interacting with TVM (TON Virtual Machine) compatible blockchains. This binding allows Python developers to utilize Nekoton's functionality in their applications, offering an abstraction over transport and support for Native tokens. It can be used to develop Python applications that interact with blockchain.

Nekoton-Python primarily consists of the following modules:

- **abi**: This module is responsible for handling the Application Binary Interface (ABI), which is used for interacting with smart contracts on the blockchain.
- **crypto**: This module provides cryptographic functionalities, including signature verification for public keys and key pairs.
- **models**: This module defines various data models used by Nekoton.
- **transport**: This module provides an abstraction over the transport layer, allowing Nekoton to send and receive messages from the blockchain.
- **util**: This module contains various utility functions used throughout Nekoton.
