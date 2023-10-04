---
outline: [2, 4]
---

# Keys & Signatures

Nekoton-Python provides a comprehensive set of tools for managing cryptographic keys and signatures, which are essential for interacting with the blockchain. This section will guide you through the usage of these tools.

## KeyPair

A `KeyPair` in cryptographic systems pertains to a duo of a private (or secret) key and a public key. It plays a pivotal role in both encrypting and decrypting data, as well as in digital signatures. Specifically, for Ed25519, a widely-accepted and modern digital signature system, the `KeyPair` can be generated from a seed or directly from a secret key.

For Ed25519, the `KeyPair` can also be generated in two primary ways:

- Randomly, utilizing the `KeyPair.generate()` method.
- Derivatively, from a seed using the `derive()` method, which is a feature of the respective seed class.

In the context of Ed25519, the secret key's size is crucial, with the standard mandating a precise length of 32 bytes. This specification ensures the security and consistency of the cryptographic operations facilitated by the key pair.

### Generating a KeyPair

A `KeyPair` can be generated randomly using the `KeyPair.generate()` method:

```python
keypair = nt.KeyPair.generate()

print(f"Public key: {keypair.public_key}")

print(f"Secret key: {keypair.secret_key}")
```

##### Result

```python
Public key: 6b6243e2fa88025d5f2fee051bd8b62ff3da730136c67bbb14033b72c60fb762

Secret key: e7f25e4cf517661a51081779dd7218d42c7db4bb85b18406e337a2ddb4359599
```

## Seeds

A seed is a piece of data that can be used to generate deterministic keys.
In Nekoton-Python, two types of seeds are supported: `Bip39` and `Legacy`.

### Bip39 Seeds

Bip39 Seeds are 12-word seeds that can be used to derive a `KeyPair` following the BIP39 standard. They can be generated using the `Bip39Seed.generate()` method, and a `KeyPair` can be derived from a Bip39 seed using the `Bip39Seed.derive()` method.

Example of generating and using a Bip39 seed:

```python
bip39_seed = nt.Bip39Seed.generate()

print(bip39_seed)
```

##### Result

```python
cricket prize gain hidden dragon fossil repeat blue dream already shaft
exclude
```

Deriving a `KeyPair` from a Bip39 Seed:

```python
keypair = bip39_seed.derive()

print(keypair.public_key, keypair.secret_key)
```

##### Result

```python
3247bd75e041a03c9029297b89bb40132744df380596fb4bda4591f1dd9313d7

b"\xbd\xac\xcd\x0e\x89c\xab\xaeZ:!\xf7\xe6\xd9\x9f\xe5a=\x06z0-n'\xc5\xd0\xed\x8e\xee\xb5\x93\x94"
```

### Legacy Seeds

Legacy Seeds are 24-word seeds that can be used to derive a `KeyPair`. They can be generated using the `LegacySeed.generate()` method, and a `KeyPair` can be derived from a legacy seed using the `LegacySeed.derive()` method.

Example of generating and using a Legacy seed:

```python
legacy_seed = nt.LegacySeed.generate()

print(legacy_seed)
```

##### Result

```python
away october another abuse bridge woman local lottery ostrich genuine
obvious minor brand wall upper column response bus nose lonely question
useful grocery unable
```

Deriving a `KeyPair` from a Legacy Seed:

```python
keypair = legacy_seed.derive()

print(keypair.public_key, keypair.secret_key)
```

##### Result

```python
7ec3e8c544c021808be23b10829440ea45175e76b9d5ede46a7e8d59085c3228

b'{I\xd1\x02*>\x16\x1c\xa7Z\x97\x01<\x1a\x07\x0b\xcc\xb0\x1d\x18i\xbar\xe7aE\x1e\x9d\xb3\xc3\xd8\xaf'
```

### Derivation Path

In the context of BIP39, a derivation path is used to derive different keys from the same seed phrase. It provides a hierarchical structure for generating and organizing keys. With Nekoton-Python, you can retrieve the default derivation path for a specified account number using the `path_for_account` method.

Here's how you can get the derivation path for a specified account number using a Bip39 seed:

```python
path = bip39_seed.path_for_account(0)

print(path) # m/44'/396'/0'/0/1
```

## Public Key Operations

The `nekoton` library provides various methods to work with public keys. You can initialize, encode, and convert a `PublicKey` using the provided methods.

### Initialization

Public keys can be initialized from different formats:

#### From Integer

You can initialize a public key from an integer using the `PublicKey.from_int()` method.

```python
public_key = nt.PublicKey.from_int(63837483679490186262641015239053288982995430350508212654141177365814141551489)

print(public_key)
```

##### Result

```python
8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

#### From Bytes

You can initialize a public key from bytes using the `PublicKey.from_bytes()` method.

```python
public_key = PublicKey.from_bytes(b'\x8d"\xbc?\x15o@\t44\x06\x07\xe3r\x07k\x9a\x02<n\xc5\x91Z\xa2\xf7\x90\xba\x9b\xce\x08\x83\x81')

print(public_key)
```

##### Result

```python
8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

#### From String

You can initialize a public key from a string directly.

```python
public_key = PublicKey("8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381")

print(public_key)
```

##### Result

```python
8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

### Encoding

A `PublicKey` can be encoded to a string using the `PublicKey.encode()` method.

```python
encoded = public_key.encode()

print(encoded)
```

##### Result

```python
8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

### Byte Conversion

Convert a `PublicKey` to bytes using the `PublicKey.to_bytes()` method.

```python
bytes_representation = public_key.to_bytes()

print(bytes_representation)
```

##### Result

```python
b'\x8d"\xbc?\x15o@\t44\x06\x07\xe3r\x07k\x9a\x02<n\xc5\x91Z\xa2\xf7\x90\xba\x9b\xce\x08\x83\x81'
```

This structure provides a clear distinction between the different methods of initializing a `PublicKey`, as well as its encoding and conversion functionalities.

## Signing Data

Signing data is a fundamental cryptographic operation that provides both authentication and data integrity. By creating a digital signature for a specific set of data, you not only prove the origin of that data (authentication) but also confirm that the data hasn't been tampered with since the signature was created (integrity). This process can be accomplished using different methods, depending on the requirements of the specific application or system.

In this section, we'll explore two primary ways of signing data: with hashing (using the `sign()` method) and without hashing (using the `sign_raw()` method).

Additionally, we will touch upon the optional incorporation of a `signature_id` to further enhance the identification of the signed data.

### Data with Hashing

When signing data using the `sign()` method, the data is first hashed before being signed. This is useful when you want to ensure the integrity of the data being signed.

```python
data = b"Hello, World 42!"

signature_id = await transport.get_signature_id() # Optional

signature = keypair.sign(data, signature_id)

print(signature)
```

##### Result

```python
Signature('b2bd3045b3ec3872bcccc96f58b71fe0fd60cba104249cc5e72c1a2ebad35cbbf1d82a631dda5cc7a8f07b540fb1564edfa0920ede751a59e08d3ed54f80e908')
```

:::tip
The `signature_id` is an optional identifier for a signature, sourced directly from the transport layer using the `get_signature_id()` method. If this is your first time encountering `signature_id` or you're unfamiliar with our transport layer, it's recommended to [read our guide on working with the transport](./working-with-transport.md) to get started.
:::

### Raw Data

The `sign_raw()` method signs the data directly without hashing it. This can be useful if you need to sign data that doesn't require hashing or in cases where the data has already been hashed:

```python
data = b"Hello, World 42!"

signature_raw = keypair.sign_raw(data)

print(signature_raw)
```

##### Result

```python
Signature('ce998c9cf3dcf0aecd2b3a372661e7d75946fd3f2dfa793a4f3944da985fe49ad9c6e23b2fbce2dee31b802a3bff646ff5ac268a4a54c7aa319882e4817b4504')
```

## Verifying Signatures

To verify a signature, you need to use the correct input depending on the signing method. If the data was signed with `sign()`, you should use the hashed data.

If it was signed with `sign_raw()`, you should either use the hash of the original data (if you want to verify the signature against hashed data) or the original data itself (if you want to verify the signature against raw data).

When a `signature_id` was used during signing, the same `signature_id` should be used for verification.

### Hashed Signature

To verify a hashed signature, you will first need to hash the original data using the SHA-256 algorithm, and then call the `check_signature()` method:

```python
import hashlib

data = hashlib.sha256(b"Hello, World 42!").digest()

is_valid = public_key.check_signature(data, signature, signature_id)

print(is_valid) # True
```

### Raw Signature

To verify a raw signature, you will need to call the `check_signature_raw()` method with the original raw data and the signature:

```python
data = hashlib.sha256(b"Hello, World 42!").digest()

signature_raw = keypair.sign_raw(data)
is_valid_raw = public_key.check_signature_raw(data, signature_raw)

print(is_valid_raw) # True
```
