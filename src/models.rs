use std::sync::Arc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use ton_block::{Deserializable, Serializable};

use crate::abi::{convert_tokens, parse_tokens, AbiParam, AbiVersion};
use crate::util::{Encoding, HandleError};

#[derive(Clone)]
#[pyclass]
pub struct BlockchainConfig(pub Arc<ton_executor::BlockchainConfig>);

#[pymethods]
impl BlockchainConfig {
    #[getter]
    fn capabilities(&self) -> u64 {
        self.0.capabilites()
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
        let key = index.serialize().unwrap();
        Ok(
            if let Some(value) = config.get(key.into()).handle_runtime_error()? {
                value.remaining_references() != 0
            } else {
                false
            },
        )
    }

    fn get_raw_param(&self, index: u32) -> PyResult<Option<Cell>> {
        let config = &self.0.raw_config().config_params;
        let key = index.serialize().unwrap();
        let value = config.get(key.into()).handle_runtime_error()?;
        Ok(value.and_then(|slice| slice.reference_opt(0)).map(Cell))
    }

    fn __repr__(&self) -> String {
        format!(
            "<BlockchainConfig capabilities=0x{:016x}, global_version=0x{}>",
            self.capabilities(),
            self.0.global_version()
        )
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
        self.0.cells.0
    }

    #[getter]
    fn bits(&self) -> u64 {
        self.0.bits.0
    }

    #[getter]
    fn public_cells(&self) -> u64 {
        self.0.public_cells.0
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
            return Err(PyRuntimeError::new_err("Transaction without incoming message"));
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
        self.0.gas_used.0
    }

    #[getter]
    fn gas_limit(&self) -> u64 {
        self.0.gas_limit.0
    }

    #[getter]
    fn gas_credit(&self) -> Option<u32> {
        self.0.gas_credit.map(|credit| credit.0)
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
        ahash::RandomState::new().hash_one(self)
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
        ahash::RandomState::new().hash_one(self)
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
        ahash::RandomState::new().hash_one(self)
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
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(&bytes)
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
            ton_block::CommonMsgInfo::IntMsgInfo(x) => Some(x.created_at.0),
            ton_block::CommonMsgInfo::ExtOutMsgInfo(x) => Some(x.created_at.0),
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
        self.0.created_at.0
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
        self.0.created_at.0
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
        ahash::RandomState::new().hash_one(self)
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
    fn new(addr: &str) -> PyResult<Self> {
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

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Address('{}')", self.0)
    }

    fn __hash__(&self) -> u64 {
        ahash::RandomState::new().hash_one(&self.0)
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

        let tokens =
            nt::abi::unpack_from_cell(&params, self.0.clone().into(), allow_partial, abi_version)
                .handle_runtime_error()?;

        convert_tokens(py, tokens)
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

#[derive(Default, Copy, Clone)]
#[pyclass]
pub struct Tokens(pub i128);

impl From<ton_block::Grams> for Tokens {
    #[inline]
    fn from(value: ton_block::Grams) -> Self {
        Tokens(value.0 as i128)
    }
}

impl TryFrom<Tokens> for u128 {
    type Error = PyErr;

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        value.0.try_into().handle_value_error()
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
                    return Err(PyOverflowError::new_err(OVERFLOW))
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
        ahash::RandomState::new().hash_one(self.0)
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
