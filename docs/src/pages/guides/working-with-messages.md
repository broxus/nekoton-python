---
outline: [2, 3]
---

# Working with Messages

The `nekoton-python` library provides a robust set of tools to work with messages in the TVM blockchain. Messages are crucial for communication between smart contracts or between smart contracts and external applications. This guide will walk you through the different types of messages and how to work with them.

## Message Types

In TVM blockchains, there are three types of messages:

- External Inbound Messages
- Internal Messages
- External Outbound Messages

These message types are represented by the `MessageType` enum in `nekoton-python`.

## Initialization Methods

The `Message` class offers two static methods that allow you to manually initialize a `Message` object. You can either use raw bytes or a cell to create a new instance of a `Message`.

### Initialize from Raw Bytes

To create a `Message` object from raw bytes, use the `from_bytes` method. The bytes should contain the BOC (Bag-of-Cells) data.

```python
message_raw_bytes = b"\xb5\xee\x9cr\x01\x01\x01\x01\x00\x81\x00\x00\xfd\x88\x00\r\x88\t3\x17iM\xeb\x9f\xc8\xcb'<|j\xc5\xda\xae|O\xef \xc6\xabcS\xc3\x9e\xc3\xebt\\\x05ag\x92\xed\xdc\xbe'(\x8a5\xbfI\xc5/\x18\x8a\xfa\x0b9y_+40\xd3\xe3\xbc\xf2\xd1\xe6W\xec\x98~\xca\xa61aB\x12\x12sS\x8e7\xed\xcd\\\xb2\xefX\x90\xbc\x9c\xf3 mk;\xa2\xbf\x84T\x14\x00\x00\x06'\xc2Fz\xf1\x00\x08\xbd\xc8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xaa"

# Initialize the message from the raw bytes
message = Message.from_bytes(message_raw_bytes)

print(message)
```

#### Result:

```python
<Message hash='dcbfbbce5aecdb9a486b8009e5f193f8bc7006455643e7ed83044afabe5cb8f2', ExternalIn>
```

### Initialize from a Cell

To create a `Message` object from a `Cell`, use the `from_cell` method. The cell should be the root message cell.

```python
message_base64_boc = "te6ccgEBAQEAgQAA/YgADYgJMxdpTeufyMsnPHxqxdqufE/vIMarY1PDnsPrdFwFYWeS7dy+JyiKNb9JxS8YivoLOXlfKzQw0+O88tHmV+yYfsqmMWFCEhJzU4437c1csu9YkLyc8yBtazuiv4RUFAAABifCRnrxAAi9yAAAAAAAAAAAAAAAAAAAAKo="

# Decode the message cell
message_cell = nt.Cell.decode(message_base64_boc)

# Initialize the message from the cell
message = Message.from_cell(message_cell)

print(message)
```

#### Result:

```python
<Message hash='dcbfbbce5aecdb9a486b8009e5f193f8bc7006455643e7ed83044afabe5cb8f2', ExternalIn>
```

## Decoding Messages

The `Message` class offers a static method for decoding a `Message` object from an encoded BOC string. The method allows you to specify the encoding type, which can be either `base64` (default) or `hex`.

### From an Encoded BOC String

```python
# Decode the message from an encoded BOC string
message = Message.decode(encoded_boc_string, encoding='base64')

print(message)
```

#### Result:

```python
<Message hash='dcbfbbce5aecdb9a486b8009e5f193f8bc7006455643e7ed83044afabe5cb8f2', ExternalIn>
```

:::tip
This method is particularly useful when the encoded BOC string is obtained from an external source, such as a blockchain explorer. It is more appropriate to use this method in such cases, rather than when you have created a cell during the initialization step.
:::

## Encoding Messages

The `Message` class provides methods for encoding a `Message` object into different formats: a string, bytes, or a cell.

### To a String

To encode a `Message` object into a string, use the `encode` method.

```python
# Encode the message to a string
encoded_message = message.encode('base64')

print(encoded_message)
```

#### Result:

```python
te6ccgEBAQEAgQAA/YgADYgJMxdpTeufyMsnPHxqxdqufE/vIMarY1PDnsPrdFwFYWeS7dy+JyiKNb9JxS8YivoLOXlfKzQw0+O88tHmV+yYfsqmMWFCEhJzU4437c1csu9YkLyc8yBtazuiv4RUFAAABifCRnrxAAi9yAAAAAAAAAAAAAAAAAAAAKo=
```

### To Bytes

To encode a `Message` object into bytes, use the `to_bytes` method.

```python
# Encode the message to bytes
encoded_message_bytes = message.to_bytes()

print(encoded_message_bytes)
```

#### Result:

```python
b"\xb5\xee\x9cr\x01\x01\x01\x01\x00\x81\x00\x00\xfd\x88\x00\r\x88\t3\x17iM\xeb\x9f\xc8\xcb'<|j\xc5\xda\xae|O\xef \xc6\xabcS\xc3\x9e\xc3\xebt\\\x05ag\x92\xed\xdc\xbe'(\x8a5\xbfI\xc5/\x18\x8a\xfa\x0b9y_+40\xd3\xe3\xbc\xf2\xd1\xe6W\xec\x98~\xca\xa61aB\x12\x12sS\x8e7\xed\xcd\\\xb2\xefX\x90\xbc\x9c\xf3 mk;\xa2\xbf\x84T\x14\x00\x00\x06'\xc2Fz\xf1\x00\x08\xbd\xc8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xaa"
```

### Build a Cell from a Message

To build a cell from a `Message` object, use the `build_cell` method.

```python
# Build a cell from the message
cell = message.build_cell()

print(cell)
```

#### Result:

```python
<Cell repr_hash='dcbfbbce5aecdb9a486b8009e5f193f8bc7006455643e7ed83044afabe5cb8f2', bits=1014, refs=0>
```

## Checking Types

You can check the type of a message using the following properties:

```python
# Check if a message is an external inbound message
is_external_in = message.is_external_in()

# Check if a message is an external outbound message
is_external_out = message.is_external_out()

# Check if a message is an internal message
is_internal = message.is_internal()

print(is_external_in)
print(is_external_out)
print(is_internal)
```

#### Result:

```python
True
False
False
```

## Message Headers

Message headers contain essential information about the message, such as the source and destination addresses, the value attached to the message, and more.

### Accessing Headers

You can access the header of a message using the `header` property:

```python
# Get the header of a message
header = message.header

# Get the type of the message from the header
message_type = header.type

print(header)
print(message_type)
```

#### Result:

```python
<builtins.ExternalInMessageHeader object at 0x101c224f0>

ExternalIn
```

### Header Properties

Depending on the type of the message, you can access more information from the header.

#### Internal Messages

For internal messages, you can access the following properties:

```python
ihr_disabled = header.ihr_disabled
bounce = header.bounce
bounced = header.bounced
src = header.src
dst = header.dst
value = header.value
ihr_fee = header.ihr_fee
fwd_fee = header.fwd_fee
created_at = header.created_at
created_lt = header.created_lt
```

##### Result:

```python
True
True
False
0:3c9893fce0c401e7890eec6599c0b79be25314299ec47a9e78627c42e947d792
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e
1
0
0.000333336
1694222262
21376022000002
```

#### External Inbound Messages

For external inbound messages, you can access the following properties:

```python
dst = header.dst
import_fee = header.import_fee

print(dst)
print(import_fee)
```

##### Result:

```python
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e
0
```

#### External Outbound Messages

For external outbound messages, you can access the following properties:

```python
src = header.src
created_at = header.created_at
created_lt = header.created_lt

print(src)
print(created_at)
print(created_lt)
```

##### Result:

```python
0:06c404998bb4a6f5cfe465939e3e3562ed573e27f7906355b1a9e1cf61f5ba2e
1694222262
21376022000004
```

## Unsigned External Message

An `UnsignedExternalMessage` represents an external message before it's signed. It provides a structured way to manage and manipulate messages before they are signed and sent to smart contracts.

### Properties of an Unsigned External Message

#### Hash

The `hash` property returns the hash of the message.

```python
# Get the hash of the unsigned message
hash = unsigned_message.hash

print(hash)
```

##### Result:

```python
b'{\x9f\xf0\xab8\xd4G\xf7\xc9\xedTV\x97//\xb4\x05o\xc0\x9eS\xed-\x81\xb24\x86I\xf0\xe3\x10\xf1'
```

#### Expiration Time

The `expire_at` property returns the expiration unix timestamp of the message.

```python
# Get the expiration time of the unsigned message
expire_at = unsigned_message.expire_at

print(expire_at) # 1695429486
```

### Signing an Unsigned External Message

The `sign` method allows you to sign an unsigned external message. This requires a `KeyPair` and an optional `signature_id`.

```python
# Sign the unsigned message
signed_message = unsigned_message.sign(keypair, signature_id)

print(signed_message)
```

##### Result:

```python
<SignedExternalMessage hash='6bddfe073564fe7439f3132c21be0d436c4a98638d8d28e35052237b1393e1d1', expire_at=1695429300, ExternalIn>
```

### Inserting a Signature

The `with_signature` method allows you to insert a signature into the body of the message. This requires a valid `ed25519` signature as the parameter.

```python
# Create a signature
signature = keypair.sign_raw(unsigned_message.hash)

# Insert a signature into the body of the message
signed_message = unsigned_message.with_signature(signature)

print(signed_message)
```

##### Result:

```python
<SignedExternalMessage hash='6bddfe073564fe7439f3132c21be0d436c4a98638d8d28e35052237b1393e1d1', expire_at=1695429300, ExternalIn>
```

### Inserting a Fake Signature

The `with_fake_signature` method allows you to insert a fake signature into the body of the message. This method does not require any parameters.

```python
# Insert a fake signature into the body of the message
signed_message = unsigned_message.with_fake_signature()

print(signed_message)
```

##### Result:

```python
<SignedExternalMessage hash='b8996a7f99e77e5c3dac9bfdf109d5c61dc83dde17fbc97809903c6f2102335b', expire_at=1695429300, ExternalIn>
```

:::tip
This method is useful for testing purposes when you need to create a signed message but the validity of the signature is not important.
:::

### Creating a Message Without a Signature

The `without_signature` method allows you to create a message without a signature. This method does not require any parameters.

```python
# Create a message without a signature
signed_message = unsigned_message.without_signature()

print(without_signature_message)
```

##### Result:

```python
<SignedExternalMessage hash='46c8f90a137f5141a2d8c8157878d36cec4f47f281f4fa2865c856738674fa6c', expire_at=1695429300, ExternalIn>
```

## Unsigned Message Body

### Signing the External Message Body

The `sign` method of the `UnsignedBody` class allows you to sign the body of an external message. This requires a `KeyPair` and an optional `signature_id`.

```python
# Sign the body of the unsigned message
signed_body = unsigned_body.sign(keypair, signature_id)

print(signed_body)
```

##### Result:

```python
<Cell repr_hash='d35449d816f78e69cae9a447fa00097468a84272d1a4480d92cd7e071355318b', bits=737, refs=0>
```

### Inserting a Signature

The `with_signature` method allows you to insert a signature into the body of the message. This requires a valid `ed25519` signature as the parameter.

```python
# Insert a signature into the body of the message
signed_body = unsigned_body.with_signature(signature)

print(signed_body)
```

##### Result:

```python
<Cell repr_hash='d35449d816f78e69cae9a447fa00097468a84272d1a4480d92cd7e071355318b', bits=737, refs=0>
```

### Inserting a Fake Signature

The `with_fake_signature` method allows you to insert a fake signature into the body of the message. This method does not require any parameters.

```python
# Insert a fake signature into the body of the message
signed_body = unsigned_body.with_fake_signature()

print(signed_body)
```

##### Result:

```python
<Cell repr_hash='cd620bb427b5a0662cffccad9015558e5ed236226bc389f6a3b1183a738ad820', bits=737, refs=0>
```

:::tip
This method is useful for testing purposes when you need to create a signed message body but the validity of the signature is not important.
:::

### Creating a Message Body Without a Signature

The `without_signature` method allows you to create a message body without a signature. This method does not require any parameters.

```python
# Create a message body without a signature
signed_body = unsigned_body.without_signature()

print(signed_body)
```

##### Result:

```python
<Cell repr_hash='fdf652f24c10b6b8e765c6f6bfac82681190f17274ff492647dcd4be76e7eb1a', bits=225, refs=0>
```

### Internal Message Body

While the `UnsignedBody` class is used for external message bodies, an internal message body is represented as a `Cell`. A `Cell` can be passed into an external message body.

This is useful in the following situations:

1. To create an internal message for a local executor. This is helpful when you need to evaluate a specific part of a chain in isolation or for testing purposes.

2. For contracts that simply forward the received `Cell` with the message. For example, in EverWallet, there is a separate method that can send up to four messages in this way.

You can create, manipulate, and evaluate `Cell` instances using the methods provided in the client libraries. However, unlike `UnsignedBody`, a `Cell` doesn't need to be signed or carry a signature because it's meant for internal use and not for external validation.

:::tip
Please consult ourguide on [**Working With Cells**](./working-with-cells.md) for detailed instructions on how to manipulate `Cell` instances for internal message bodies
:::

## Signed External Messages

A `SignedExternalMessage` represents an external message after it's signed.

### Properties of a Signed External Message

#### Hash

The `hash` property returns the hash of the message.

```python
# Get the hash of the signed message
hash = signed_message.hash

print(hash)
```

##### Result:

```python
b"\xc7\xc3\xa4 @\x0f\xdf\xc6#\xd8\x1c\\\xdd\x0e\xd7\x0f\x01\x9f\x18M\xe53\xa8\x94\xcc\x8f$\x10'\xd5\x95\xd5"
```

#### Expiration Time

The `expire_at` property returns the expiration unix timestamp of the message.

```python
# Get the expiration time of the signed message
expire_at = signed_message.expire_at

print(expire_at) # 1695429300
```

### Splitting a Signed Message

The `split` method allows split into inner message and expiration timestamp.

```python
# Split the signed message
inner_message, expire_at = signed_message.split()

print(inner_message)
print(expire_at)

```

##### Result:

```python
<Message hash='2db0a84737948e6ea62108e9a152ee08e00adfed8c2504fadfc530d54918f40f', ExternalIn>
1695429300
```
