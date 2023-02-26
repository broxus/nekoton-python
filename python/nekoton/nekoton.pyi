from typing import Any, ClassVar, Optional, List, Tuple, Dict

class AbiAddress(AbiParam):
    """
    A class for a `address` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiArray(AbiParam):
    """
    A class for an `T[]` ABI type.

    :param value_type: the ABI type of array items.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...

class AbiBool(AbiParam):
    """
    A class for a `bool` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiBytes(AbiParam):
    """
    A class for a `bytes` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiCell(AbiParam):
    """
    A class for a `cell` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiFixedArray(AbiParam):
    """
    A class for a `T[N]` ABI type.

    :param value_type: the ABI type of array items.
    :param len: number of array items.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam, len: int) -> None: ...

class AbiFixedBytes(AbiParam):
    """
    A class for a `fixedbytesN` ABI type.

    :param len: number of bytes.
    """

    @classmethod
    def __init__(cls, len: int) -> None: ...

class AbiInt(AbiParam):
    """
    A class for an `intN` ABI type.

    :param size: int size in bits.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...

class AbiMap(AbiParam):
    """
    A class for an `map(K, V)` ABI type.

    :param key_type: the ABI type of mapping keys.
    :param value_type: the ABI type of mapping values.
    """

    @classmethod
    def __init__(cls, key_type: AbiParam, value_type: AbiParam) -> None: ...

class AbiOptional(AbiParam):
    """
    A class for an `optional(T)` ABI type.

    :param value_type: the ABI type of the inner value.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...

class AbiParam:
    """
    Base ABI type.
    """
    @classmethod
    def __init__(cls) -> None: ...

class AbiRef(AbiParam):
    """
    A class for a `ref(T)` ABI type.

    :param value_type: the ABI type of the inner value.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...

class AbiString(AbiParam):
    """
    A class for `string` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiToken(AbiParam):
    """
    A class for `token` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...

class AbiTuple(AbiParam):
    """
    A class for `(T1, T2, ..., Tn)` ABI type.

    :param items: ABI types of inner values.
    """

    @classmethod
    def __init__(cls, items: List[Tuple[str, AbiParam]]) -> None: ...

class AbiUint(AbiParam):
    """
    A class for `uintN` ABI type.

    :param size: uint size in bits.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...

class AbiVarInt(AbiParam):
    """
    A class for `varintN` ABI type.

    :param size: varint size in bytes.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...

class AbiVarUint(AbiParam):
    """
    A class for `varuintN` ABI type.

    :param size: varuint size in bytes.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...

class AbiVersion:
    """
    TVM ABI version.

    :param major: major version component.
    :param minor: minor version component.
    """

    @classmethod
    def __init__(cls, major: int, minor: int) -> None: ...
    def major(self) -> int:
        """
        Major TVM ABI version component.
        """
        ...
    def minor(self) -> int: 
        """
        Minor TVM ABI version component.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class AccountState:
    """
    A state of an existing account.
    """

    balance: int
    """Account balance in nano EVERs."""

    due_payment: Optional[int]
    """Optional account debt in nano EVERs."""

    frozen_state_hash: Optional[bytes]
    """A hash of the last known state for the frozen account."""

    last_paid: int
    """The last time when storage phase was executed."""

    last_trans_lt: int
    """The logical time of the last transaction."""

    state_init: Optional[StateInit]
    """StateInit for the active account."""

    status: AccountStatus
    """Account status."""

    storage_used: StorageUsed
    """Storage usage statistics."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class AccountStatus:
    """
    Account status.
    """

    Active: ClassVar[AccountStatus] = ...
    """Active (deplyed) account."""

    Frozen: ClassVar[AccountStatus] = ...
    """Frozen account. (Has no state, can be unfrozen or deleted)."""

    NotExists: ClassVar[AccountStatus] = ...
    """Account does not exist."""

    Uninit: ClassVar[AccountStatus] = ...
    """Account without a state."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __int__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class AccountStatusChange:
    """
    Account status change during transaction phase.
    """

    Deleted: ClassVar[AccountStatusChange] = ...
    """Account was deleted."""

    Frozen: ClassVar[AccountStatusChange] = ...
    """Account was frozen."""

    Unchanged: ClassVar[AccountStatusChange] = ...
    """Account status has not changed."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __int__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class Address:
    """
    Account address (`StdAddr`).

    :param addr: a string with raw address.
    """

    account: bytes
    """Hash of the initial state."""

    workchain: int
    """Address workchain."""

    @classmethod
    def __init__(cls, addr: str) -> None: ...
    def from_parts(self, workchain: int, account: bytes) -> Any: 
        """
        Creates an address from parts.

        :param workchain: address workchain.
        :param account: hash of the initial state.
        """
        ...

    def validate(self, addr: str) -> bool:
        """
        Checks whether the specified string is a valid address.

        :param addr: a string with raw address.
        """
        ...

    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class Bip39Seed(Seed):
    """
    BIP39 seed.

    :param phrase: a string with 12 words.
    """

    @classmethod
    def __init__(cls, phrase: str) -> None: ...
    def derive(self, path: Optional[str]) -> KeyPair:
        """
        Derives a key pair using some derivation path.

        :param path: custom derivation path.
        """
        ...
    def generate(self) -> Bip39Seed:
        """Generates a random BIP39 seed."""
        ...
    def path_for_account(self, id: int) -> str:
        """
        Returns a default derivation path for the specified account number.

        :param id: account number.
        """
        ...

class BlockchainConfig:
    """
    Partially parsed blockchain config.
    """

    capabilities: int
    """Required software capabilities as integer mask."""

    config_address: Address
    """Address of the config contract."""

    elector_address: Address
    """Address of the elector contract."""

    fee_collector_address: Address
    """Address of the fee collector contract."""

    minter_address: Address
    """Address of the minter contract."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def contains_param(self, index: int) -> bool:
        """
        Returns `True` if the config contains the specified param.

        :param index: param index.
        """
        ...

    def get_raw_param(self, index: int) -> Optional[Cell]:
        """
        Returns the corresponding config value as a cell.

        :param index: param index.
        """
        ...

class Cell:
    """
    A container with up to 1023 bits of data and up to 4 children.
    """

    repr_hash: bytes
    """Representation hash of the cell."""

    @classmethod
    def __init__(cls) -> None: ...
    def build(
        self,
        abi: List[Tuple[str, AbiParam]],
        value: Dict[str, Any],
        abi_version: AbiVersion
    ) -> Cell:
        """
        Packs values into cell using the provided ABI.

        :param abi: ABI structure.
        :param value: a dictinary with corresponding values.
        :param abi_version: optional ABI version.
        """
        ...
    def decode(self, value: str, encoding: Optional[str]) -> Cell:
        """
        Decodes the cell from the encoded BOC.

        :param value: a string with encoded BOC.
        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...
    def encode(self, encoding: Optional[str]) -> str:
        """
        Encodes the cell into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...
    def from_bytes(self, bytes: bytes) -> Cell:
        """
        Decodes cell from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...
    def to_bytes(self) -> bytes:
        """
        Encodes cell into raw bytes.
        """
        ...
    def unpack(
        self,
        abi: List[Tuple[str, AbiParam]],
        abi_version: Optional[AbiVersion],
        allow_partial: Optional[bool]
    ) -> Dict[str, Any]: 
        """
        Unpack values using the provided ABI.

        :param abi: ABI structure.
        :param abi_version: optional ABI version.
        :param allow_partial: whether to unpack only the prefix of the cell. (`False` by default).
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class Clock:
    """
    Time context.

    :param offset: optional offset in milliseconds.
    """

    offset: int
    """Clock offset in milliseconds."""

    @classmethod
    def __init__(cls, offset: Optional[int]) -> None: ...
    def now_ms(self) -> int:
        """Returns current timestamp in milliseconds."""
        ...
    def now_sec(self) -> int:
        """Returns current timestamp in seconds."""
        ...

class ContractAbi:
    """Parsed contract ABI."""

    abi_version: AbiVersion
    """
    TVM ABI version.

    :param abi: a string with JSON ABI description.
    """

    @classmethod
    def __init__(cls, abi: str) -> None: ...
    def decode_init_data(self, data: Cell) -> Tuple[Optional[PublicKey], Dict[str, Any]]:
        """
        Decodes initial contract data using the contract ABI.

        :param data: initial contract data (from TVC).
        """
        ...
    def decode_transaction(self, transaction: Transaction) -> Optional[FunctionCallFull]:
        """
        Decodes function call and events from the specified transaction.

        :param transaction: transaction to decode.
        """
        ...
    def decode_transaction_events(self, transaction: Transaction) -> List[Tuple[EventAbi, Dict[str, Any]]]:
        """
        Decodes only events from the specified transaction.

        :param transaction: transaction to decode.
        """
        ...
    def encode_init_data(
        self,
        data: Dict[str, Any],
        public_key: Optional[PublicKey],
        existing_data: Optional[Cell]
    ) -> Cell:
        """
        Encodes initial contract data using the specified values and public key.

        :param data: initial data values.
        :param public_key: updates public key if specified.
        :param existing_data: updates the specified initial contract data.
        """
        ...
    def get_event(self, name: str) -> Optional[EventAbi]:
        """
        Searches for the event ABI with the specified name.

        :param name: event name.
        """
        ...
    def get_function(self, name: str) -> Optional[EventAbi]:
        """
        Searches for the function ABI with the specified name.

        :param name: function name.
        """
        ...

class EventAbi:
    """Parsed event ABI."""

    abi_version: AbiVersion
    """TVM ABI version."""

    id: int
    """Event id."""

    name: str
    """Event name."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def decode_message(self, message: Message) -> Dict[str, Any]:
        """
        Tries to decode event data from the specified message using event ABI.

        :param message: message to decode.
        """
        ...
    def decode_message_body(self, message_body: Cell) -> Dict[str, Any]:
        """
        Tries to decode event data from the specified message body using event ABI.

        :param message_body: message body to decode.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class ExecutionOutput:
    exit_code: int
    """Exit code from the compute phase."""

    output: Optional[Dict[str, Any]]
    """Parsed output in case of successful execution."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class ExternalInMessageHeader(MessageHeader):
    """External incoming message header."""

    dst: Address
    """Message destination."""

    import_fee: int
    """Import fee in nano EVERs"""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class ExternalOutMessageHeader(MessageHeader):
    """External outgoing message header."""

    created_at: int
    """A unix timestamp when the message was created."""

    created_lt: int
    """A logical time when the message was created."""

    src: Address
    """Message source."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class FunctionAbi:
    """Parsed function ABI."""

    abi_version: AbiVersion
    """TVM ABI version."""

    input_id: int
    """Input id."""

    name: str
    """Function name."""

    output_id: int
    """Output id."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def call(
        self,
        account_state: AccountState,
        input: Dict,
        responsible: Optional[bool],
        clock: Optional[Clock]
    ) -> ExecutionOutput:
        """
        Runs this function as a getter.

        :param account_state: a state of existing account which will be used for execution.
        :param input: function intput.
        :param responsible: whether to run this getter as responsible.
        :param clock: optional clock to modify execution timestamp.
        """
        ...
    def decode_input(
        self,
        message_body: Cell,
        internal: bool,
        allow_partial: Optional[bool]
    ) -> Dict[str, Any]:
        """
        Decodes message body as input using the function ABI.

        :param message_body: message body to decode.
        :param internal: whether this body is from an internal message.
        :param allow_partial: whether to decode only the prefix of the body.
        """
        ...
    def decode_output(self, message_body: Cell, allow_partial: Optional[bool]) -> Dict[str, Any]:
        """
        Decodes message body as output using the function ABI.

        :param message_body: message body to decode.
        :param allow_partial: whether to decode only the prefix of the body.
        """
        ...
    def decode_transaction(self, transaction: Transaction) -> FunctionCall:
        """
        Decodes transaction as a function call using the function ABI.

        :param transaction: transaction to decode.
        """
        ...
    def encode_external_input(
        self,
        input: Dict[str, Any],
        public_key: Optional[PublicKey],
        timeout: Optional[int],
        address: Optional[Address],
        clock: Optional[Clock]
    ) -> UnsignedBody:
        """
        Encodes external function input using the function ABI.

        :param input: function arguments.
        :param public_key: public key which will be used for signature.
        :param timeout: expiration timeout.
        :param address: destination account address (for ABI 2.3).
        :param clock: optional clock to modify used timestamp.
        """
        ...
    def encode_external_message(
        self,
        dst: Address,
        input: Dict[str, Any],
        public_key: Optional[PublicKey],
        state_init: Optional[StateInit],
        timeout: Optional[int],
        clock: Optional[Clock]
    ) -> UnsignedExternalMessage:
        """
        Encodes external message using the function ABI.

        :param dst: destination account address.
        :param input: function arguments.
        :param public_key: public key which will be used for signature.
        :param state_init: optional state init.
        :param timeout: expiration timeout.
        :param clock: optional clock to modify used timestamp.
        """
        ...
    def encode_internal_input(self, input: Dict[str, Any]) -> Cell:
        """
        Encodes internal function input using the function ABI.

        :param input: function arguments.
        """
        ...
    def encode_internal_message(
        self,
        input: Dict[str, Any],
        value: int,
        bounce: bool,
        dst: Address,
        src: Optional[Address],
        state_init: Optional[StateInit],
    ) -> Message:
        """
        Encodes internal message using the function ABI.

        :param input: function arguments.
        :param value: attached amount of nano EVERs.
        :param bounce: whether to return the amount in case of error.
        :param dst: destination account address.
        :param src: source account address.
        :param state_init: optional state init.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class FunctionCall:
    """Parsed function call."""

    input: Dict[str, Any]
    """Parsed function input."""

    output: Dict[str, Any]
    """Parsed function output."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class FunctionCallFull(FunctionCall):
    """Extended parsed function cell."""

    events: List[Tuple[EventAbi, Dict[str, Any]]]
    """Parsed events"""

    function: FunctionAbi
    """ABI object of the parsed function"""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class GqlTransport(Transport):
    """
    GraphQl transport.

    :param endpoints: a list of gql endpoints.
    :param clock: optional clock to modify timestamp.
    :param local: whether the connection is with local node.
    """

    @classmethod
    def __init__(
        cls,
        endpoints: List[str],
        clock: Optional[Clock],
        local: Optional[bool]
    ) -> None: ...

class InternalMessageHeader(MessageHeader):
    bounce: bool
    bounced: bool
    created_at: int
    created_lt: int
    dst: Address
    fwd_fee: int
    ihr_disabled: bool
    ihr_fee: int
    src: Address
    value: int
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class JrpcTransport(Transport):
    """
    JRPC trsnport.

    :param endpoint: JRPC endpoint.
    :param clock: optional clock to modify timestamp.
    """

    @classmethod
    def __init__(cls, endpoint: str, clock: Optional[Clock]) -> None: ...

class KeyPair:
    """
    Ed25519 key pair.

    :param secret: 32 bytes of secret.
    """

    public_key: PublicKey
    """Corresponding public key."""

    @classmethod
    def __init__(cls, secret: bytes) -> None: ...
    def check_signature(self, data: bytes, signature: Signature) -> bool:
        """
        Returns `True` if the signature is correct.

        :param data: signed message.
        :param signature: signature to check.
        """
        ...
    def generate(self) -> KeyPair:
        """Generates a new keypair."""
        ...
    def sign(self, data: bytes, signature_id: Optional[int]) -> Signature:
        """
        Signs a hash of the specified data.

        :param data: data to sign.
        :param signature_id: optional signature id.
        """
        ...
    def sign_raw(self, data: bytes, signature_id: Optional[int]) -> Signature:
        """
        Signs data as is.

        :param data: data to sign.
        :param signature_id: optional signature id.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class LegacySeed(Seed):
    """
    Legacy seed phase.

    :param phrase: a string with 24 words.
    """

    @classmethod
    def __init__(cls, phrase: str) -> None: ...
    def derive(self) -> KeyPair: ...
    def generate(self) -> LegacySeed:
        """
        Generates a new legacy seed.
        """
        ...

class Message:
    """
    Blockchain message.
    """

    body: Optional[Cell]
    """Optional message body."""

    bounced: bool
    """Whether this message was bounced."""

    created_at: int
    """A unix timestamp when this message was created. (always 0 for `ExternalIn`)."""

    created_lt: int
    """A logical timestamp when this message was created. (always 0 for `ExternalIn`)."""

    dst: Optional[Address]
    """Destination address. (None for `ExternalOut`)."""

    hash: bytes
    """The hash of the root message cell."""

    header: MessageHeader
    """Message header"""

    is_external_in: bool
    """Whether this message is `ExternalIn`."""

    is_external_out: bool
    """Whether this message is `ExternalOut`."""

    is_internal: bool
    """Whether this message is `Internal`."""

    src: Optional[Address]
    """Source address. (None for ExternalIn)."""

    state_init: Optional[StateInit]
    """Optional state init."""

    type: MessageType
    """Message type."""

    value: int
    """Attached amount of nano EVERs. (always 0 for non `Internal`)."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def build_cell(self) -> Cell:
        """
        Encodes message into a new cell.
        """
        ...
    def decode(self, value: str, encoding: Optional[str]) -> Message:
        """
        Decodes the message from the encoded BOC.

        :param value: a string with encoded BOC.
        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...
    def encode(self, encoding: Optional[str]) -> str:
        """
        Encodes the message into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...
    def from_bytes(self, bytes: bytes) -> Message:
        """
        Decodes message from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...
    def to_bytes(self) -> bytes:
        """
        Encodes message into raw bytes.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class MessageHeader:
    """Base message header."""

    type: MessageType
    """Message type."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class MessageType:
    """Message type."""

    ExternalIn: ClassVar[MessageType] = ...
    """External incoming message. (External calls)."""

    ExternalOut: ClassVar[MessageType] = ...
    """External outgoing message. (Events)."""

    Internal: ClassVar[MessageType] = ...
    """Internal message. (Messages between accounts)."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __int__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class PublicKey:
    """
    Ed25519 public key.

    :param value: a string with encoded public key.
    :param encoding: encoding of the value. `hex` (default) or `base64`.
    """

    @classmethod
    def __init__(cls, value: str, encoding: Optional[str]) -> None: ...
    def check_signature(self, data: bytes, signature: Signature) -> bool:
        """
        Returns `True` if the signature is correct.

        :param data: signed message.
        :param signature: signature to check.
        """
        ...
    def encode(self, encoding: Optional[str]) -> str:
        """
        Encodes public key into string.

        :param encoding: encoding of the value. `hex` (default) or `base64`.
        """
        ...
    def from_bytes(self, bytes: bytes) -> PublicKey:
        """
        Tries to construct a public key from raw bytes.

        :param bytes: 32 bytes of public key.
        """
        ...
    def to_bytes(self) -> bytes:
        """Converts public key into raw bytes."""
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class Seed:
    """Base seed"""

    word_count: int
    """Number of words in phrase."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class Signature:
    """
    Ed25519 signature.

    :param value: a string with encoded signature.
    :param encoding: encoding of the value. `hex` (default) of `base64`.
    """

    @classmethod
    def __init__(cls, value: str, encoding: Optional[str]) -> None: ...
    def encode(self, encoding: Optional[str]) -> str:
        """
        Encodes signature into string.

        :param encoding: encoding of the value. `hex` (default) of `base64`.
        """
        ...
    def from_bytes(self, bytes: bytes) -> Signature:
        """
        Tries to construct a signature from raw bytes.

        :param bytes: 64 bytes of signature.
        """
        ...
    def to_bytes(self) -> bytes:
        """Converts signature into raw bytes."""
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class SignedExternalMessage(Message):
    """
    External message with an additional expiration param.
    """
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def split(self) -> Tuple[Message, int]:
        """Splits into inner message and expiration timestamp."""
        ...

class StateInit:
    """
    Contract code and data.

    :param code: optional contract code.
    :param data: optional contract data.
    """

    code: Optional[Cell]
    """Optional contract code."""

    code_hash: Optional[bytes]
    """Optional code hash."""

    data: Optional[Cell]
    """Optional contract data."""

    @classmethod
    def __init__(cls, code: Optional[Cell], data: Optional[Cell]) -> None: ...
    def build_cell(self) -> Cell:
        """
        Creates a new cell with StateInit.
        """
        ...
    def compute_address(self, workchain: Optional[int]) -> Address:
        """
        Computes an address for this StateInit.

        :param workchain: optional workchain. (0 by default).
        """
        ...
    def encode(self, encoding: Optional[str]) -> str:
        """
        Encodes the state init into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...
    def from_bytes(self, bytes: bytes) -> StateInit:
        """
        Decodes state init from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...
    def get_code_salt(self) -> Optional[Cell]:
        """
        Tries to extract the code salt
        """
        ...
    def set_code_salt(self, salt: Cell):
        """
        Tries to update the code salt.

        :param salt: a cell with code salt.
        """
        ...
    def to_bytes(self) -> bytes:
        """
        Encodes state init into raw bytes.
        """
        ...

class StorageUsed:
    """
    Account storage stats.
    """

    bits: int
    """Number of bits occupied by this account."""

    cells: int
    """Number of cells occupied by this account."""

    public_cells: int
    """Number of public cells (libraries) provided by this account."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class Transaction:
    """Blockchain transaction."""

    aborted: bool
    """Whether this transaction was not successfull."""

    account: bytes
    """Account part of the address."""

    action_phase: Optional[TransactionActionPhase]
    """Optional action phase."""

    bounce_phase: Optional[TransactionBouncePhase]
    """Optional bounce phase."""

    compute_phase: Optional[TransactionComputePhase]
    """Optional compute phase."""

    credit_first: bool
    """Whether the account balance was updated before the credit storage phase."""

    credit_phase: Optional[TransactionCreditPhase]
    """Optional credit phase."""

    destroyed: bool
    """Whether the account was destroyed during this transaction."""

    end_status: AccountStatus
    """Account status after this transaction."""

    has_in_msg: bool
    """Whether this transaction has an incoming message. (`True` for ordinary transactions)."""

    has_out_msgs: bool
    """WHether this transaction has any outgoing message."""

    hash: bytes
    """Hash of the root cell."""

    in_msg_hash: Optional[bytes]
    """Hash of the incoming message."""

    lt: int
    """Logical time when the transaction was created."""

    now: int
    """Unix timestamp when the transaction was created."""

    orig_status: AccountStatus
    """Account status before this transaction."""

    out_msgs_len: int
    """Number of outgoing messages."""

    prev_trans_hash: bytes
    """A hash of the previous transaction."""

    prev_trans_lt: int
    """A logical time of the prefious transaction."""

    storage_phase: Optional[TransactionStoragePhase]
    """Optional storage phase."""

    total_fees: int
    """A total amount of fees in nano EVERs."""

    type: TransactionType
    """Transaction type."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def get_in_msg(self) -> Message:
        """
        Loads internal message.
        """
        ...
    def get_out_msgs(self) -> List[Message]:
        """
        Loads outgoing messages.
        """
        ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class TransactionActionPhase:
    action_list_hash: bytes
    messages_created: int
    no_funds: bool
    result_arg: Optional[int]
    result_code: int
    skipped_actions: int
    special_actions: int
    status_change: AccountStatusChange
    success: bool
    total_action_fees: Optional[int]
    total_actions: int
    total_fwd_fees: Optional[int]
    valid: bool
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class TransactionBouncePhase:
    fwd_fees: int
    msg_fees: int
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class TransactionComputePhase:
    account_activated: bool
    exit_arg: Optional[int]
    exit_code: int
    gas_credit: Optional[int]
    gas_fees: int
    gas_limit: int
    gas_used: int
    mode: int
    msg_state_used: bool
    success: bool
    vm_final_state_hash: bytes
    vm_init_state_hash: bytes
    vm_steps: int
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class TransactionCreditPhase:
    credit: int
    due_fees_collected: Optional[int]
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class TransactionExecutor:
    """
    Local transaction executor.

    :param config: blockchain config which will be used during execution.
    :param clock: optional clock to modify used timestamp.
    :param check_signature: whether to check signature.
    """

    check_signature: bool
    """Whether to require valid signatures."""

    @classmethod
    def __init__(cls, config: BlockchainConfig, clock: Optional[Clock], check_signature: Optional[bool]) -> None: ...
    def execute(
        self,
        message: Message,
        account: Optional[AccountState]
    ) -> Tuple[Transaction, Optional[AccountState]]:
        """
        Executes the specified message on account state.

        :param message: message to execute.
        :param account: account state. (`None` for non-existing).
        """
        ...

class TransactionStoragePhase:
    status_change: bool
    storage_fees_collected: Optional[int]
    storage_fees_due: AccountStatusChange
    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...

class TransactionType:
    Ordinary: ClassVar[TransactionType] = ...
    """Ordinary transaction."""

    TickTock: ClassVar[TransactionType] = ...
    """Special TickTock transaction. (Without incoming message)."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def __eq__(self, other) -> Any: ...
    def __ge__(self, other) -> Any: ...
    def __gt__(self, other) -> Any: ...
    def __hash__(self) -> Any: ...
    def __int__(self) -> Any: ...
    def __le__(self, other) -> Any: ...
    def __lt__(self, other) -> Any: ...
    def __ne__(self, other) -> Any: ...

class Transport:
    """Base transport"""

    clock: Clock
    """Time context."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    async def check_connection(self):
        """Checks the connection."""
        ...
    async def get_account_state(self, address: Address) -> Optional[AccountState]:
        """
        Fetches an account state for the specified address.

        :param address: account address.
        """
        ...
    async def get_accounts_by_code_hash(
        self,
        code_hash: bytes,
        continuation: Optional[Address],
        limit: Optional[int]
    ) -> List[Address]:
        """
        Fetches a list of address of accounts with the specified code hash.

        :param code_hash: code hash.
        :param continuation: optional account address from the previous batch.
        :param limit: max number of items in response.
        """
        ...
    async def get_blockchain_config(self, force: Optional[bool]) -> BlockchainConfig:
        """
        Fetches the latest blockchain config.

        :param force: whether to ignore cache.
        """
        ...
    async def get_dst_transaction(self, message_hash: bytes) -> Optional[Transaction]:
        """
        Searches for a transaction by the hash of incoming message.

        :param message_hash: a hash of the incoming message.
        """
        ...
    async def get_signature_id(self) -> Optional[int]:
        """Fetches signature id for the selected network."""
        ...
    async def get_transaction(self, transaction_hash: bytes) -> Optional[Transaction]:
        """
        Fetches the transaction by hash.

        :param transaction_hash: transaction hash.
        """
        ...
    async def get_transactions(
        self,
        address: Address,
        lt: Optional[int],
        limit: Optional[int]
    ) -> List[Transaction]:
        """
        Fetches a transactions batch for the specified account.

        :param address: account address.
        :param lt: optoinal logical time of the latest transaction.
        :param limit: max number of items in response.
        """
        ...
    async def send_external_message(self, message: SignedExternalMessage) -> Optional[Transaction]:
        """
        Sends an external message to the network and waits until the transaction.

        :param message: signed external message.
        """
        ...

class UnsignedBody:
    """Unsigned function input."""

    expire_at: int
    """Expiration unix timestamp."""

    hash: bytes
    """A hash to sign."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def sign(self, keypair: KeyPair, signature_id: Optional[int]) -> Cell:
        """
        Signs function input with the specified keypair and signature id.

        :param keypair: signer keypair.
        :param signature_id: optional signature id.
        """
        ...
    def with_fake_signature(self) -> Cell:
        """Inserts a fake signature into the body."""
        ...
    def with_signature(self, signature: Signature) -> Cell:
        """
        Inserts a signature into the body.

        :param signature: a valid ed25519 signature.
        """
        ...
    def without_signature(self) -> Cell:
        """Creates an input without a signature."""
        ...

class UnsignedExternalMessage:
    """Unsigned external message with function intput."""

    expire_at: int
    """Expiration unix timestamp."""

    hash: bytes
    """A hash to sign."""

    state_init: Optional[StateInit]
    """Optional state init."""

    @classmethod
    def __init__(cls, *args, **kwargs) -> None: ...
    def sign(self, keypair: KeyPair, signature_id: Optional[int]) -> SignedExternalMessage:
        """
        Signs function input with the specified keypair and signature id.

        :param keypair: signer keypair.
        :param signature_id: optional signature id.
        """
        ...
    def with_fake_signature(self) -> SignedExternalMessage:
        """Inserts a fake signature into the body."""
        ...
    def with_signature(self, *args, **kwargs) -> SignedExternalMessage:
        """
        Inserts a signature into the body.

        :param signature: a valid ed25519 signature.
        """
        ...
    def without_signature(self) -> SignedExternalMessage:
        """Creates an input without a signature."""
        ...

