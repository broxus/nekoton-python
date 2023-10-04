# Data Representation

In the context of TVM (TON Virtual Machine) based blockchains, data representation plays a crucial role. This document aims to provide a comprehensive overview of how data is represented and stored in these blockchains.

At the most basic level, everything in a blockchain is stored in a structure called a `cell`. A cell can contain up to **1023 data bits** and **up to 4 references** to other cells. Any value can be represented as a tree of cells. The specific structure for representing various data types is described in the ABI specification.

## Cells: The Fundamental Units of Data

Importantly, cyclic references between cells are not allowed. As a result, all cells form a directed acyclic graph (`DAG`), where each cell can be viewed as an individual node in the graph.

The diagram below illustrates this concept, showing how the cells interconnect to form a DAG structure.

<BDKImgContainer src="./../dag-diagram.png" maxWidth='50%' altText="directed acyclic graph" padding="20px 0 20px 0"/>

## Constraints on Cell Structure

While the cell structure provides a flexible way to store data, there are some constraints on the overall structure:

- The maximum depth of the tree of cells is **2<sup>16</sup>**.
- For external messages, the maximum tree depth is limited to **512**.

## Example of Data Representation

For example, a tuple `(uint32, bool, uint32[])` with values `(0x539, true, [0x0B, 0x16])` can be represented as a tree of cells as shown in the example below:

```
Ordinary   l: 000   bits: 66   refs: 1   data: 00000539800000016_
hashes: 29a11f1e37e0c64354f52be1f517992639e91a7d07487630c1e3800a479277ba
depths: 2
  └─Ordinary   l: 000   bits: 9   refs: 2   data: cfc_
  hashes: a590c29333e1d2060a079b8bd1f8f57a56408d87e1586ccbe6caa888ae34abc0
  depths: 1
  ├─Ordinary   l: 000   bits: 34   refs: 0   data: 00000002e_
  │ hashes: 3d10b2cb5aa6f262a35dc82a384d326f9b3667c1c8002021382987a88ca8482b
  │ depths: 0
  └─Ordinary   l: 000   bits: 34   refs: 0   data: 00000005a_
    hashes: 43bd1f7b6ad2214e74ff517098fc7c45b9acd979b0da5e0cc804f6af313ce474
    depths: 0
```

This representation can be encoded as a `base64` string for transmission or storage.

## BOC (Bag of Cells)

The Bag of Cells (BOC) is a universal format for data packaging in TVM. Every object — account, transaction, message, block — is stored in the blockchain database as BOCs. The BOC of a block includes BOCs of all messages and transactions that were executed in this block.

## Cell Types and Flavors

There are five types of cells: `ordinary` and four `exotic` types, which include:

- Pruned branch cells
- Library reference cells
- Merkle proof cells
- Merkle update cells

:::tip
For more on exotic cells see: [TVM Whitepaper, Section 3](https://ton.org/tvm.pdf).
:::

Cells also come in different "flavors" for different purposes:

- **Builder**: for partially constructed cells, allowing fast operations for appending bitstrings, integers, other cells, and references to other cells.
- **Slice**: for 'dissected' cells representing either the remainder of a partially parsed cell or a value (subcell) residing inside such a cell.

## Serialization of Data to Cells

Any object in a blockchain (message, message queue, block, whole blockchain state, contract code, and data) serializes to a cell. The serialization process is described by a `TL-B` scheme, which is a formal description of how an object can be serialized into a `Builder` or parsed from a `Slice`.

In conclusion, data representation in blockchain is a complex but efficient process, allowing for compact storage and flexibility in data structures. Understanding this process is crucial for anyone working with these blockchains.

## Cost of Data Storage in Cells

Storing data in a blockchain's cells is associated with costs, and the contract pays for the storage of both bits and references. These costs are crucial to understand, as they influence how data structures are described and organized.

- In the `masterchain`, each bit costs 1000 units, and every reference costs 500,000 units.
- In the `base workchain`, each bit costs 1 unit, and every reference costs 500 units.

The exact values can be found in the 18th parameter of the network config, and these costs might vary among different TVM chains.

### Example: Storage Fees on Everscale

In the Everscale network, a specific example of a TVM chain, the storage fees for each smart contract are calculated as:

```
storage_fees = CEIL(
   (
       account.bits * global_bit_price
       + account.cells * global_cell_price
   ) * period / 2 ^ 16
)
```

Here:

- `account.bits` and `account.cells`: Number of bits and cells in the smart contract, including code and data.
- `global_bit_price`: Price for storing one bit (p18 for both masterchain and workchains).
- `global_cell_price`: Price for storing one cell (p18 for both masterchain and workchains).
- `period`: Number of seconds a smart contract is stored for.

A cell can contain no more than 1023 bits and 4 references, and more complex data structures may require more cells to store the same amount of data.

An example calculation for storing 1062.5 KB of data for one day in Everscale is given [here](https://everscan.io/accounts/0:cd0b3e21ea59fc43a8343f935c7e74b4c22e3ba43f5e08410ffa371cedfe3dee). The minimum storage fee would be 0.014291591 EVERs.

:::warning Note
If there are insufficient funds to cover the storage fee, the smart contract will be frozen, its balance reduced to zero, and the remaining fee marked as debt.
:::
