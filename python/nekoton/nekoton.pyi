from typing import Any, ClassVar, Optional, List, Tuple, Dict


#########
## ABI ##
#########
# <editor-fold desc="ABI">


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
    def __init__(cls, config: BlockchainConfig, clock: Optional[Clock] = None,
                 check_signature: Optional[bool] = None) -> None: ...

    def execute(
            self,
            message: Message,
            account: Optional[AccountState] = None
    ) -> Tuple[Transaction, Optional[AccountState]]:
        """
        Executes the specified message on account state.

        :param message: message to execute.
        :param account: account state. (`None` for non-existing).
        """
        ...


class ContractAbi:
    """
    Parsed contract ABI.

    :param abi: a string with JSON ABI description.
    """

    abi_version: AbiVersion
    """TVM ABI version."""

    @classmethod
    def __init__(cls, abi: str) -> None: ...

    def get_function(self, name: str) -> Optional[FunctionAbi]:
        """
        Searches for the function ABI with the specified name.

        :param name: function name.
        """
        ...

    def get_event(self, name: str) -> Optional[EventAbi]:
        """
        Searches for the event ABI with the specified name.

        :param name: event name.
        """
        ...

    def encode_init_data(
            self,
            data: Dict[str, Any],
            public_key: Optional[PublicKey] = None,
            existing_data: Optional[Cell] = None
    ) -> Cell:
        """
        Encodes initial contract data using the specified values and public key.

        :param data: initial data values.
        :param public_key: updates public key if specified.
        :param existing_data: updates the specified initial contract data.
        """
        ...

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


class FunctionAbi:
    """Parsed function ABI."""

    abi_version: AbiVersion
    """TVM ABI version."""

    name: str
    """Function name."""

    input_id: int
    """Input id."""

    output_id: int
    """Output id."""

    def call(
            self,
            account_state: AccountState,
            input: Dict,
            responsible: Optional[bool] = None,
            clock: Optional[Clock] = None,
    ) -> ExecutionOutput:
        """
        Runs this function as a getter.

        :param account_state: a state of existing account which will be used for execution.
        :param input: function intput.
        :param responsible: whether to run this getter as responsible.
        :param clock: optional clock to modify execution timestamp.
        """
        ...

    def encode_external_message(
            self,
            dst: Address,
            input: Dict[str, Any],
            public_key: Optional[PublicKey],
            state_init: Optional[StateInit] = None,
            timeout: Optional[int] = None,
            clock: Optional[Clock] = None
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

    def encode_external_input(
            self,
            input: Dict[str, Any],
            public_key: Optional[PublicKey],
            timeout: Optional[int] = None,
            address: Optional[Address] = None,
            clock: Optional[Clock] = None
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

    def encode_internal_message(
            self,
            input: Dict[str, Any],
            value: int,
            bounce: bool,
            dst: Address,
            src: Optional[Address] = None,
            state_init: Optional[StateInit] = None,
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

    def encode_internal_input(self, input: Dict[str, Any]) -> Cell:
        """
        Encodes internal function input using the function ABI.

        :param input: function arguments.
        """
        ...

    def decode_transaction(self, transaction: Transaction) -> FunctionCall:
        """
        Decodes transaction as a function call using the function ABI.

        :param transaction: transaction to decode.
        """
        ...

    def decode_input(
            self,
            message_body: Cell,
            internal: bool,
            allow_partial: Optional[bool] = None
    ) -> Dict[str, Any]:
        """
        Decodes message body as input using the function ABI.

        :param message_body: message body to decode.
        :param internal: whether this body is from an internal message.
        :param allow_partial: whether to decode only the prefix of the body.
        """
        ...

    def decode_output(self, message_body: Cell, allow_partial: Optional[bool] = None) -> Dict[str, Any]:
        """
        Decodes message body as output using the function ABI.

        :param message_body: message body to decode.
        :param allow_partial: whether to decode only the prefix of the body.
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


class FunctionCall:
    """Parsed function call."""

    input: Dict[str, Any]
    """Parsed function input."""

    output: Dict[str, Any]
    """Parsed function output."""


class FunctionCallFull(FunctionCall):
    """Extended parsed function cell."""

    events: List[Tuple[EventAbi, Dict[str, Any]]]
    """Parsed events"""

    function: FunctionAbi
    """ABI object of the parsed function"""


class EventAbi:
    """Parsed event ABI."""

    abi_version: AbiVersion
    """TVM ABI version."""

    name: str
    """Event name."""

    id: int
    """Event id."""

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


class SignedExternalMessage(Message):
    """
    External message with an additional expiration param.
    """

    def split(self) -> Tuple[Message, int]:
        """Splits into inner message and expiration timestamp."""
        ...


class UnsignedExternalMessage:
    """Unsigned external message with function intput."""

    hash: bytes
    """A hash to sign."""

    expire_at: int
    """Expiration unix timestamp."""

    state_init: Optional[StateInit]
    """Optional state init."""

    def sign(self, keypair: KeyPair, signature_id: Optional[int]) -> SignedExternalMessage:
        """
        Signs function input with the specified keypair and signature id.

        :param keypair: signer keypair.
        :param signature_id: optional signature id.
        """
        ...

    def with_signature(self, signature: Signature) -> SignedExternalMessage:
        """
        Inserts a signature into the body.

        :param signature: a valid ed25519 signature.
        """
        ...

    def with_fake_signature(self) -> SignedExternalMessage:
        """Inserts a fake signature into the body."""
        ...

    def without_signature(self) -> SignedExternalMessage:
        """Creates an input without a signature."""
        ...


class UnsignedBody:
    """Unsigned function input."""

    hash: bytes
    """A hash to sign."""

    expire_at: int
    """Expiration unix timestamp."""

    def sign(self, keypair: KeyPair, signature_id: Optional[int]) -> Cell:
        """
        Signs function input with the specified keypair and signature id.

        :param keypair: signer keypair.
        :param signature_id: optional signature id.
        """
        ...

    def with_signature(self, signature: Signature) -> Cell:
        """
        Inserts a signature into the body.

        :param signature: a valid ed25519 signature.
        """
        ...

    def with_fake_signature(self) -> Cell:
        """Inserts a fake signature into the body."""
        ...

    def without_signature(self) -> Cell:
        """Creates an input without a signature."""
        ...


class AbiParam:
    """
    Base ABI type.
    """


class AbiUint(AbiParam):
    """
    A class for `uintN` ABI type.

    :param size: uint size in bits.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...


class AbiInt(AbiParam):
    """
    A class for an `intN` ABI type.

    :param size: int size in bits.
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


class AbiVarInt(AbiParam):
    """
    A class for `varintN` ABI type.

    :param size: varint size in bytes.
    """

    @classmethod
    def __init__(cls, size: int) -> None: ...


class AbiBool(AbiParam):
    """
    A class for a `bool` ABI type.
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


class AbiArray(AbiParam):
    """
    A class for an `T[]` ABI type.

    :param value_type: the ABI type of array items.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...


class AbiFixedArray(AbiParam):
    """
    A class for a `T[N]` ABI type.

    :param value_type: the ABI type of array items.
    :param len: number of array items.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam, len: int) -> None: ...


class AbiCell(AbiParam):
    """
    A class for a `cell` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...


class AbiMap(AbiParam):
    """
    A class for an `map(K, V)` ABI type.

    :param key_type: the ABI type of mapping keys.
    :param value_type: the ABI type of mapping values.
    """

    @classmethod
    def __init__(cls, key_type: AbiParam, value_type: AbiParam) -> None: ...


class AbiAddress(AbiParam):
    """
    A class for a `address` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...


class AbiBytes(AbiParam):
    """
    A class for a `bytes` ABI type.
    """

    @classmethod
    def __init__(cls) -> None: ...


class AbiFixedBytes(AbiParam):
    """
    A class for a `fixedbytesN` ABI type.

    :param len: number of bytes.
    """

    @classmethod
    def __init__(cls, len: int) -> None: ...


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


class AbiOptional(AbiParam):
    """
    A class for an `optional(T)` ABI type.

    :param value_type: the ABI type of the inner value.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...


class AbiRef(AbiParam):
    """
    A class for a `ref(T)` ABI type.

    :param value_type: the ABI type of the inner value.
    """

    @classmethod
    def __init__(cls, value_type: AbiParam) -> None: ...


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


# </editor-fold>

############
## MODELS ##
############
# <editor-fold desc="MODELS">


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

    minter_address: Address
    """Address of the minter contract."""

    fee_collector_address: Address
    """Address of the fee collector contract."""

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


class AccountState:
    """
    A state of an existing account.
    """

    storage_used: StorageUsed
    """Storage usage statistics."""

    last_paid: int
    """The last time when storage phase was executed."""

    due_payment: Optional[int]
    """Optional account debt in nano EVERs."""

    last_trans_lt: int
    """The logical time of the last transaction."""

    balance: int
    """Account balance in nano EVERs."""

    status: AccountStatus
    """Account status."""

    state_init: Optional[StateInit]
    """StateInit for the active account."""

    frozen_state_hash: Optional[bytes]
    """A hash of the last known state for the frozen account."""


class StorageUsed:
    """
    Account storage stats.
    """

    cells: int
    """Number of cells occupied by this account."""

    bits: int
    """Number of bits occupied by this account."""

    public_cells: int
    """Number of public cells (libraries) provided by this account."""


class Transaction:
    """Blockchain transaction."""

    hash: bytes
    """Hash of the root cell."""

    type: TransactionType
    """Transaction type."""

    account: bytes
    """Account part of the address."""

    lt: int
    """Logical time when the transaction was created."""

    now: int
    """Unix timestamp when the transaction was created."""

    prev_trans_hash: bytes
    """A hash of the previous transaction."""

    prev_trans_lt: int
    """A logical time of the previous transaction."""

    orig_status: AccountStatus
    """Account status before this transaction."""

    end_status: AccountStatus
    """Account status after this transaction."""

    total_fees: int
    """A total amount of fees in nano EVERs."""

    has_in_msg: bool
    """Whether this transaction has an incoming message. (`True` for ordinary transactions)."""

    has_out_msgs: bool
    """Whether this transaction has any outgoing message."""

    out_msgs_len: int
    """Number of outgoing messages."""

    in_msg_hash: Optional[bytes]
    """Hash of the incoming message."""

    credit_first: bool
    """Whether the account balance was updated before the credit storage phase."""

    aborted: bool
    """Whether this transaction was not successful."""

    destroyed: bool
    """Whether the account was destroyed during this transaction."""

    storage_phase: Optional[TransactionStoragePhase]
    """Optional storage phase."""

    credit_phase: Optional[TransactionCreditPhase]
    """Optional credit phase."""

    compute_phase: Optional[TransactionComputePhase]
    """Optional compute phase."""

    action_phase: Optional[TransactionActionPhase]
    """Optional action phase."""

    bounce_phase: Optional[TransactionBouncePhase]
    """Optional bounce phase."""

    def get_in_msg(self) -> Message:
        """Loads an internal message."""
        ...

    def get_out_msgs(self) -> List[Message]:
        """Loads outgoing messages."""
        ...

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class TransactionStoragePhase:
    """Transaction storage phase."""

    status_change: AccountStatusChange
    """Account status change during this phase."""

    storage_fees_collected: int
    """Amount of collected storage fees in nano EVERs."""

    storage_fees_due: Optional[int]
    """Payed storage debt."""


class TransactionCreditPhase:
    """Transaction credit phase."""

    due_fees_collected: Optional[int]
    """Amount of collected storage fees in nano EVERs."""

    credit: int
    """Increased balance in nano EVERs."""


class TransactionComputePhase:
    """Transaction compute phase."""

    success: bool
    msg_state_used: bool
    account_activated: bool
    gas_fees: int
    gas_used: int
    gas_limit: int
    gas_credit: Optional[int]
    mode: int
    exit_code: int
    exit_arg: Optional[int]
    vm_steps: int
    vm_init_state_hash: bytes
    vm_final_state_hash: bytes


class TransactionActionPhase:
    """Transaction action phase."""

    success: bool
    valid: bool
    no_funds: bool
    status_change: AccountStatusChange
    total_fwd_fees: Optional[int]
    total_action_fees: Optional[int]
    result_code: int
    result_arg: Optional[int]
    total_actions: int
    special_actions: int
    skipped_actions: int
    messages_created: int
    action_list_hash: bytes


class TransactionBouncePhase:
    msg_fees: int
    fwd_fees: int


class TransactionType:
    Ordinary: ClassVar[TransactionType] = ...
    """Ordinary transaction."""

    TickTock: ClassVar[TransactionType] = ...
    """Special TickTock transaction. (Without incoming message)."""

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __int__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class AccountStatus:
    """
    Account status.
    """

    Active: ClassVar[AccountStatus] = ...
    """Active (deployed) account."""

    Frozen: ClassVar[AccountStatus] = ...
    """Frozen account. (Has no state, can be unfrozen or deleted)."""

    NotExists: ClassVar[AccountStatus] = ...
    """Account does not exist."""

    Uninit: ClassVar[AccountStatus] = ...
    """Account without a state."""

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

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __int__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class Message:
    """
    Blockchain message.
    """

    hash: bytes
    """The hash of the root message cell."""

    is_external_in: bool
    """Whether this message is `ExternalIn`."""

    is_external_out: bool
    """Whether this message is `ExternalOut`."""

    is_internal: bool
    """Whether this message is `Internal`."""

    type: MessageType
    """Message type."""

    header: MessageHeader
    """Message header"""

    created_at: int
    """A unix timestamp when this message was created. (always 0 for `ExternalIn`)."""

    created_lt: int
    """A logical timestamp when this message was created. (always 0 for `ExternalIn`)."""

    src: Optional[Address]
    """Source address. (None for ExternalIn)."""

    dst: Optional[Address]
    """Destination address. (None for `ExternalOut`)."""

    value: int
    """Attached amount of nano EVERs. (always 0 for non `Internal`)."""

    bounced: bool
    """Whether this message was bounced."""

    body: Optional[Cell]
    """Optional message body."""

    state_init: Optional[StateInit]
    """Optional state init."""

    @staticmethod
    def from_bytes(bytes: bytes) -> Message:
        """
        Decodes message from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...

    @staticmethod
    def decode(value: str, encoding: Optional[str] = None) -> Message:
        """
        Decodes the message from the encoded BOC.

        :param value: a string with encoded BOC.
        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...

    def encode(self, encoding: Optional[str] = None) -> str:
        """
        Encodes the message into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...

    def to_bytes(self) -> bytes:
        """Encodes message into raw bytes."""
        ...

    def build_cell(self) -> Cell:
        """Encodes message into a new cell."""
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


class InternalMessageHeader(MessageHeader):
    """Internal message header."""

    ihr_disabled: bool
    bounce: bool
    bounced: bool
    src: Address
    dst: Address
    value: int
    ihr_fee: int
    fwd_fee: int
    created_at: int
    created_lt: int


class ExternalInMessageHeader(MessageHeader):
    """External incoming message header."""

    dst: Address
    """Message destination."""

    import_fee: int
    """Import fee in nano EVERs"""


class ExternalOutMessageHeader(MessageHeader):
    """External outgoing message header."""

    created_at: int
    """A unix timestamp when the message was created."""

    created_lt: int
    """A logical time when the message was created."""

    src: Address
    """Message source."""


class MessageType:
    """Message type."""

    ExternalIn: ClassVar[MessageType] = ...
    """External incoming message. (External calls)."""

    ExternalOut: ClassVar[MessageType] = ...
    """External outgoing message. (Events)."""

    Internal: ClassVar[MessageType] = ...
    """Internal message. (Messages between accounts)."""

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __int__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class StateInit:
    """
    Contract code and data.

    :param code: optional contract code.
    :param data: optional contract data.
    """

    code_hash: Optional[bytes]
    """Optional code hash."""

    code: Optional[Cell]
    """Optional contract code."""

    data: Optional[Cell]
    """Optional contract data."""

    @staticmethod
    def from_bytes(bytes: bytes) -> StateInit:
        """
        Decodes state init from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...

    @classmethod
    def __init__(cls, code: Optional[Cell], data: Optional[Cell]) -> None: ...

    def set_code_salt(self, salt: Cell):
        """
        Tries to update the code salt.

        :param salt: a cell with code salt.
        """
        ...

    def get_code_salt(self) -> Optional[Cell]:
        """
        Tries to extract the code salt
        """
        ...

    def compute_address(self, workchain: Optional[int] = None) -> Address:
        """
        Computes an address for this StateInit.

        :param workchain: optional workchain. (0 by default).
        """
        ...

    def encode(self, encoding: Optional[str] = None) -> str:
        """
        Encodes the state init into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...

    def to_bytes(self) -> bytes:
        """Encodes state init into raw bytes."""
        ...

    def build_cell(self) -> Cell:
        """Creates a new cell with StateInit."""
        ...


class Address:
    """
    Account address (`StdAddr`).

    :param addr: a string with raw address.
    """

    account: bytes
    """Hash of the initial state."""

    workchain: int
    """Address workchain."""

    @staticmethod
    def validate(addr: str) -> bool:
        """
        Checks whether the specified string is a valid address.

        :param addr: a string with raw address.
        """
        ...

    @staticmethod
    def from_parts(workchain: int, account: bytes) -> Any:
        """
        Creates an address from parts.

        :param workchain: address workchain.
        :param account: hash of the initial state.
        """
        ...

    @classmethod
    def __init__(cls, addr: str) -> None: ...

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class Cell:
    """
    A container with up to 1023 bits of data and up to 4 children.
    """

    repr_hash: bytes
    """Representation hash of the cell."""

    @staticmethod
    def from_bytes(bytes: bytes) -> Cell:
        """
        Decodes cell from raw bytes.

        :param bytes: raw bytes with BOC.
        """
        ...

    @staticmethod
    def build(
            abi: List[Tuple[str, AbiParam]],
            value: Dict[str, Any],
            abi_version: Optional[AbiVersion] = None,
    ) -> Cell:
        """
        Packs values into cell using the provided ABI.

        :param abi: ABI structure.
        :param value: a dictionary with corresponding values.
        :param abi_version: optional ABI version.
        """
        ...

    @staticmethod
    def decode(value: str, encoding: Optional[str] = None) -> Cell:
        """
        Decodes the cell from the encoded BOC.

        :param value: a string with encoded BOC.
        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...

    @classmethod
    def __init__(cls) -> None: ...

    def encode(self, encoding: Optional[str] = None) -> str:
        """
        Encodes the cell into BOC.

        :param encoding: encoding type. `base64` (default) or `hex`.
        """
        ...

    def to_bytes(self) -> bytes:
        """Encodes cell into raw bytes."""
        ...

    def unpack(
            self,
            abi: List[Tuple[str, AbiParam]],
            abi_version: Optional[AbiVersion] = None,
            allow_partial: Optional[bool] = None,
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


# </editor-fold>

###############
## TRANSPORT ##
###############
# <editor-fold desc="TRANSPORT">


class Transport:
    """Base transport"""

    clock: Clock
    """Time context."""

    async def check_connection(self):
        """Checks the connection."""
        ...

    async def send_external_message(self, message: SignedExternalMessage) -> Optional[Transaction]:
        """
        Sends an external message to the network and waits until the transaction.

        :param message: signed external message.
        """
        ...

    async def get_signature_id(self) -> Optional[int]:
        """Fetches signature id for the selected network."""
        ...

    async def get_blockchain_config(self, force: Optional[bool] = None) -> BlockchainConfig:
        """
        Fetches the latest blockchain config.

        :param force: whether to ignore cache.
        """
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
            continuation: Optional[Address] = None,
            limit: Optional[int] = None,
    ) -> List[Address]:
        """
        Fetches a list of address of accounts with the specified code hash.

        :param code_hash: code hash.
        :param continuation: optional account address from the previous batch.
        :param limit: max number of items in response.
        """
        ...

    async def get_transaction(self, transaction_hash: bytes) -> Optional[Transaction]:
        """
        Fetches the transaction by hash.

        :param transaction_hash: transaction hash.
        """
        ...

    async def get_dst_transaction(self, message_hash: bytes) -> Optional[Transaction]:
        """
        Searches for a transaction by the hash of incoming message.

        :param message_hash: a hash of the incoming message.
        """
        ...

    async def get_transactions(
            self,
            address: Address,
            lt: Optional[int] = None,
            limit: Optional[int] = None,
    ) -> List[Transaction]:
        """
        Fetches a transactions batch for the specified account.

        :param address: account address.
        :param lt: optoinal logical time of the latest transaction.
        :param limit: max number of items in response.
        """
        ...


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
            clock: Optional[Clock] = None,
            local: Optional[bool] = None,
    ) -> None: ...


class JrpcTransport(Transport):
    """
    JRPC transport.

    :param endpoint: JRPC endpoint.
    :param clock: optional clock to modify timestamp.
    """

    @classmethod
    def __init__(cls, endpoint: str, clock: Optional[Clock] = None) -> None: ...


class Clock:
    """
    Time context.

    :param offset: optional offset in milliseconds.
    """

    offset: int
    """Clock offset in milliseconds."""

    @classmethod
    def __init__(cls, offset: Optional[int] = None) -> None: ...

    def now_sec(self) -> int:
        """Returns current timestamp in seconds."""
        ...

    def now_ms(self) -> int:
        """Returns current timestamp in milliseconds."""
        ...


# </editor-fold>

############
## CRYPTO ##
############
# <editor-fold desc="CRYPTO">


class PublicKey:
    """
    Ed25519 public key.

    :param value: a string with encoded public key.
    :param encoding: encoding of the value. `hex` (default) or `base64`.
    """

    @staticmethod
    def from_bytes(bytes: bytes) -> PublicKey:
        """
        Tries to construct a public key from raw bytes.

        :param bytes: 32 bytes of public key.
        """
        ...

    @classmethod
    def __init__(cls, value: str, encoding: Optional[str] = None) -> None: ...

    def check_signature(self, data: bytes, signature: Signature) -> bool:
        """
        Returns `True` if the signature is correct.

        :param data: signed message.
        :param signature: signature to check.
        """
        ...

    def encode(self, encoding: Optional[str] = None) -> str:
        """
        Encodes public key into string.

        :param encoding: encoding of the value. `hex` (default) or `base64`.
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


class KeyPair:
    """
    Ed25519 key pair.

    :param secret: 32 bytes of secret.
    """

    public_key: PublicKey
    """Corresponding public key."""

    @staticmethod
    def generate() -> KeyPair:
        """Generates a new keypair."""
        ...

    @classmethod
    def __init__(cls, secret: bytes) -> None: ...

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

    def check_signature(self, data: bytes, signature: Signature) -> bool:
        """
        Returns `True` if the signature is correct.

        :param data: signed message.
        :param signature: signature to check.
        """
        ...

    def __eq__(self, other) -> Any: ...

    def __ge__(self, other) -> Any: ...

    def __gt__(self, other) -> Any: ...

    def __hash__(self) -> Any: ...

    def __le__(self, other) -> Any: ...

    def __lt__(self, other) -> Any: ...

    def __ne__(self, other) -> Any: ...


class Signature:
    """
    Ed25519 signature.

    :param value: a string with encoded signature.
    :param encoding: encoding of the value. `hex` (default) of `base64`.
    """

    @staticmethod
    def from_bytes(bytes: bytes) -> Signature:
        """
        Tries to construct a signature from raw bytes.

        :param bytes: 64 bytes of signature.
        """
        ...

    @classmethod
    def __init__(cls, value: str, encoding: Optional[str] = None) -> None: ...

    def encode(self, encoding: Optional[str] = None) -> str:
        """
        Encodes signature into string.

        :param encoding: encoding of the value. `hex` (default) of `base64`.
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


class Seed:
    """Base seed."""

    word_count: int
    """Number of words in phrase."""


class LegacySeed(Seed):
    """
    Legacy seed phase.

    :param phrase: a string with 24 words.
    """

    @staticmethod
    def generate() -> LegacySeed:
        """Generates a new legacy seed."""
        ...

    @classmethod
    def __init__(cls, phrase: str) -> None: ...

    def derive(self) -> KeyPair:
        """Derives a key pair."""
        ...


class Bip39Seed(Seed):
    """
    BIP39 seed.

    :param phrase: a string with 12 words.
    """

    @staticmethod
    def generate() -> Bip39Seed:
        """Generates a random BIP39 seed."""
        ...

    @staticmethod
    def path_for_account(id: int) -> str:
        """
        Returns a default derivation path for the specified account number.

        :param id: account number.
        """
        ...

    @classmethod
    def __init__(cls, phrase: str) -> None: ...

    def derive(self, path: Optional[str] = None) -> KeyPair:
        """
        Derives a key pair using some derivation path.

        :param path: custom derivation path.
        """
        ...

# </editor-fold>
