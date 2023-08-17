---
outline: deep
---

# Keys & Signatures

Nekoton-Python provides a comprehensive set of tools for managing cryptographic keys and signatures, which are essential for interacting with the blockchain. This section will guide you through the usage of these tools.

## Seeds

A seed is a piece of data that can be used to generate deterministic keys. In Nekoton-Python, two types of seeds are supported: Legacy and Bip39.

- **Legacy Seeds**: These are 24-word seeds that can be used to derive a `KeyPair`. They can be generated using the `LegacySeed.generate()` method, and a `KeyPair` can be derived from a legacy seed using the `LegacySeed.derive()` method.

- **Bip39 Seeds**: These are 12-word seeds that can be used to derive a `KeyPair` following the BIP39 standard. They can be generated using the `Bip39Seed.generate()` method, and a `KeyPair` can be derived from a Bip39 seed using the `Bip39Seed.derive()` method.

Examples of generating and using both types of seeds are provided below:

```python
from nekoton import LegacySeed

legacy_seed = LegacySeed.generate()
print(legacy_seed)

# away october another abuse bridge woman local lottery ostrich genuine
# obvious minor brand wall upper column response bus nose lonely question
# useful grocery unable
```

```python
from nekoton import Bip39Seed

bip39_seed = Bip39Seed.generate()
print(bip39_seed)

# cricket prize gain hidden dragon fossil repeat blue dream already shaft
# exclude
```

### Derivation Path

In the context of BIP39, a derivation path is used to derive different keys from the same seed phrase. It provides a hierarchical structure for generating and organizing keys. With Nekoton-Python, you can retrieve the default derivation path for a specified account number using the `path_for_account` method.

Here's how you can get the derivation path for a specified account number using a Bip39 seed:

```python
path = bip39_seed.path_for_account(0)
print(path)

# m/44'/396'/0'/0/1
```

## KeyPair

A `KeyPair` consists of a private key (or secret key) and a public key. It can be generated from a seed or from a secret key. A `KeyPair` can be generated randomly using the `KeyPair.generate()` method or derived from a seed using the `derive()` method of the respective seed class.

### Generating a KeyPair

A `KeyPair` can be generated randomly using the `KeyPair.generate()` method:

```python
from nekoton import KeyPair

keypair = KeyPair.generate()
print(keypair.public_key, keypair.secret_key)

# c32a0df58d495c15c37d19e0d9c0437a53280676d7941dd7256a3de057a80c51
# b"~\xe3P\na\xa6\xa5D\xf1\xd2z\x89I'
# 'W\xe7)\xb1\xe3c\xad!\xc7{\xe4\xc1\x06{W\xd24"
```

### Deriving a KeyPair

A `KeyPair` can also be derived from a seed. Here is an example of deriving a `KeyPair` from a `Legacy` seed:

```python
legacy_seed = LegacySeed.generate()
keypair = legacy_seed.derive()
print(keypair.public_key, keypair.secret_key)

# 7ec3e8c544c021808be23b10829440ea45175e76b9d5ede46a7e8d59085c3228
# b'{I\xd1\x02*>\x16\x1c\xa7Z\x97\x01<\x1a\x07\x0b\xcc\xb0\x1d\x18i\xbar\xe7aE\x1e\x9d\xb3\xc3\xd8\xaf'
```

And here is an example of deriving a `KeyPair` from a `Bip39` seed:

```python
from nekoton.crypto import Bip39Seed

bip39_seed = Bip39Seed.generate()
keypair = bip39_seed.derive()
print(keypair.public_key, keypair.secret_key)

# 3247bd75e041a03c9029297b89bb40132744df380596fb4bda4591f1dd9313d7
# b"\xbd\xac\xcd\x0e\x89c\xab\xaeZ:!\xf7\xe6\xd9\x9f\xe5a=\x06z0-n'\xc5\xd0\xed\x8e\xee\xb5\x93\x94"
```

## Public Key Operationsэ

Public keys can be initialized from an integer, bytes, or a string using the `PublicKey.from_int()`, `PublicKey.from_bytes()` methods respectively. They can be encoded to a string using the `PublicKey.encode()` method and can be converted to bytes using the `PublicKey.to_bytes()` method.

Examples of initializing, encoding, and converting a `PublicKey`:

```python
from nekoton import PublicKey

public_key = PublicKey.from_int(63837483679490186262641015239053288982995430350508212654141177365814141551489)
print(public_key)

public_key = PublicKey.from_bytes(b'\x8d"\xbc?\x15o@\t44\x06\x07\xe3r\x07k\x9a\x02<n\xc5\x91Z\xa2\xf7\x90\xba\x9b\xce\x08\x83\x81')
print(public_key)

public_key = PublicKey("8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381")
print(public_key)

# 8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

### Encoding

A `PublicKey` can be encoded to a string using the `PublicKey.encode()` method.

```python
public_key = PublicKey.from_bytes(b'\x8d"\xbc?\x15o@\t44\x06\x07\xe3r\x07k\x9a\x02<n\xc5\x91Z\xa2\xf7\x90\xba\x9b\xce\x08\x83\x81')
encoded = public_key.encode()
print(encoded)

# 8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381
```

### Byte Conversion

A `PublicKey` can be converted to bytes using the `PublicKey.to_bytes()` method.

```python
bytes = public_key.to_bytes()
print(bytes)

# b'\x8d"\xbc?\x15o@\t44\x06\x07\xe3r\x07k\x9a\x02<n\xc5\x91Z\xa2\xf7\x90\xba\x9b\xce\x08\x83\x81'
```

## Signing Data

When signing data using the `sign()` method, the data is first hashed before being signed. This is useful when you want to ensure the integrity of the data being signed. The `sign_raw()` method, on the other hand, signs the data directly without hashing it. This can be useful if you need to sign data that doesn't require hashing or in cases where the data has already been hashed.

### Data with Hashing

The `sign()` method hashes the data before signing it. This ensures the authenticity and integrity of the data:

```python
data = b"Hello, World 42!"
signature = keypair.sign(data)
print(signature)

# Signature('117f4a23998bb476aaef44963ba0f4d7a979cfb1290e262618acc3a2067967706edc8ced33373f25fe9486b78b9a55648731dae31f0f230eef37f2838c65df02')
```

## Verifying Signatures

To verify a signature, you need to use the correct input depending on the signing method. If the data was signed with `sign()`, you should use the hashed data. If it was signed with `sign_raw()`, you should either use the hash of the original data (if you want to verify the signature against hashed data) or the original data itself (if you want to verify the signature against raw data).

### Hashed Signature

To verify a hashed signature, you will first need to hash the original data using the SHA-256 algorithm, and then call the `check_signature()` method:

```python
import hashlib

data = hashlib.sha256(b"Hello, World 42!").digest()

is_valid = public_key.check_signature(data, signature)
print(is_valid)

# True
```

### Raw Signature

To verify a raw signature, you will need to call the `check_signature_raw()` method with the original raw data and the signature:

```python
data = hashlib.sha256(b"Hello, World 42!").digest()

signature_raw = keypair.sign_raw(data)
is_valid_raw = public_key.check_signature_raw(data, signature_raw)
print(is_valid_raw)

# True
```
