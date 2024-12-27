use std::collections::VecDeque;
use std::sync::Arc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use ton_block::{Deserializable, GetRepresentationHash, Serializable};
use ton_types::IBitstring;

use crate::abi::{convert_tokens, parse_tokens, AbiParam, AbiVersion};
use crate::crypto::{PublicKey, Signature};
use crate::util::{make_hasher, py_none, Encoding, HandleError};

#[derive(Clone)]
#[pyclass]
pub struct BlockchainConfig(Arc<ton_executor::BlockchainConfig>);

#[pymethods]
impl BlockchainConfig {
    #[getter]
    fn global_id(&self) -> i32 {
        self.0.global_id()
    }

    #[getter]
    fn capabilities(&self) -> u64 {
        self.0.capabilites()
    }

    #[getter]
    fn signature_id(&self) -> Option<i32> {
        if self
            .0
            .has_capability(ton_block::GlobalCapabilities::CapSignatureWithId)
        {
            Some(self.0.global_id())
        } else {
            None
        }
    }

    #[getter]
    fn global_version(&self) -> u32 {
        self.0.global_version()
    }

    #[getter]
    fn config_address(&self) -> PyResult<Address> {
        let config = self.0.raw_config();
        let addr = config.config_address().handle_runtime_error()?;
        ton_block::MsgAddressInt::with_standart(None, -1, addr.into())
            .handle_runtime_error()
            .map(Address)
    }

    #[getter]
    fn elector_address(&self) -> PyResult<Address> {
        let config = self.0.raw_config();
        let addr = config.elector_address().handle_runtime_error()?;
        ton_block::MsgAddressInt::with_standart(None, -1, addr.into())
            .handle_runtime_error()
            .map(Address)
    }

    #[getter]
    fn minter_address(&self) -> PyResult<Address> {
        let config = self.0.raw_config();
        let addr = config.minter_address().handle_runtime_error()?;
        ton_block::MsgAddressInt::with_standart(None, -1, addr.into())
            .handle_runtime_error()
            .map(Address)
    }

    #[getter]
    fn fee_collector_address(&self) -> PyResult<Address> {
        let config = self.0.raw_config();
        let addr = config.fee_collector_address().handle_runtime_error()?;
        ton_block::MsgAddressInt::with_standart(None, -1, addr.into())
            .handle_runtime_error()
            .map(Address)
    }

    // TODO: add other params

    fn contains_param(&self, index: u32) -> PyResult<bool> {
        let config = &self.0.raw_config().config_params;
        let key = index
            .serialize()
            .and_then(ton_types::SliceData::load_cell)
            .unwrap();
        Ok(
            if let Some(value) = config.get(key).handle_runtime_error()? {
                value.remaining_references() != 0
            } else {
                false
            },
        )
    }

    fn get_raw_param(&self, index: u32) -> PyResult<Option<Cell>> {
        let config = &self.0.raw_config().config_params;
        let key = index
            .serialize()
            .and_then(ton_types::SliceData::load_cell)
            .unwrap();
        let value = config.get(key).handle_runtime_error()?;
        Ok(value.and_then(|slice| slice.reference_opt(0)).map(Cell))
    }

    fn build_params_dict_cell(&self) -> PyResult<Cell> {
        let config = self.0.raw_config();
        config
            .config_params
            .serialize()
            .handle_runtime_error()
            .map(Cell)
    }

    fn __repr__(&self) -> String {
        format!(
            "<BlockchainConfig global_id={} capabilities=0x{:016x}, global_version=0x{}>",
            self.global_id(),
            self.capabilities(),
            self.0.global_version()
        )
    }
}

impl From<ton_executor::BlockchainConfig> for BlockchainConfig {
    fn from(value: ton_executor::BlockchainConfig) -> Self {
        Self(Arc::new(value))
    }
}

impl AsRef<ton_executor::BlockchainConfig> for BlockchainConfig {
    #[inline]
    fn as_ref(&self) -> &ton_executor::BlockchainConfig {
        self.0.as_ref()
    }
}

#[pyclass]
pub struct AccountState(pub ton_block::AccountStuff);

#[pymethods]
impl AccountState {
    #[getter]
    fn storage_used(&self) -> StorageUsed {
        StorageUsed(self.0.storage_stat.used.clone())
    }

    #[getter]
    fn last_paid(&self) -> u32 {
        self.0.storage_stat.last_paid
    }

    #[getter]
    fn due_payment(&self) -> Option<Tokens> {
        self.0.storage_stat.due_payment.map(Tokens::from)
    }

    #[getter]
    fn last_trans_lt(&self) -> u64 {
        self.0.storage.last_trans_lt
    }

    #[getter]
    fn balance(&self) -> Tokens {
        self.0.storage.balance.grams.into()
    }

    #[getter]
    fn status(&self) -> AccountStatus {
        match &self.0.storage.state {
            ton_block::AccountState::AccountUninit => AccountStatus::Uninit,
            ton_block::AccountState::AccountActive { .. } => AccountStatus::Active,
            ton_block::AccountState::AccountFrozen { .. } => AccountStatus::Frozen,
        }
    }

    #[getter]
    fn state_init(&self) -> Option<StateInit> {
        match &self.0.storage.state {
            ton_block::AccountState::AccountActive { state_init } => {
                Some(StateInit(state_init.clone()))
            }
            _ => None,
        }
    }

    #[getter]
    fn frozen_state_hash<'a>(&self, py: Python<'a>) -> Option<&'a PyBytes> {
        match &self.0.storage.state {
            ton_block::AccountState::AccountFrozen { state_init_hash } => {
                Some(PyBytes::new(py, state_init_hash.as_slice()))
            }
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "<AccountState balance={}, {:?}>",
            self.balance(),
            self.status(),
        )
    }
}

#[pyclass]
pub struct StorageUsed(ton_block::StorageUsed);

#[pymethods]
impl StorageUsed {
    #[getter]
    fn cells(&self) -> u64 {
        self.0.cells.as_u64()
    }

    #[getter]
    fn bits(&self) -> u64 {
        self.0.bits.as_u64()
    }

    #[getter]
    fn public_cells(&self) -> u64 {
        self.0.public_cells.as_u64()
    }

    fn __repr__(&self) -> String {
        format!(
            "<StorageUsed cells={}, bits={}, public_cells={}>",
            self.cells(),
            self.bits(),
            self.public_cells()
        )
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Transaction(pub Arc<SharedTransaction>);

impl TryFrom<nt::transport::models::RawTransaction> for Transaction {
    type Error = PyErr;

    fn try_from(value: nt::transport::models::RawTransaction) -> Result<Self, Self::Error> {
        let descr = value.data.read_description().handle_runtime_error()?;
        Ok(Self(Arc::new(SharedTransaction {
            hash: value.hash,
            data: value.data,
            descr,
        })))
    }
}

impl TryFrom<ton_types::Cell> for Transaction {
    type Error = PyErr;

    fn try_from(value: ton_types::Cell) -> Result<Self, Self::Error> {
        let hash = value.repr_hash();
        let data = ton_block::Transaction::construct_from_cell(value).handle_runtime_error()?;
        let descr = data.read_description().handle_runtime_error()?;
        Ok(Self(Arc::new(SharedTransaction { data, descr, hash })))
    }
}

#[pymethods]
impl Transaction {
    #[staticmethod]
    fn from_bytes(mut bytes: &[u8]) -> PyResult<Self> {
        let cell = ton_types::deserialize_tree_of_cells(&mut bytes).handle_runtime_error()?;
        Self::try_from(cell)
    }

    #[staticmethod]
    fn from_cell(cell: &Cell) -> PyResult<Self> {
        Self::try_from(cell.0.clone())
    }

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(&bytes)
    }

    #[getter]
    pub fn hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.hash.as_slice())
    }

    #[getter]
    fn get_type(&self) -> PyResult<TransactionType> {
        match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(_) => Ok(TransactionType::Ordinary),
            ton_block::TransactionDescr::TickTock(descr) => {
                Ok(if descr.tt == ton_block::TransactionTickTock::Tick {
                    TransactionType::Tick
                } else {
                    TransactionType::Tock
                })
            }
            _ => Err(PyRuntimeError::new_err("Unsupported transaction type")),
        }
    }

    #[getter]
    pub fn account<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        let account = self.0.data.account_addr.get_bytestring_on_stack(0);
        PyBytes::new(py, &account)
    }

    #[getter]
    pub fn lt(&self) -> u64 {
        self.0.data.lt
    }

    #[getter]
    pub fn now(&self) -> u32 {
        self.0.data.now
    }

    #[getter]
    pub fn prev_trans_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.data.prev_trans_hash.as_slice())
    }

    #[getter]
    pub fn prev_trans_lt(&self) -> u64 {
        self.0.data.prev_trans_lt
    }

    #[getter]
    pub fn orig_status(&self) -> AccountStatus {
        self.0.data.orig_status.clone().into()
    }

    #[getter]
    pub fn end_status(&self) -> AccountStatus {
        self.0.data.end_status.clone().into()
    }

    #[getter]
    pub fn total_fees(&self) -> Tokens {
        self.0.data.total_fees.grams.into()
    }

    #[getter]
    pub fn has_in_msg(&self) -> bool {
        self.0.data.in_msg.is_some()
    }

    #[getter]
    pub fn has_out_msgs(&self) -> bool {
        self.0.data.outmsg_cnt > 0
    }

    #[getter]
    pub fn out_msgs_len(&self) -> usize {
        self.0.data.outmsg_cnt as usize
    }

    #[getter]
    pub fn in_msg_hash<'a>(&self, py: Python<'a>) -> Option<&'a PyBytes> {
        let msg_cell = self.0.data.in_msg_cell()?;
        Some(PyBytes::new(py, msg_cell.repr_hash().as_slice()))
    }

    #[getter]
    fn credit_first(&self) -> bool {
        let ton_block::TransactionDescr::Ordinary(descr) = &self.0.descr else {
            return false;
        };
        descr.credit_first
    }

    #[getter]
    fn aborted(&self) -> bool {
        self.0.descr.is_aborted()
    }

    #[getter]
    fn destroyed(&self) -> PyResult<bool> {
        match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(descr) => Ok(descr.destroyed),
            ton_block::TransactionDescr::TickTock(descr) => Ok(descr.destroyed),
            _ => Err(PyRuntimeError::new_err("Unsupported transaction type")),
        }
    }

    #[getter]
    fn storage_phase(&self) -> PyResult<Option<TransactionStoragePhase>> {
        match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(descr) => {
                Ok(descr.storage_ph.clone().map(TransactionStoragePhase))
            }
            ton_block::TransactionDescr::TickTock(descr) => {
                Ok(Some(TransactionStoragePhase(descr.storage.clone())))
            }
            _ => Err(PyRuntimeError::new_err("Unsupported transaction type")),
        }
    }

    #[getter]
    fn credit_phase(&self) -> PyResult<Option<TransactionCreditPhase>> {
        match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(descr) => {
                Ok(descr.credit_ph.clone().map(TransactionCreditPhase))
            }
            ton_block::TransactionDescr::TickTock(_) => Ok(None),
            _ => Err(PyRuntimeError::new_err("Unsupported transaction type")),
        }
    }

    #[getter]
    fn compute_phase(&self) -> PyResult<Option<TransactionComputePhase>> {
        let compute_phase = match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(descr) => &descr.compute_ph,
            ton_block::TransactionDescr::TickTock(descr) => &descr.compute_ph,
            _ => return Err(PyRuntimeError::new_err("Unsupported transaction type")),
        };

        Ok(match compute_phase {
            ton_block::TrComputePhase::Skipped(_) => None,
            ton_block::TrComputePhase::Vm(phase) => Some(TransactionComputePhase(phase.clone())),
        })
    }

    #[getter]
    fn action_phase(&self) -> PyResult<Option<TransactionActionPhase>> {
        let action = match &self.0.descr {
            ton_block::TransactionDescr::Ordinary(descr) => &descr.action,
            ton_block::TransactionDescr::TickTock(descr) => &descr.action,
            _ => return Err(PyRuntimeError::new_err("Unsupported transaction type")),
        };
        Ok(action.clone().map(TransactionActionPhase))
    }

    #[getter]
    fn bounce_phase(&self) -> Option<TransactionBouncePhase> {
        if let ton_block::TransactionDescr::Ordinary(descr) = &self.0.descr {
            if let ton_block::TrBouncePhase::Ok(phase) = descr.bounce.as_ref()? {
                return Some(TransactionBouncePhase(phase.clone()));
            }
        }
        None
    }

    pub fn get_in_msg(&self) -> PyResult<Message> {
        let Some(msg_cell) = self.0.data.in_msg_cell() else {
            return Err(PyRuntimeError::new_err(
                "Transaction without incoming message",
            ));
        };
        let hash = msg_cell.repr_hash();
        let data = ton_block::Message::construct_from_cell(msg_cell).handle_runtime_error()?;
        Ok(Message { data, hash })
    }

    pub fn get_out_msgs(&self) -> PyResult<Vec<Message>> {
        let mut result = Vec::with_capacity(self.0.data.outmsg_cnt as usize);
        self.0
            .data
            .out_msgs
            .iterate_slices(|msg_slice| {
                let msg_cell = msg_slice.reference(0)?;
                let hash = msg_cell.repr_hash();
                let data = ton_block::Message::construct_from_cell(msg_cell)?;
                result.push(Message { data, hash });
                Ok(true)
            })
            .handle_runtime_error()?;
        Ok(result)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.0.data.serialize().handle_runtime_error()?;
        encoding.encode_cell(&cell)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.0.data.serialize().handle_runtime_error()?;
        let bytes = ton_types::serialize_toc(&cell).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        self.0.data.serialize().handle_runtime_error().map(Cell)
    }

    fn __repr__(&self) -> String {
        format!(
            "<Transaction hash='{:x}', {:?}>",
            self.0.hash,
            self.get_type().unwrap_or(TransactionType::Ordinary)
        )
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes(self.0.hash.as_slice()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.hash.cmp(&other.0.hash))
    }
}

pub struct SharedTransaction {
    pub data: ton_block::Transaction,
    pub descr: ton_block::TransactionDescr,
    pub hash: ton_types::UInt256,
}

#[pyclass]
pub struct TransactionStoragePhase(ton_block::TrStoragePhase);

#[pymethods]
impl TransactionStoragePhase {
    #[getter]
    fn storage_fees_collected(&self) -> Tokens {
        self.0.storage_fees_collected.into()
    }

    #[getter]
    fn storage_fees_due(&self) -> Option<Tokens> {
        self.0.storage_fees_due.map(Tokens::from)
    }

    #[getter]
    fn status_change(&self) -> AccountStatusChange {
        self.0.status_change.clone().into()
    }
}

#[pyclass]
pub struct TransactionCreditPhase(ton_block::TrCreditPhase);

#[pymethods]
impl TransactionCreditPhase {
    #[getter]
    fn due_fees_collected(&self) -> Option<Tokens> {
        self.0.due_fees_collected.map(Tokens::from)
    }

    #[getter]
    fn credit(&self) -> Tokens {
        self.0.credit.grams.into()
    }
}

#[pyclass]
pub struct TransactionComputePhase(ton_block::TrComputePhaseVm);

#[pymethods]
impl TransactionComputePhase {
    #[getter]
    fn success(&self) -> bool {
        self.0.success
    }

    #[getter]
    fn msg_state_used(&self) -> bool {
        self.0.msg_state_used
    }

    #[getter]
    fn account_activated(&self) -> bool {
        self.0.account_activated
    }

    #[getter]
    fn gas_fees(&self) -> Tokens {
        self.0.gas_fees.into()
    }

    #[getter]
    fn gas_used(&self) -> u64 {
        self.0.gas_used.as_u64()
    }

    #[getter]
    fn gas_limit(&self) -> u64 {
        self.0.gas_limit.as_u64()
    }

    #[getter]
    fn gas_credit(&self) -> Option<u32> {
        self.0.gas_credit.map(|credit| credit.as_u32())
    }

    #[getter]
    fn mode(&self) -> i8 {
        self.0.mode
    }

    #[getter]
    fn exit_code(&self) -> i32 {
        self.0.exit_code
    }

    #[getter]
    fn exit_arg(&self) -> Option<i32> {
        self.0.exit_arg
    }

    #[getter]
    fn vm_steps(&self) -> u32 {
        self.0.vm_steps
    }

    #[getter]
    fn vm_init_state_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.vm_init_state_hash.as_slice())
    }

    #[getter]
    fn vm_final_state_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.vm_final_state_hash.as_slice())
    }
}

#[pyclass]
pub struct TransactionActionPhase(ton_block::TrActionPhase);

#[pymethods]
impl TransactionActionPhase {
    #[getter]
    fn success(&self) -> bool {
        self.0.success
    }

    #[getter]
    fn valid(&self) -> bool {
        self.0.valid
    }

    #[getter]
    fn no_funds(&self) -> bool {
        self.0.no_funds
    }

    #[getter]
    fn status_change(&self) -> AccountStatusChange {
        self.0.status_change.clone().into()
    }

    #[getter]
    fn total_fwd_fees(&self) -> Option<Tokens> {
        self.0.total_fwd_fees.map(Tokens::from)
    }

    #[getter]
    fn total_action_fees(&self) -> Option<Tokens> {
        self.0.total_action_fees.map(Tokens::from)
    }

    #[getter]
    fn result_code(&self) -> i32 {
        self.0.result_code
    }

    #[getter]
    fn result_arg(&self) -> Option<i32> {
        self.0.result_arg
    }

    #[getter]
    fn total_actions(&self) -> i16 {
        self.0.tot_actions
    }

    #[getter]
    fn special_actions(&self) -> i16 {
        self.0.spec_actions
    }

    #[getter]
    fn skipped_actions(&self) -> i16 {
        self.0.skipped_actions
    }

    #[getter]
    fn messages_created(&self) -> i16 {
        self.0.msgs_created
    }

    #[getter]
    fn action_list_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.action_list_hash.as_slice())
    }
}

#[pyclass]
pub struct TransactionBouncePhase(ton_block::TrBouncePhaseOk);

#[pymethods]
impl TransactionBouncePhase {
    #[getter]
    fn msg_fees(&self) -> Tokens {
        self.0.msg_fees.into()
    }

    #[getter]
    fn fwd_fees(&self) -> Tokens {
        self.0.fwd_fees.into()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[pyclass]
pub enum TransactionType {
    Ordinary = 0,
    Tick = 2,
    Tock = 3,
}

#[pymethods]
impl TransactionType {
    #[getter]
    fn is_ordinary(&self) -> bool {
        matches!(self, Self::Ordinary)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __repr__(&self) -> String {
        format!("TransactionType.{:?}", self)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(self)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[pyclass]
pub enum AccountStatus {
    NotExists,
    Uninit,
    Active,
    Frozen,
}

impl From<ton_block::AccountStatus> for AccountStatus {
    fn from(value: ton_block::AccountStatus) -> Self {
        match value {
            ton_block::AccountStatus::AccStateNonexist => Self::NotExists,
            ton_block::AccountStatus::AccStateUninit => Self::Uninit,
            ton_block::AccountStatus::AccStateActive => Self::Active,
            ton_block::AccountStatus::AccStateFrozen => Self::Frozen,
        }
    }
}

#[pymethods]
impl AccountStatus {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __repr__(&self) -> String {
        format!("AccountStatus.{:?}", self)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(self)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[pyclass]
pub enum AccountStatusChange {
    Unchanged,
    Frozen,
    Deleted,
}

impl From<ton_block::AccStatusChange> for AccountStatusChange {
    fn from(value: ton_block::AccStatusChange) -> Self {
        match value {
            ton_block::AccStatusChange::Unchanged => Self::Unchanged,
            ton_block::AccStatusChange::Frozen => Self::Frozen,
            ton_block::AccStatusChange::Deleted => Self::Deleted,
        }
    }
}

#[pymethods]
impl AccountStatusChange {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __repr__(&self) -> String {
        format!("AccountStatusChange.{:?}", self)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(self)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct Message {
    pub data: ton_block::Message,
    pub hash: ton_types::UInt256,
}

impl TryFrom<ton_types::Cell> for Message {
    type Error = PyErr;

    fn try_from(msg_cell: ton_types::Cell) -> Result<Self, Self::Error> {
        let hash = msg_cell.repr_hash();
        let data = ton_block::Message::construct_from_cell(msg_cell).handle_value_error()?;
        Ok(Self { data, hash })
    }
}

#[pymethods]
impl Message {
    #[staticmethod]
    fn from_bytes(mut bytes: &[u8]) -> PyResult<Self> {
        let cell = ton_types::deserialize_tree_of_cells(&mut bytes).handle_runtime_error()?;
        Self::try_from(cell)
    }

    #[staticmethod]
    fn from_cell(cell: &Cell) -> PyResult<Self> {
        Self::try_from(cell.0.clone())
    }

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(&bytes)
    }

    #[new]
    pub fn new(
        header: PyRef<'_, MessageHeader>,
        body: Option<Cell>,
        state_init: Option<StateInit>,
        py: Python<'_>,
    ) -> PyResult<Self> {
        let message_type = header.get_type();
        let header = header.into_py(py);

        let mut message = match message_type {
            MessageType::ExternalIn => ton_block::Message::with_ext_in_header(
                header
                    .extract::<PyRef<ExternalInMessageHeader>>(py)?
                    .0
                    .clone(),
            ),
            MessageType::ExternalOut => ton_block::Message::with_ext_out_header(
                header
                    .extract::<PyRef<ExternalOutMessageHeader>>(py)?
                    .0
                    .clone(),
            ),
            MessageType::Internal => ton_block::Message::with_int_header(
                header
                    .extract::<PyRef<InternalMessageHeader>>(py)?
                    .0
                    .clone(),
            ),
        };

        if let Some(body) = body {
            message.set_body(ton_types::SliceData::load_cell(body.0).handle_value_error()?);
        }
        if let Some(state_init) = state_init {
            message.set_state_init(state_init.0);
        }

        let hash = message.hash().handle_value_error()?;

        Ok(Self {
            data: message,
            hash,
        })
    }

    #[getter]
    fn hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.hash.as_slice())
    }

    #[getter]
    fn is_external_in(&self) -> bool {
        self.data.is_inbound_external()
    }

    #[getter]
    fn is_external_out(&self) -> bool {
        self.data.is_outbound_external()
    }

    #[getter]
    fn is_internal(&self) -> bool {
        self.data.is_internal()
    }

    #[getter]
    fn get_type(&self) -> MessageType {
        match self.data.header() {
            ton_block::CommonMsgInfo::IntMsgInfo(_) => MessageType::Internal,
            ton_block::CommonMsgInfo::ExtInMsgInfo(_) => MessageType::ExternalIn,
            ton_block::CommonMsgInfo::ExtOutMsgInfo(_) => MessageType::ExternalOut,
        }
    }

    #[getter]
    fn header(&self, py: Python<'_>) -> PyObject {
        match self.data.header().clone() {
            ton_block::CommonMsgInfo::IntMsgInfo(header) => {
                let ty = PyClassInitializer::from(MessageHeader(MessageType::Internal))
                    .add_subclass(InternalMessageHeader(header));
                Py::new(py, ty).unwrap().into_py(py)
            }
            ton_block::CommonMsgInfo::ExtInMsgInfo(header) => {
                let ty = PyClassInitializer::from(MessageHeader(MessageType::ExternalIn))
                    .add_subclass(ExternalInMessageHeader(header));
                Py::new(py, ty).unwrap().into_py(py)
            }
            ton_block::CommonMsgInfo::ExtOutMsgInfo(header) => {
                let ty = PyClassInitializer::from(MessageHeader(MessageType::ExternalOut))
                    .add_subclass(ExternalOutMessageHeader(header));
                Py::new(py, ty).unwrap().into_py(py)
            }
        }
    }

    #[getter]
    fn created_at(&self) -> Option<u32> {
        match self.data.header() {
            ton_block::CommonMsgInfo::IntMsgInfo(x) => Some(x.created_at.as_u32()),
            ton_block::CommonMsgInfo::ExtOutMsgInfo(x) => Some(x.created_at.as_u32()),
            ton_block::CommonMsgInfo::ExtInMsgInfo(_) => None,
        }
    }
    #[getter]
    fn created_lt(&self) -> Option<u64> {
        match self.data.header() {
            ton_block::CommonMsgInfo::IntMsgInfo(x) => Some(x.created_lt),
            ton_block::CommonMsgInfo::ExtOutMsgInfo(x) => Some(x.created_lt),
            ton_block::CommonMsgInfo::ExtInMsgInfo(_) => None,
        }
    }

    #[getter]
    fn src(&self) -> Option<Address> {
        self.data.src().map(Address)
    }

    #[getter]
    fn dst(&self) -> Option<Address> {
        self.data.dst().map(Address)
    }

    #[getter]
    fn value(&self) -> Tokens {
        self.data
            .value()
            .map(|c| c.grams.into())
            .unwrap_or_default()
    }

    #[getter]
    fn bounced(&self) -> bool {
        self.data.bounced()
    }

    #[getter]
    fn body(&self) -> Option<Cell> {
        self.data
            .body()
            .map(ton_types::SliceData::into_cell)
            .map(Cell)
    }

    #[getter]
    fn state_init(&self) -> Option<StateInit> {
        self.data.state_init().cloned().map(StateInit)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.data.serialize().handle_runtime_error()?;
        encoding.encode_cell(&cell)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.data.serialize().handle_runtime_error()?;
        let bytes = ton_types::serialize_toc(&cell).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        self.data.serialize().handle_runtime_error().map(Cell)
    }

    fn __repr__(&self) -> String {
        format!("<Message hash='{:x}', {:?}>", self.hash, self.get_type())
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes(self.hash.as_slice()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.hash.cmp(&other.hash))
    }
}

#[pyclass(subclass)]
pub struct MessageHeader(MessageType);

#[pymethods]
impl MessageHeader {
    #[getter]
    fn get_type(&self) -> MessageType {
        self.0
    }
}

#[pyclass(extends = MessageHeader)]
pub struct InternalMessageHeader(pub ton_block::InternalMessageHeader);

#[pymethods]
impl InternalMessageHeader {
    #[allow(clippy::too_many_arguments)]
    #[new]
    pub fn new(
        value: Tokens,
        dst: Address,
        src: Option<Address>,
        ihr_disabled: Option<bool>,
        bounce: Option<bool>,
        bounced: Option<bool>,
        ihr_fee: Option<Tokens>,
        fwd_fee: Option<Tokens>,
        created_lt: Option<u64>,
        created_at: Option<u32>,
    ) -> PyResult<PyClassInitializer<Self>> {
        Ok(
            PyClassInitializer::from(MessageHeader(MessageType::Internal)).add_subclass(Self(
                ton_block::InternalMessageHeader {
                    ihr_disabled: ihr_disabled.unwrap_or(true),
                    bounce: bounce.unwrap_or_default(),
                    bounced: bounced.unwrap_or_default(),
                    src: src
                        .map(|Address(addr)| ton_block::MsgAddressIntOrNone::Some(addr))
                        .unwrap_or_default(),
                    dst: dst.0,
                    value: value.try_into()?,
                    ihr_fee: ihr_fee.unwrap_or_default().try_into()?,
                    fwd_fee: fwd_fee.unwrap_or_default().try_into()?,
                    created_lt: created_lt.unwrap_or_default(),
                    created_at: created_at.unwrap_or_default().into(),
                },
            )),
        )
    }

    #[getter]
    pub fn ihr_disabled(&self) -> bool {
        self.0.ihr_disabled
    }

    #[getter]
    pub fn bounce(&self) -> bool {
        self.0.bounce
    }

    #[getter]
    pub fn bounced(&self) -> bool {
        self.0.bounced
    }

    #[getter]
    pub fn src(&self) -> PyResult<Address> {
        match &self.0.src {
            ton_block::MsgAddressIntOrNone::Some(addr) => Ok(Address(addr.clone())),
            ton_block::MsgAddressIntOrNone::None => {
                Err(PyValueError::new_err("Message without source address"))
            }
        }
    }

    #[getter]
    pub fn dst(&self) -> Address {
        Address(self.0.dst.clone())
    }

    #[getter]
    pub fn value(&self) -> Tokens {
        self.0.value.grams.into()
    }

    #[getter]
    pub fn ihr_fee(&self) -> Tokens {
        self.0.ihr_fee.into()
    }

    #[getter]
    pub fn fwd_fee(&self) -> Tokens {
        self.0.fwd_fee.into()
    }

    #[getter]
    pub fn created_at(&self) -> u32 {
        self.0.created_at.as_u32()
    }

    #[getter]
    pub fn created_lt(&self) -> u64 {
        self.0.created_lt
    }
}

#[pyclass(extends = MessageHeader)]
pub struct ExternalInMessageHeader(ton_block::ExternalInboundMessageHeader);

#[pymethods]
impl ExternalInMessageHeader {
    #[new]
    pub fn new(dst: Address, import_fee: Option<Tokens>) -> PyResult<PyClassInitializer<Self>> {
        Ok(
            PyClassInitializer::from(MessageHeader(MessageType::ExternalIn)).add_subclass(Self(
                ton_block::ExternalInboundMessageHeader {
                    dst: dst.0,
                    import_fee: import_fee.unwrap_or_default().try_into()?,
                    ..Default::default()
                },
            )),
        )
    }

    #[getter]
    pub fn dst(&self) -> Address {
        Address(self.0.dst.clone())
    }

    #[getter]
    pub fn import_fee(&self) -> Tokens {
        self.0.import_fee.into()
    }
}

#[pyclass(extends = MessageHeader)]
pub struct ExternalOutMessageHeader(ton_block::ExtOutMessageHeader);

#[pymethods]
impl ExternalOutMessageHeader {
    #[new]
    pub fn new(
        src: Option<Address>,
        created_lt: Option<u64>,
        created_at: Option<u32>,
    ) -> PyResult<PyClassInitializer<Self>> {
        Ok(
            PyClassInitializer::from(MessageHeader(MessageType::ExternalOut)).add_subclass(Self(
                ton_block::ExtOutMessageHeader {
                    src: src
                        .map(|Address(addr)| ton_block::MsgAddressIntOrNone::Some(addr))
                        .unwrap_or_default(),
                    dst: Default::default(),
                    created_lt: created_lt.unwrap_or_default(),
                    created_at: created_at.unwrap_or_default().into(),
                },
            )),
        )
    }

    #[getter]
    pub fn src(&self) -> PyResult<Address> {
        match &self.0.src {
            ton_block::MsgAddressIntOrNone::Some(addr) => Ok(Address(addr.clone())),
            ton_block::MsgAddressIntOrNone::None => {
                Err(PyValueError::new_err("Message without source address"))
            }
        }
    }

    #[getter]
    pub fn created_at(&self) -> u32 {
        self.0.created_at.as_u32()
    }

    #[getter]
    pub fn created_lt(&self) -> u64 {
        self.0.created_lt
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[pyclass]
pub enum MessageType {
    Internal = 0,
    ExternalIn = 1,
    ExternalOut = 2,
}

#[pymethods]
impl MessageType {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __repr__(&self) -> String {
        format!("MessageType.{:?}", self)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(self)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}

#[derive(Clone)]
#[pyclass]
pub struct StateInit(pub ton_block::StateInit);

impl StateInit {
    fn expect_code(&self) -> PyResult<ton_types::Cell> {
        match &self.0.code {
            Some(code) => Ok(code.clone()),
            None => Err(PyRuntimeError::new_err("StateInit has no code")),
        }
    }
}

#[pymethods]
impl StateInit {
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        ton_block::StateInit::construct_from_bytes(bytes)
            .handle_value_error()
            .map(Self)
    }

    #[staticmethod]
    fn from_cell(cell: &Cell) -> PyResult<Self> {
        ton_block::StateInit::construct_from_cell(cell.0.clone())
            .handle_value_error()
            .map(Self)
    }

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(&bytes)
    }

    #[new]
    fn new(code: Option<Cell>, data: Option<Cell>) -> Self {
        Self(ton_block::StateInit {
            code: code.map(|Cell(code)| code),
            data: data.map(|Cell(cell)| cell),
            ..Default::default()
        })
    }

    #[getter]
    fn code_hash<'a>(&self, py: Python<'a>) -> Option<&'a PyBytes> {
        let code = self.0.code.as_ref()?;
        Some(PyBytes::new(py, code.repr_hash().as_slice()))
    }

    #[getter]
    fn get_code(&self) -> Option<Cell> {
        self.0.code.clone().map(Cell)
    }

    #[setter]
    fn set_code(&mut self, code: Option<Cell>) {
        self.0.code = code.map(|Cell(code)| code);
    }

    #[getter]
    fn get_data(&self) -> Option<Cell> {
        self.0.data.clone().map(Cell)
    }

    #[setter]
    fn set_data(&mut self, data: Option<Cell>) {
        self.0.data = data.map(|Cell(data)| data);
    }

    /// Adds the specified salt to the code of this state init.
    fn set_code_salt(&mut self, salt: &Cell) -> PyResult<()> {
        self.0.code = nt::abi::set_code_salt(self.expect_code()?, salt.0.clone())
            .handle_runtime_error()
            .map(Some)?;
        Ok(())
    }

    /// Tries to extract a salt from the code of this state init.
    fn get_code_salt(&self) -> PyResult<Option<Cell>> {
        let salt = nt::abi::get_code_salt(self.expect_code()?).handle_runtime_error()?;
        Ok(salt.map(Cell))
    }

    fn compute_address(&self, workchain: Option<i8>) -> PyResult<Address> {
        let cell = self.build_cell()?;
        ton_block::MsgAddressInt::with_standart(
            None,
            workchain.unwrap_or_default(),
            cell.0.repr_hash().into(),
        )
        .handle_runtime_error()
        .map(Address)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.0.serialize().handle_runtime_error()?;
        encoding.encode_cell(&cell)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.0.serialize().handle_runtime_error()?;
        let bytes = ton_types::serialize_toc(&cell).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        self.0.serialize().handle_runtime_error().map(Cell)
    }

    fn __repr__(&self) -> String {
        use std::borrow::Cow;

        fn field_repr(cell: &Option<ton_types::Cell>) -> Cow<'static, str> {
            match cell {
                Some(cell) => Cow::Owned(format!("'{:x}'", cell.repr_hash())),
                None => Cow::Borrowed("None"),
            }
        }

        format!(
            "<StateInit code_hash='{}', data_hash='{}'>",
            field_repr(&self.0.code),
            field_repr(&self.0.data),
        )
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Address(pub ton_block::MsgAddressInt);

#[pymethods]
impl Address {
    #[staticmethod]
    fn validate(addr: &str) -> bool {
        nt::utils::validate_address(addr)
    }

    #[staticmethod]
    fn from_parts(workchain: i8, account: &[u8]) -> PyResult<Self> {
        if account.len() != 32 {
            return Err(PyValueError::new_err("Account len must be 32 bytes"));
        }

        let account = ton_types::UInt256::from_le_bytes(account);
        ton_block::MsgAddressInt::with_standart(None, workchain, account.into())
            .handle_value_error()
            .map(Self)
    }

    #[new]
    pub fn new(addr: &str) -> PyResult<Self> {
        nt::utils::repack_address(addr.trim())
            .map(Self)
            .handle_value_error()
    }

    #[getter]
    fn get_workchain(&self) -> i32 {
        self.0.workchain_id()
    }

    #[setter]
    fn set_workchain(&mut self, workchain: i32) -> PyResult<()> {
        match &mut self.0 {
            ton_block::MsgAddressInt::AddrStd(addr) => {
                addr.workchain_id = workchain.try_into().handle_value_error()?
            }
            ton_block::MsgAddressInt::AddrVar(addr) => addr.workchain_id = workchain,
        }
        Ok(())
    }

    #[getter]
    fn account<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        let bytes = self.0.address().get_bytestring_on_stack(0);
        PyBytes::new(py, &bytes)
    }

    #[pyo3(signature = (url_safe = true, bounce = false))]
    fn to_base64(&self, url_safe: bool, bounce: bool) -> PyResult<String> {
        nt::utils::pack_std_smc_addr(url_safe, &self.0, bounce).handle_value_error()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Address('{}')", self.0)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(&self.0)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.cmp(&other.0))
    }
}

#[derive(Default, Clone)]
#[pyclass]
pub struct Cell(pub ton_types::Cell);

impl Cell {
    pub fn try_from_struct(value: &dyn Serializable) -> PyResult<Self> {
        value.serialize().handle_runtime_error().map(Self)
    }
}

#[pymethods]
impl Cell {
    #[staticmethod]
    fn from_bytes(mut bytes: &[u8]) -> PyResult<Self> {
        ton_types::deserialize_tree_of_cells(&mut bytes)
            .handle_value_error()
            .map(Self)
    }

    #[staticmethod]
    fn build(
        abi: Vec<(String, AbiParam)>,
        value: &PyDict,
        abi_version: Option<AbiVersion>,
    ) -> PyResult<Self> {
        let params = abi
            .into_iter()
            .map(|(name, AbiParam { param })| ton_abi::Param { name, kind: param })
            .collect::<Vec<_>>();

        let tokens = parse_tokens(&params, value)?;

        let abi_version = match abi_version {
            Some(version) => version.0,
            None => ton_abi::contract::ABI_VERSION_2_2,
        };

        nt::abi::pack_into_cell(&tokens, abi_version)
            .map(Self)
            .handle_runtime_error()
    }

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding.decode_cell(value.trim()).map(Self)
    }

    /// Constructs a new empty cell.
    #[new]
    fn new() -> Self {
        Self(Default::default())
    }

    /// Returns a hex encoded repr hash of the root cell.
    #[getter]
    fn repr_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.repr_hash().as_slice())
    }

    #[getter]
    fn bits(&self) -> usize {
        self.0.bit_length()
    }

    #[getter]
    fn refs(&self) -> usize {
        self.0.references_count()
    }

    fn as_slice(&self) -> PyResult<CellSlice> {
        Ok(CellSlice {
            slice: ton_types::SliceData::load_cell(self.0.clone()).handle_value_error()?,
            cell: self.clone(),
        })
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding.encode_cell(&self.0)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let bytes = ton_types::serialize_toc(&self.0).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn unpack<'a>(
        &self,
        py: Python<'a>,
        abi: Vec<(String, AbiParam)>,
        abi_version: Option<AbiVersion>,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let params = abi
            .into_iter()
            .map(|(name, AbiParam { param })| ton_abi::Param { name, kind: param })
            .collect::<Vec<_>>();

        let abi_version = match abi_version {
            Some(version) => version.0,
            None => ton_abi::contract::ABI_VERSION_2_2,
        };

        let allow_partial = allow_partial.unwrap_or_default();
        let data = ton_types::SliceData::load_cell_ref(&self.0).handle_runtime_error()?;

        let tokens = nt::abi::unpack_from_cell(&params, data, allow_partial, abi_version)
            .handle_runtime_error()?;

        convert_tokens(py, tokens)
    }

    /// Tries to interpret this cell as an unsalted code and
    /// returns a new cell with the salt added to it.
    fn with_code_salt(&self, salt: &Cell) -> PyResult<Cell> {
        nt::abi::set_code_salt(self.0.clone(), salt.0.clone())
            .handle_runtime_error()
            .map(Cell)
    }

    /// Tries to interpret this cell as a salted code and tries to extract the salt from it.
    fn get_code_salt(&self) -> PyResult<Option<Cell>> {
        let salt = nt::abi::get_code_salt(self.0.clone()).handle_runtime_error()?;
        Ok(salt.map(Cell))
    }

    fn __repr__(&self) -> String {
        format!(
            "<Cell repr_hash='{:x}', bits={}, refs={}>",
            self.0.repr_hash(),
            self.0.bit_length(),
            self.0.references_count()
        )
    }

    fn __hash__(&self) -> u64 {
        let hash = self.0.repr_hash();
        u64::from_le_bytes(hash.as_slice()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.repr_hash().cmp(&other.0.repr_hash()))
    }
}

impl AsRef<ton_types::Cell> for Cell {
    #[inline]
    fn as_ref(&self) -> &ton_types::Cell {
        &self.0
    }
}

impl From<ton_types::Cell> for Cell {
    #[inline]
    fn from(value: ton_types::Cell) -> Self {
        Self(value)
    }
}

impl From<Cell> for ton_types::Cell {
    #[inline]
    fn from(value: Cell) -> Self {
        value.0
    }
}

#[derive(Default, Clone)]
#[pyclass]
pub struct CellSlice {
    pub slice: ton_types::SliceData,
    pub cell: Cell,
}

#[pymethods]
impl CellSlice {
    fn advance(&mut self, bits: Option<usize>, refs: Option<usize>) -> PyResult<()> {
        if let Some(bits) = bits {
            self.slice.move_by(bits).handle_value_error()?;
        }

        if let Some(refs) = refs {
            for _ in 0..refs {
                self.slice.checked_drain_reference().handle_value_error()?;
            }
        }

        Ok(())
    }

    pub fn shrink(&mut self, bits: Option<usize>, refs: Option<usize>) -> PyResult<()> {
        if let Some(bits) = bits {
            if bits > self.slice.remaining_bits() {
                return Err(ton_types::ExceptionCode::CellUnderflow).handle_value_error()?;
            }

            self.slice.shrink_data(..bits);
        }

        if let Some(refs) = refs {
            if refs > self.slice.remaining_references() {
                return Err(ton_types::ExceptionCode::CellUnderflow).handle_value_error()?;
            }

            self.slice.shrink_references(..refs);
        }

        Ok(())
    }

    #[getter]
    fn cell(&self) -> Cell {
        self.cell.clone()
    }

    #[getter]
    fn bits(&self) -> usize {
        self.slice.remaining_bits()
    }

    #[getter]
    fn refs(&self) -> usize {
        self.slice.remaining_references()
    }

    #[getter]
    fn bits_offset(&self) -> usize {
        self.slice.pos()
    }

    #[getter]
    fn refs_offset(&self) -> usize {
        let total_refs = self.slice.cell().references_count();
        self.slice.remaining_references() - total_refs
    }

    fn is_empty(&self) -> bool {
        self.is_data_empty() && self.is_refs_empty()
    }

    fn is_data_empty(&self) -> bool {
        self.slice.is_empty()
    }

    fn is_refs_empty(&self) -> bool {
        self.slice.remaining_references() == 0
    }

    fn has_remaining(&self, bits: usize, refs: usize) -> bool {
        self.slice.remaining_bits() >= bits && self.slice.remaining_references() >= refs
    }

    fn get_bit(&self, offset: usize) -> PyResult<bool> {
        self.slice.get_bit(offset).handle_value_error()
    }

    fn get_u8(&self, offset: usize) -> PyResult<u8> {
        self.slice.get_byte(offset).handle_value_error()
    }

    fn get_i8(&self, offset: usize) -> PyResult<i8> {
        self.get_u8(offset).map(|value| value as i8)
    }

    fn get_u16(&self, offset: usize) -> PyResult<u16> {
        let mut value: u16 = 0;
        for i in 0..2 {
            value |=
                (self.slice.get_byte(offset + 8 * i).handle_value_error()? as u16) << (8 * (1 - i));
        }
        Ok(value)
    }

    fn get_i16(&self, offset: usize) -> PyResult<i16> {
        self.get_u16(offset).map(|value| value as i16)
    }

    fn get_u32(&self, offset: usize) -> PyResult<u32> {
        let mut value: u32 = 0;
        for i in 0..4 {
            value |=
                (self.slice.get_byte(offset + 8 * i).handle_value_error()? as u32) << (8 * (3 - i));
        }
        Ok(value)
    }

    fn get_i32(&self, offset: usize) -> PyResult<i32> {
        self.get_u32(offset).map(|value| value as i32)
    }

    fn get_u64(&self, offset: usize) -> PyResult<u64> {
        let mut value: u64 = 0;
        for i in 0..8 {
            value |=
                (self.slice.get_byte(offset + 8 * i).handle_value_error()? as u64) << (8 * (7 - i));
        }
        Ok(value)
    }

    fn get_i64(&self, offset: usize) -> PyResult<i64> {
        self.get_u64(offset).map(|value| value as i64)
    }

    fn get_u128(&self, offset: usize) -> PyResult<u128> {
        let mut value: u128 = 0;
        for i in 0..16 {
            value |= (self.slice.get_byte(offset + 8 * i).handle_value_error()? as u128)
                << (8 * (15 - i));
        }
        Ok(value)
    }

    fn get_i128(&self, offset: usize) -> PyResult<i128> {
        self.get_u128(offset).map(|value| value as i128)
    }

    fn get_u256(&self, offset: usize) -> PyResult<num_bigint::BigUint> {
        let mut value = num_bigint::BigUint::default();
        for i in 0..32 {
            value |= num_bigint::BigUint::from(
                self.slice.get_byte(offset + 8 * i).handle_value_error()?,
            ) << (8 * (31 - i));
        }
        Ok(value)
    }

    fn get_public_key(&self, offset: usize) -> PyResult<PublicKey> {
        let mut bytes = [0; 32];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.slice.get_byte(offset + 8 * i).handle_value_error()?;
        }
        PublicKey::from_bytes(&bytes)
    }

    fn get_signature(&self, offset: usize) -> PyResult<Signature> {
        let mut bytes = [0; 64];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.slice.get_byte(offset + 8 * i).handle_value_error()?;
        }
        Signature::from_bytes(&bytes)
    }

    fn get_bytes<'a>(&self, offset: usize, size: usize, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let mut bytes = Vec::with_capacity(size);
        for i in 0..size {
            bytes.push(self.slice.get_byte(offset + 8 * i).handle_value_error()?);
        }
        Ok(PyBytes::new(py, &bytes))
    }

    fn get_reference(&self, offset: usize) -> PyResult<Cell> {
        self.slice
            .reference(offset)
            .handle_runtime_error()
            .map(Cell)
    }

    fn load_bit(&mut self) -> PyResult<bool> {
        self.slice.get_next_bit().handle_runtime_error()
    }

    fn load_u8(&mut self) -> PyResult<u8> {
        self.slice.get_next_byte().handle_runtime_error()
    }

    fn load_i8(&mut self) -> PyResult<i8> {
        self.load_u8().map(|value| value as i8)
    }

    fn load_u16(&mut self) -> PyResult<u16> {
        self.slice.get_next_u16().handle_runtime_error()
    }

    fn load_i16(&mut self) -> PyResult<i16> {
        self.load_u16().map(|value| value as i16)
    }

    fn load_u32(&mut self) -> PyResult<u32> {
        self.slice.get_next_u32().handle_runtime_error()
    }

    fn load_i32(&mut self) -> PyResult<i32> {
        self.load_u32().map(|value| value as i32)
    }

    fn load_u64(&mut self) -> PyResult<u64> {
        self.slice.get_next_u64().handle_runtime_error()
    }

    fn load_i64(&mut self) -> PyResult<i64> {
        self.load_u64().map(|value| value as i64)
    }

    fn load_u128(&mut self) -> PyResult<u128> {
        self.slice.get_next_u128().handle_runtime_error()
    }

    fn load_i128(&mut self) -> PyResult<i128> {
        self.load_u128().map(|value| value as i128)
    }

    fn load_u256(&mut self) -> PyResult<num_bigint::BigUint> {
        let bytes = self.slice.get_next_bytes(32).handle_value_error()?;
        Ok(num_bigint::BigUint::from_bytes_be(&bytes))
    }

    fn load_public_key(&mut self) -> PyResult<PublicKey> {
        let bytes = self.slice.get_next_bytes(32).handle_value_error()?;
        PublicKey::from_bytes(&bytes)
    }

    fn load_signature(&mut self) -> PyResult<Signature> {
        let bytes = self.slice.get_next_bytes(64).handle_value_error()?;
        Signature::from_bytes(&bytes)
    }

    fn load_bytes<'a>(&mut self, size: usize, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let bytes = self.slice.get_next_bytes(size).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn load_reference(&mut self) -> PyResult<Cell> {
        self.slice
            .checked_drain_reference()
            .handle_runtime_error()
            .map(Cell)
    }

    fn __repr__(&self) -> String {
        format!(
            "<CellSlice cell={}, bits={}..{}, refs={}..{}>",
            self.cell.0.repr_hash().to_hex_string(),
            self.bits_offset(),
            self.bits_offset() + self.bits(),
            self.refs_offset(),
            self.refs_offset() + self.refs()
        )
    }
}

#[derive(Default, Clone)]
#[pyclass]
pub struct CellBuilder {
    pub builder: ton_types::BuilderData,
    pub is_exotic: bool,
}

#[pymethods]
impl CellBuilder {
    /// Constructs a new empty builder.
    #[new]
    fn new() -> Self {
        Self {
            builder: ton_types::BuilderData::new(),
            is_exotic: false,
        }
    }

    #[getter]
    fn bits(&self) -> usize {
        self.builder.bits_used()
    }

    #[getter]
    fn refs(&self) -> usize {
        self.builder.references_used()
    }

    #[getter]
    fn spare_bits(&self) -> usize {
        self.builder.bits_free()
    }

    #[getter]
    fn spare_refs(&self) -> usize {
        self.builder.references_free()
    }

    #[getter]
    fn get_is_exotic(&self) -> bool {
        self.is_exotic
    }

    #[setter]
    fn set_is_exotic(&mut self, is_exotic: bool) {
        self.is_exotic = is_exotic;
    }

    fn build(&self) -> PyResult<Cell> {
        let mut builder = self.builder.clone();
        if self.is_exotic {
            if builder.length_in_bits() < 8 {
                return Err("Not enough data for an exotic cell").handle_value_error();
            }

            let mut children_mask = ton_types::LevelMask::default();
            for child in builder.references() {
                children_mask |= child.level_mask();
            }

            let cell_type =
                ton_types::CellType::try_from(builder.data()[0]).handle_value_error()?;

            let level_mask = match cell_type {
                ton_types::CellType::PrunedBranch => {
                    if builder.length_in_bits() < 16 {
                        return Err(ton_types::ExceptionCode::CellUnderflow).handle_value_error();
                    }

                    let raw_mask = builder.data()[1];
                    if raw_mask > 0b111 {
                        return Err("Invalid pruned branch mask").handle_value_error();
                    }
                    ton_types::LevelMask::with_mask(raw_mask)
                }
                ton_types::CellType::LibraryReference => ton_types::LevelMask::default(),
                ton_types::CellType::MerkleProof | ton_types::CellType::MerkleUpdate => {
                    children_mask.virtualize(1)
                }
                _ => {
                    return Err(format!("Incorrect type of exotic cell: {cell_type}"))
                        .handle_value_error();
                }
            };

            builder.set_type(cell_type);
            builder.set_level_mask(level_mask);
        }

        builder.into_cell().handle_value_error().map(Cell)
    }

    fn store_zeros(&mut self, bits: usize) -> PyResult<()> {
        static ZEROS: &[u8; 128] = &[0; 128];
        self.store_raw(ZEROS, bits)
    }

    fn store_ones(&mut self, bits: usize) -> PyResult<()> {
        static ONES: &[u8; 128] = &[0xff; 128];
        self.store_raw(ONES, bits)
    }

    fn store_bit_zero(&mut self) -> PyResult<()> {
        self.builder.append_bit_zero().handle_value_error()?;
        Ok(())
    }

    fn store_bit_one(&mut self) -> PyResult<()> {
        self.builder.append_bit_one().handle_value_error()?;
        Ok(())
    }

    fn store_bit(&mut self, bit: bool) -> PyResult<()> {
        self.builder.append_bit_bool(bit).handle_value_error()?;
        Ok(())
    }

    fn store_u8(&mut self, value: u8) -> PyResult<()> {
        self.builder.append_u8(value).handle_value_error()?;
        Ok(())
    }

    fn store_i8(&mut self, value: i8) -> PyResult<()> {
        self.builder.append_i8(value).handle_value_error()?;
        Ok(())
    }

    fn store_u16(&mut self, value: u16) -> PyResult<()> {
        self.builder.append_u16(value).handle_value_error()?;
        Ok(())
    }

    fn store_i16(&mut self, value: i16) -> PyResult<()> {
        self.builder.append_i16(value).handle_value_error()?;
        Ok(())
    }

    fn store_u32(&mut self, value: u32) -> PyResult<()> {
        self.builder.append_u32(value).handle_value_error()?;
        Ok(())
    }

    fn store_i32(&mut self, value: i32) -> PyResult<()> {
        self.builder.append_i32(value).handle_value_error()?;
        Ok(())
    }

    fn store_u64(&mut self, value: u64) -> PyResult<()> {
        self.builder.append_u64(value).handle_value_error()?;
        Ok(())
    }

    fn store_i64(&mut self, value: i64) -> PyResult<()> {
        self.builder.append_i64(value).handle_value_error()?;
        Ok(())
    }

    fn store_u128(&mut self, value: u128) -> PyResult<()> {
        self.builder.append_u128(value).handle_value_error()?;
        Ok(())
    }

    fn store_i128(&mut self, value: i128) -> PyResult<()> {
        self.builder.append_i128(value).handle_value_error()?;
        Ok(())
    }

    fn store_uint(&mut self, value: num_bigint::BigUint, bits: usize) -> PyResult<()> {
        self.store_int(
            num_bigint::BigInt::from_biguint(num_bigint::Sign::Plus, value),
            bits,
        )
    }

    fn store_int(&mut self, value: num_bigint::BigInt, bits: usize) -> PyResult<()> {
        if bits > self.builder.bits_free() {
            return Err(ton_types::ExceptionCode::CellOverflow).handle_value_error();
        }

        let vec = value.to_signed_bytes_be();
        let vec_bits_length = vec.len() * 8;

        if bits > vec_bits_length {
            let padding = if value.sign() == num_bigint::Sign::Minus {
                0xffu8
            } else {
                0u8
            };

            let diff = bits - vec_bits_length;

            let mut vec_padding = Vec::new();
            vec_padding.resize(diff / 8 + 1, padding);

            self.builder
                .append_raw(&vec_padding, diff)
                .handle_value_error()?;
            self.builder
                .append_raw(&vec, bits - diff)
                .handle_value_error()?;
        } else {
            let number_bits = value.bits();
            if number_bits > bits as u64 {
                return Err(format!("Too many bits in value to fit into: {number_bits}"))
                    .handle_value_error();
            }

            let offset = vec_bits_length - bits;
            let first_byte = vec[offset / 8] << (offset % 8);

            self.builder
                .append_raw(&[first_byte], 8 - offset % 8)
                .handle_value_error()?;
            self.builder
                .append_raw(&vec[offset / 8 + 1..], vec[offset / 8 + 1..].len() * 8)
                .handle_value_error()?;
        };

        Ok(())
    }

    fn store_public_key(&mut self, key: &PublicKey) -> PyResult<()> {
        self.store_bytes(key.0.as_bytes())
    }

    fn store_signature(&mut self, signature: &Signature) -> PyResult<()> {
        self.store_bytes(signature.0.as_ref())
    }

    fn store_bytes(&mut self, bytes: &[u8]) -> PyResult<()> {
        self.store_raw(bytes, bytes.len() * 8)
    }

    fn store_raw(&mut self, bytes: &[u8], bits: usize) -> PyResult<()> {
        if bits > self.builder.bits_free() {
            return Err(ton_types::ExceptionCode::CellOverflow).handle_value_error();
        }
        self.builder.append_raw(bytes, bits).handle_value_error()?;
        Ok(())
    }

    fn store_reference(&mut self, cell: Cell) -> PyResult<()> {
        self.builder
            .checked_append_reference(cell.0.clone())
            .handle_value_error()?;
        Ok(())
    }

    fn store_builder(&mut self, value: &CellBuilder) -> PyResult<()> {
        self.builder
            .append_builder(&value.builder)
            .handle_value_error()?;
        Ok(())
    }

    fn store_slice(&mut self, value: &CellSlice) -> PyResult<()> {
        self.builder
            .append_builder(&ton_types::BuilderData::from_slice(&value.slice))
            .handle_value_error()?;
        Ok(())
    }

    fn store_abi(
        &mut self,
        abi: Vec<(String, AbiParam)>,
        value: &PyDict,
        abi_version: Option<AbiVersion>,
    ) -> PyResult<()> {
        let params = abi
            .into_iter()
            .map(|(name, AbiParam { param })| ton_abi::Param { name, kind: param })
            .collect::<Vec<_>>();

        let tokens = parse_tokens(&params, value)?;

        let abi_version = match abi_version {
            Some(version) => version.0,
            None => ton_abi::contract::ABI_VERSION_2_2,
        };

        let cells = vec![ton_abi::token::SerializedValue {
            data: self.builder.clone(),
            max_bits: self.builder.bits_used(),
            max_refs: self.builder.references_used(),
        }];
        let builder = ton_abi::TokenValue::pack_values_into_chain(&tokens, cells, &abi_version)
            .handle_value_error()?;
        self.builder = builder;

        Ok(())
    }

    fn __repr__(&self) -> String {
        format!(
            "<CellBuilder bits={}, refs={}, is_exotic={}>",
            self.builder.bits_used(),
            self.builder.references_used(),
            self.is_exotic,
        )
    }
}

#[derive(Default, Copy, Clone)]
#[pyclass]
pub struct Tokens(pub i128);

impl From<ton_block::Grams> for Tokens {
    #[inline]
    fn from(value: ton_block::Grams) -> Self {
        Tokens(value.as_u128() as i128)
    }
}

impl TryFrom<Tokens> for u128 {
    type Error = PyErr;

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        value.0.try_into().handle_value_error()
    }
}

impl TryFrom<Tokens> for ton_block::Grams {
    type Error = PyErr;

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        let value = value.0.try_into().handle_value_error()?;
        ton_block::Grams::new(value).handle_value_error()
    }
}

impl TryFrom<Tokens> for ton_block::CurrencyCollection {
    type Error = PyErr;

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        Ok(ton_block::CurrencyCollection::from_grams(value.try_into()?))
    }
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let int = self.0 / 1000000000;
        let mut frac = self.0.abs() % 1000000000;

        int.fmt(f)?;

        let mut n = 9;
        if frac > 0 {
            while frac % 10 == 0 && frac > 0 {
                frac /= 10;
                n -= 1;
            }
            f.write_fmt(format_args!(".{frac:0n$}"))?;
        }
        Ok(())
    }
}

#[derive(FromPyObject)]
enum TokensValue<'a> {
    #[pyo3(transparent, annotation = "str")]
    String(&'a str),
    #[pyo3(transparent, annotation = "int")]
    Int(i64),
}

#[pymethods]
impl Tokens {
    #[staticmethod]
    fn from_nano(nano: i128) -> Self {
        Self(nano)
    }

    #[new]
    fn new(value: TokensValue<'_>) -> PyResult<Self> {
        const ONE: i128 = 1000000000;
        const OVERFLOW: &str = "Tokens overflow";

        let value = match value {
            TokensValue::String(value) => {
                let (value, negative) = match value.strip_prefix('-') {
                    Some(value) => (value, true),
                    None => (value, false),
                };

                let (int, frac) = match value.split_once('.') {
                    Some((int, frac)) => {
                        let int = int.parse::<u64>().handle_value_error()?;

                        let frac_scale = match 9usize.checked_sub(frac.len()) {
                            Some(scale) => 10u64.pow(scale as u32),
                            None => return Err(PyValueError::new_err("Invalid tokens precision")),
                        };
                        let frac = frac.parse::<u64>().handle_value_error()?;

                        (int, frac * frac_scale)
                    }
                    None => (value.parse::<u64>().handle_value_error()?, 0),
                };

                let Some(int) = (int as i128).checked_mul(ONE) else {
                    return Err(PyOverflowError::new_err(OVERFLOW));
                };

                match int.checked_add(frac as i128) {
                    Some(value) if negative => -value,
                    Some(value) => value,
                    None => return Err(PyOverflowError::new_err(OVERFLOW)),
                }
            }
            TokensValue::Int(value) => (value as i128) * ONE,
        };
        Ok(Self(value))
    }

    #[getter]
    fn is_signed(&self) -> bool {
        self.0 < 0
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn max(&self, other: &Self) -> Self {
        Tokens(std::cmp::max(self.0, other.0))
    }

    fn min(&self, other: &Self) -> Self {
        Tokens(std::cmp::min(self.0, other.0))
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_nano(&self) -> i128 {
        self.0
    }

    fn abs(&self) -> Tokens {
        Tokens(self.0.abs())
    }

    fn __bool__(&self) -> bool {
        self.0 != 0
    }

    fn __int__(&self) -> i128 {
        self.0
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Tokens.from_nano({})", self.0)
    }

    fn __hash__(&self) -> u64 {
        make_hasher().hash_one(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.cmp(&other.0))
    }

    fn __add__(&self, other: &Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    fn __sub__(&self, other: &Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    fn __mul__(&self, other: i64) -> Self {
        Self(self.0.saturating_mul(other as i128))
    }

    fn __rmul__(&self, other: i64) -> Self {
        Self(self.0.saturating_mul(other as i128))
    }

    fn __truediv__(&self, other: i64) -> PyResult<Self> {
        match self.0.checked_div(other as i128) {
            Some(i) => Ok(Self(i)),
            None => Err(PyZeroDivisionError::new_err("division by zero")),
        }
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }
}

#[pyclass]
pub struct TransactionTree {
    root: Py<Transaction>,
    children_raw: Vec<Py<TransactionTree>>,
    children: PyObject,
}

impl TransactionTree {
    fn unpack(py: Python<'_>, mut slice: ton_types::SliceData) -> PyResult<Py<Self>> {
        if !slice.get_next_bit().handle_value_error()? {
            return Err(PyValueError::new_err("Invalid transaction tree node"));
        }

        let mut root = {
            let cell = slice.checked_drain_reference().handle_value_error()?;
            Self::unpack_only_root(py, cell)?
        };

        Self::unpack_children(py, slice, &mut root)?;

        root.finalize(py);

        Py::new(py, root)
    }

    fn unpack_only_root(py: Python<'_>, cell: ton_types::Cell) -> PyResult<Self> {
        let tx = Transaction::try_from(cell)?;
        Ok(Self {
            root: Py::new(py, tx)?,
            children_raw: Vec::new(),
            children: py_none(),
        })
    }

    fn unpack_children(
        py: Python<'_>,
        mut slice: ton_types::SliceData,
        root: &mut TransactionTree,
    ) -> PyResult<()> {
        while let Ok(bit) = slice.get_next_bit() {
            let cell = slice.checked_drain_reference().handle_runtime_error()?;
            let slice = ton_types::SliceData::load_cell(cell).handle_runtime_error()?;

            if bit {
                root.children_raw.push(Self::unpack(py, slice)?);
            } else {
                Self::unpack_children(py, slice, root)?
            }
        }
        Ok(())
    }

    fn finalize(&mut self, py: Python<'_>) {
        self.children = PyList::new(py, &self.children_raw).into_py(py);
    }
}

#[pymethods]
impl TransactionTree {
    #[staticmethod]
    fn from_bytes(py: Python<'_>, mut bytes: &[u8]) -> PyResult<Py<Self>> {
        let cell = ton_types::deserialize_tree_of_cells(&mut bytes).handle_runtime_error()?;
        let slice = ton_types::SliceData::load_cell(cell).handle_runtime_error()?;
        TransactionTree::unpack(py, slice)
    }

    #[staticmethod]
    fn decode(py: Python<'_>, value: &str, encoding: Option<&str>) -> PyResult<Py<Self>> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(py, &bytes)
    }

    #[getter]
    fn root(&self) -> Py<Transaction> {
        self.root.clone()
    }

    #[getter]
    fn children(&self) -> PyObject {
        self.children.clone()
    }

    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<TransactionTreeIter>> {
        let mut queue = VecDeque::new();
        queue.push_front(Py::from(slf));
        Py::new(py, TransactionTreeIter { queue })
    }
}

#[pyclass]
pub struct TransactionTreeIter {
    queue: VecDeque<Py<TransactionTree>>,
}

#[pymethods]
impl TransactionTreeIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> Option<Py<Transaction>> {
        let node = match self.queue.pop_back() {
            Some(next) => {
                let next = next.borrow(py);
                for child in &next.children_raw {
                    self.queue.push_front(child.clone());
                }

                Some(next.root.clone())
            }
            None => None,
        };
        node
    }
}
