use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use rand::Rng;
use ton_block::{GetRepresentationHash, Serializable};

use crate::crypto::{KeyPair, PublicKey, Signature};
use crate::models::*;
use crate::transport::Clock;
use crate::util::*;

#[pyclass]
pub struct TransactionExecutor {
    clock: Option<Clock>,
    config: BlockchainConfig,
    check_signature: bool,
}

#[pymethods]
impl TransactionExecutor {
    #[new]
    fn new(config: BlockchainConfig, clock: Option<Clock>, check_signature: Option<bool>) -> Self {
        Self {
            clock,
            config,
            check_signature: check_signature.unwrap_or(true),
        }
    }

    #[setter]
    fn set_check_signature(&mut self, value: bool) {
        self.check_signature = value;
    }

    #[getter]
    fn get_check_signature(&self) -> bool {
        self.check_signature
    }

    fn execute(
        &self,
        message: &Message,
        account: Option<&AccountState>,
    ) -> PyResult<(Transaction, Option<AccountState>)> {
        use ton_executor::TransactionExecutor;

        const TRANSACTION_LT_OFFSET: u64 = 10;

        let (last_trans_lt, mut account) = match account {
            None => (0, ton_block::Account::AccountNone),
            Some(state) => (
                state.0.storage.last_trans_lt,
                ton_block::Account::Account(state.0.clone()),
            ),
        };

        let mut executor =
            ton_executor::OrdinaryTransactionExecutor::new(self.config.as_ref().clone());
        executor.set_signature_check_disabled(!self.check_signature);

        let clock = match &self.clock {
            Some(clock) => clock.as_ref(),
            None => &nt::utils::SimpleClock,
        };

        let block_unixtime = clock.now_sec_u64() as u32;
        let block_lt = last_trans_lt + TRANSACTION_LT_OFFSET;

        let params = ton_executor::ExecuteParams {
            block_unixtime,
            block_lt,
            last_tr_lt: Arc::new(AtomicU64::new(block_lt)),
            seed_block: ton_types::UInt256::from(rand::thread_rng().gen::<[u8; 32]>()),
            behavior_modifiers: Some(executor.behavior_modifiers()),
            ..Default::default()
        };

        let transaction = executor
            .execute_with_params(Some(&message.data), &mut account, params)
            .handle_runtime_error()?;
        let hash = transaction.hash().handle_runtime_error()?;

        if executor
            .config()
            .has_capability(ton_block::GlobalCapabilities::CapFastStorageStat)
        {
            account.update_storage_stat_fast().handle_runtime_error()?;
        } else {
            account.update_storage_stat().handle_runtime_error()?;
        }

        let account_state = match account {
            ton_block::Account::AccountNone => None,
            ton_block::Account::Account(state) => Some(AccountState(state)),
        };

        Ok((
            Transaction::try_from(nt::transport::models::RawTransaction {
                data: transaction,
                hash,
            })?,
            account_state,
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "<TransactionExecutor check_signature={}>",
            DisplayBool(self.check_signature)
        )
    }
}

#[derive(Clone)]
#[pyclass]
pub struct ContractAbi(Arc<SharedContractAbi>);

#[pymethods]
impl ContractAbi {
    #[staticmethod]
    fn from_file(path: PathBuf) -> PyResult<Self> {
        let abi = std::fs::read_to_string(path).handle_runtime_error()?;
        Self::new(&abi)
    }

    #[new]
    fn new(abi: &str) -> PyResult<Self> {
        let contract = ton_abi::Contract::load(abi.trim()).handle_value_error()?;

        let functions = contract
            .functions
            .iter()
            .map(|(name, abi)| (name.clone(), FunctionAbi(Arc::new(abi.clone()))))
            .collect();

        let events = contract
            .events
            .iter()
            .map(|(name, abi)| (name.clone(), EventAbi(Arc::new(abi.clone()))))
            .collect();

        let shared = Arc::new(SharedContractAbi {
            contract,
            functions,
            events,
        });

        Ok(Self(shared))
    }

    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.contract.abi_version)
    }

    fn get_function(&self, name: &str) -> Option<FunctionAbi> {
        self.0.functions.get(name).cloned()
    }

    fn get_event(&self, name: &str) -> Option<EventAbi> {
        self.0.events.get(name).cloned()
    }

    fn encode_init_data(
        &self,
        data: &PyDict,
        public_key: Option<&PublicKey>,
        existing_data: Option<Cell>,
    ) -> PyResult<Cell> {
        let mut map = ton_types::HashmapE::with_hashmap(
            ton_abi::Contract::DATA_MAP_KEYLEN,
            existing_data.and_then(|Cell(cell)| cell.reference(0).ok()),
        );

        if let Some(public_key) = public_key {
            map.set_builder(
                serialize_state_init_data_key(0),
                ton_types::BuilderData::new()
                    .append_raw(public_key.0.as_bytes(), 256)
                    .unwrap(),
            )
            .handle_runtime_error()?;
        }

        if !self.0.contract.data.is_empty() {
            for (param_name, param) in &self.0.contract.data {
                let value = match data.get_item(param_name) {
                    Some(value) => parse_token(&param.value.kind, value)?,
                    None => {
                        return Err(PyValueError::new_err(format!(
                            "Param '{param_name}' not found"
                        )))
                    }
                };

                let builder = value
                    .pack_into_chain(&self.0.contract.abi_version)
                    .handle_runtime_error()?;

                map.set_builder(serialize_state_init_data_key(param.key), &builder)
                    .handle_runtime_error()?;
            }
        }

        map.write_to_new_cell()
            .and_then(ton_types::BuilderData::into_cell)
            .handle_runtime_error()
            .map(Cell)
    }

    fn decode_init_data<'a>(
        &self,
        py: Python<'a>,
        data: &Cell,
    ) -> PyResult<(Option<PublicKey>, &'a PyDict)> {
        let pubkey = {
            let map = ton_types::HashmapE::with_hashmap(
                ton_abi::Contract::DATA_MAP_KEYLEN,
                data.0.reference(0).ok(),
            );

            let value = map
                .get(serialize_state_init_data_key(0))
                .handle_value_error()?;
            match value {
                Some(mut value) => {
                    let pubkey = value.get_next_hash().handle_value_error()?;
                    if pubkey.is_zero() {
                        None
                    } else {
                        Some(PublicKey(
                            ed25519_dalek::PublicKey::from_bytes(pubkey.as_slice())
                                .handle_value_error()?,
                        ))
                    }
                }
                None => None,
            }
        };

        let data = ton_types::SliceData::load_cell_ref(&data.0).handle_value_error()?;

        let tokens = self.0.contract.decode_data(data).handle_value_error()?;
        Ok((pubkey, convert_tokens(py, tokens)?))
    }

    fn decode_fields<'a>(
        &self,
        py: Python<'a>,
        data: DataOrState<'a>,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let data = match data {
            DataOrState::Data(cell) => cell.0.clone(),
            DataOrState::State(state) => match &state.0.storage.state {
                ton_block::AccountState::AccountActive { state_init } => match state_init.data() {
                    Some(cell) => cell.clone(),
                    None => return Err(PyValueError::new_err("Account state without data")),
                },
                ton_block::AccountState::AccountFrozen { .. } => {
                    return Err(PyValueError::new_err("Account frozen"));
                }
                ton_block::AccountState::AccountUninit => {
                    return Err(PyValueError::new_err("Account not deployed"));
                }
            },
        };
        let data = ton_types::SliceData::load_cell(data).handle_value_error()?;

        let contract = &self.0.contract;
        let tokens = ton_abi::TokenValue::decode_params(
            &contract.fields,
            data,
            &contract.abi_version,
            allow_partial.unwrap_or_default(),
        )
        .handle_value_error()?;
        convert_tokens(py, tokens)
    }

    fn decode_transaction(
        &self,
        py: Python<'_>,
        transaction: &Transaction,
    ) -> PyResult<Option<Py<FunctionCallFull>>> {
        use ton_block::Deserializable;

        let contract = &self.0.contract;
        let tx = &transaction.0.data;

        let Some(in_msg) = tx.read_in_msg().handle_runtime_error()? else {
            return Ok(None);
        };
        let Some(in_msg_body) = in_msg.body() else {
            return Ok(None);
        };

        let function = match nt::abi::guess_method_by_input(
            contract,
            &in_msg_body,
            &nt::abi::MethodName::Guess,
            in_msg.is_internal(),
        )
        .handle_value_error()?
        {
            Some(function) => self.0.functions.get(&function.name).unwrap().clone(),
            None => return Ok(None),
        };

        let input = function
            .0
            .decode_input(in_msg_body, in_msg.is_internal())
            .handle_runtime_error()?;

        let mut output = None;
        let mut events = Vec::new();
        tx.out_msgs
            .iterate_slices(|value| {
                let msg_cell = value.reference(0)?;
                let msg = ton_block::Message::construct_from_cell(msg_cell)?;
                if !msg.is_outbound_external() {
                    return Ok(true);
                }
                let Some(msg_body) = msg.body() else {
                    return Ok(true);
                };

                if let Ok(id) = nt::abi::read_function_id(&msg_body) {
                    if id == function.0.output_id {
                        output = Some(function.0.decode_output(msg_body, false)?);
                    } else if let Ok(event) = contract.event_by_id(id) {
                        let event = self.0.events.get(&event.name).unwrap();
                        let input = event.0.decode_input(msg_body)?;
                        events.push((event, input));
                    }
                }

                Ok(true)
            })
            .handle_runtime_error()?;

        let output = match output {
            Some(x) => x,
            None if !function.0.has_output() => Default::default(),
            None => return Err(PyRuntimeError::new_err("No output messages produced")),
        };

        let events = events
            .into_iter()
            .map(|(event, input)| {
                let event = Py::new(py, event.clone())?;
                PyResult::Ok((event, convert_tokens(py, input)?))
            })
            .collect::<PyResult<Vec<(Py<EventAbi>, &PyDict)>>>()?;

        let function_call = FunctionCall {
            input: convert_tokens(py, input)?.into_py(py),
            output: convert_tokens(py, output)?.into_py(py),
        };

        let events = PyList::new(py, events).into_py(py);

        Py::new(
            py,
            PyClassInitializer::from(function_call)
                .add_subclass(FunctionCallFull { function, events }),
        )
        .map(Some)
    }

    fn decode_transaction_events<'a>(
        &self,
        py: Python<'a>,
        transaction: &Transaction,
    ) -> PyResult<Vec<(EventAbi, &'a PyDict)>> {
        use ton_block::Deserializable;

        let contract = &self.0.contract;
        let tx = &transaction.0.data;

        let mut events = Vec::new();
        tx.out_msgs
            .iterate_slices(|value| {
                let msg_cell = value.reference(0)?;
                let msg = ton_block::Message::construct_from_cell(msg_cell)?;
                if !msg.is_outbound_external() {
                    return Ok(true);
                }
                let Some(msg_body) = msg.body() else {
                    return Ok(true);
                };

                if let Ok(id) = nt::abi::read_function_id(&msg_body) {
                    if let Ok(event) = contract.event_by_id(id) {
                        let event = self.0.events.get(&event.name).unwrap();
                        let input = event.0.decode_input(msg_body)?;
                        events.push((event, input));
                    }
                }

                Ok(true)
            })
            .handle_runtime_error()?;

        events
            .into_iter()
            .map(|(event, input)| PyResult::Ok((event.clone(), convert_tokens(py, input)?)))
            .collect::<PyResult<Vec<_>>>()
    }
}

struct SharedContractAbi {
    contract: ton_abi::Contract,
    functions: FastHashMap<String, FunctionAbi>,
    events: FastHashMap<String, EventAbi>,
}

#[derive(FromPyObject)]
enum DataOrState<'a> {
    #[pyo3(transparent, annotation = "Cell")]
    Data(PyRef<'a, Cell>),
    #[pyo3(transparent, annotation = "AccountState")]
    State(PyRef<'a, AccountState>),
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct FunctionAbi(Arc<ton_abi::Function>);

#[pymethods]
impl FunctionAbi {
    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.abi_version)
    }

    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn input_id(&self) -> u32 {
        self.0.input_id
    }

    #[getter]
    fn output_id(&self) -> u32 {
        self.0.output_id
    }

    fn with_args(&self, py: Python<'_>, args: &PyDict) -> FunctionAbiWithArgs {
        FunctionAbiWithArgs {
            abi: self.clone(),
            args: args.into_py(py),
        }
    }

    fn call(
        &self,
        py: Python<'_>,
        account_state: &AccountState,
        input: &PyDict,
        responsible: Option<bool>,
        clock: Option<&Clock>,
        config: Option<BlockchainConfig>,
    ) -> PyResult<ExecutionOutput> {
        use nt::abi::FunctionExt;

        let input = parse_tokens(&self.0.inputs, input)?;
        let clock = match clock {
            Some(clock) => clock.as_ref(),
            None => &nt::utils::SimpleClock,
        };

        let config = match &config {
            Some(config) => nt::abi::BriefBlockchainConfig::from(config.as_ref()),
            None => nt::abi::BriefBlockchainConfig::default(),
        };

        let responsible = matches!(responsible, Some(true));
        let execution_output = self
            .0
            .run_local_ext(clock, account_state.0.clone(), &input, responsible, &config)
            .handle_runtime_error()?;

        Ok(ExecutionOutput {
            exit_code: execution_output.result_code,
            output: execution_output
                .tokens
                .map(|tokens| PyResult::Ok(convert_tokens(py, tokens)?.into_py(py)))
                .transpose()?,
        })
    }

    fn encode_external_message(
        &self,
        dst: Address,
        input: &PyDict,
        public_key: Option<&PublicKey>,
        state_init: Option<&StateInit>,
        timeout: Option<u32>,
        clock: Option<&Clock>,
    ) -> PyResult<UnsignedExternalMessage> {
        let body = self.encode_external_input(input, public_key, timeout, Some(&dst), clock)?;
        Ok(UnsignedExternalMessage {
            dst: dst.0,
            state_init: state_init.cloned(),
            body,
        })
    }

    fn encode_external_input(
        &self,
        input: &PyDict,
        public_key: Option<&PublicKey>,
        timeout: Option<u32>,
        address: Option<&Address>,
        clock: Option<&Clock>,
    ) -> PyResult<UnsignedBody> {
        use nt::utils::Clock;

        let tokens = parse_tokens(&self.0.inputs, input)?;

        let now = match clock {
            Some(clock) => clock.0.now_ms_u64(),
            None => nt::utils::SimpleClock.now_ms_u64(),
        };
        let (expire_at, headers) = default_headers(
            now,
            nt::core::models::Expiration::Timeout(timeout.unwrap_or(DEFAULT_TIMEOUT)),
            public_key.map(|key| key.0),
        );

        let (payload, hash) = self
            .0
            .create_unsigned_call(
                &headers,
                &tokens,
                false,
                true,
                address.map(|addr| addr.0.clone()),
            )
            .handle_runtime_error()?;

        Ok(UnsignedBody {
            abi_version: self.0.abi_version,
            payload,
            hash,
            expire_at: expire_at.timestamp,
        })
    }

    fn encode_internal_message(
        &self,
        input: &PyDict,
        value: Tokens,
        bounce: bool,
        dst: Address,
        src: Option<Address>,
        state_init: Option<&StateInit>,
    ) -> PyResult<Message> {
        let value = value.try_into()?;
        let body = self.encode_internal_input(input)?;
        let body: ton_types::SliceData =
            ton_types::SliceData::load_cell(body.0).handle_value_error()?;

        let value = ton_block::Grams::new(value).handle_value_error()?;
        let mut message = ton_block::Message::with_int_header(ton_block::InternalMessageHeader {
            ihr_disabled: true,
            bounce,
            value: ton_block::CurrencyCollection::from_grams(value),
            src: src
                .map(|src| ton_block::MsgAddressIntOrNone::Some(src.0))
                .unwrap_or(ton_block::MsgAddressIntOrNone::None),
            dst: dst.0,
            ..Default::default()
        });

        if let Some(state_init) = state_init {
            message.set_state_init(state_init.0.clone())
        }

        message.set_body(body);

        let hash = message.hash().handle_runtime_error()?;

        Ok(Message {
            data: message,
            hash,
        })
    }

    fn encode_internal_input(&self, input: &PyDict) -> PyResult<Cell> {
        let tokens = parse_tokens(&self.0.inputs, input)?;
        let input = self
            .0
            .encode_internal_input(&tokens)
            .handle_runtime_error()?;
        input.into_cell().map(Cell).handle_runtime_error()
    }

    fn decode_transaction(
        &self,
        py: Python<'_>,
        transaction: &Transaction,
    ) -> PyResult<FunctionCall> {
        use nt::abi::FunctionExt;

        let tx = &transaction.0.data;

        let Some(in_msg) = tx.read_in_msg().handle_runtime_error()? else {
            return Err(PyRuntimeError::new_err(
                "Transaction without incoming message",
            ));
        };
        let Some(in_msg_body) = in_msg.body() else {
            return Err(PyRuntimeError::new_err("Incoming message without body"));
        };

        let input = self
            .0
            .decode_input(in_msg_body, in_msg.is_internal())
            .handle_runtime_error()?;
        let output = self.0.parse(tx).handle_runtime_error()?;

        Ok(FunctionCall {
            input: convert_tokens(py, input)?.into_py(py),
            output: convert_tokens(py, output)?.into_py(py),
        })
    }

    fn decode_input<'a>(
        &self,
        py: Python<'a>,
        message_body: &Cell,
        internal: bool,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let abi = self.0.as_ref();
        let body = ton_types::SliceData::load_cell_ref(&message_body.0).handle_value_error()?;
        let values = if matches!(allow_partial, Some(true)) {
            abi.decode_input_partial(body, internal)
        } else {
            abi.decode_input(body, internal)
        }
        .handle_runtime_error()?;

        convert_tokens(py, values)
    }

    fn decode_output<'a>(
        &self,
        py: Python<'a>,
        message_body: &Cell,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let abi = self.0.as_ref();
        let body = ton_types::SliceData::load_cell_ref(&message_body.0).handle_value_error()?;
        let values = if matches!(allow_partial, Some(true)) {
            abi.decode_output_partial(body, false)
        } else {
            abi.decode_output(body, false)
        }
        .handle_runtime_error()?;

        convert_tokens(py, values)
    }

    fn __repr__(&self) -> String {
        format!(
            "<FunctionAbi name='{}', input_id=0x{:08x}, output_id=0x{:08x}>",
            self.0.name, self.0.input_id, self.0.output_id
        )
    }

    fn __hash__(&self) -> u64 {
        self.0.input_id as u64
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.0.eq(&other.0),
            pyo3::basic::CompareOp::Ne => !self.0.eq(&other.0),
            pyo3::basic::CompareOp::Lt => self.0.input_id < other.0.input_id,
            pyo3::basic::CompareOp::Le => self.0.input_id <= other.0.input_id,
            pyo3::basic::CompareOp::Gt => self.0.input_id > other.0.input_id,
            pyo3::basic::CompareOp::Ge => self.0.input_id >= other.0.input_id,
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct FunctionAbiWithArgs {
    abi: FunctionAbi,
    args: Py<PyDict>,
}

#[pymethods]
impl FunctionAbiWithArgs {
    #[getter]
    fn abi(&self) -> FunctionAbi {
        self.abi.clone()
    }

    #[getter]
    fn args(&self) -> Py<PyDict> {
        self.args.clone()
    }

    fn call(
        &self,
        py: Python<'_>,
        account_state: &AccountState,
        responsible: Option<bool>,
        clock: Option<&Clock>,
        config: Option<BlockchainConfig>,
    ) -> PyResult<ExecutionOutput> {
        let input = self.args.as_ref(py);
        self.abi
            .call(py, account_state, input, responsible, clock, config)
    }

    fn encode_external_message(
        &self,
        py: Python<'_>,
        dst: Address,
        public_key: Option<&PublicKey>,
        state_init: Option<&StateInit>,
        timeout: Option<u32>,
        clock: Option<&Clock>,
    ) -> PyResult<UnsignedExternalMessage> {
        let input = self.args.as_ref(py);
        self.abi
            .encode_external_message(dst, input, public_key, state_init, timeout, clock)
    }

    fn encode_external_input(
        &self,
        py: Python<'_>,
        public_key: Option<&PublicKey>,
        timeout: Option<u32>,
        address: Option<&Address>,
        clock: Option<&Clock>,
    ) -> PyResult<UnsignedBody> {
        let input = self.args.as_ref(py);
        self.abi
            .encode_external_input(input, public_key, timeout, address, clock)
    }

    fn encode_internal_message(
        &self,
        py: Python<'_>,
        value: Tokens,
        bounce: bool,
        dst: Address,
        src: Option<Address>,
        state_init: Option<&StateInit>,
    ) -> PyResult<Message> {
        let input = self.args.as_ref(py);
        self.abi
            .encode_internal_message(input, value, bounce, dst, src, state_init)
    }

    fn encode_internal_input(&self, py: Python<'_>) -> PyResult<Cell> {
        let input = self.args.as_ref(py);
        self.abi.encode_internal_input(input)
    }

    fn __repr__(&self) -> String {
        format!(
            "<FunctionAbiWithArgs name='{}', input_id=0x{:08x}, output_id=0x{:08x}>",
            self.abi.name(),
            self.abi.input_id(),
            self.abi.output_id()
        )
    }

    fn __hash__(&self, py: Python<'_>) -> PyResult<u64> {
        #[derive(Hash)]
        struct HashHelper {
            input_id: u64,
            args_hash: u64,
        }

        let args = self.args.as_ref(py);

        Ok(make_hasher().hash_one(HashHelper {
            input_id: self.abi.input_id() as u64,
            args_hash: args.hash()? as u64,
        }))
    }

    fn __richcmp__(
        &self,
        py: Python<'_>,
        other: &Self,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<bool> {
        let args = self.args.as_ref(py);
        let other_args = other.args.as_ref(py);

        Ok(self.abi.__richcmp__(&other.abi, op) && args.rich_compare(other_args, op)?.is_true()?)
    }
}

#[pyclass(get_all)]
pub struct ExecutionOutput {
    exit_code: i32,
    output: Option<Py<PyDict>>,
}

#[pymethods]
impl ExecutionOutput {
    fn __repr__(&self) -> String {
        format!(
            "<ExecutionOutput exit_code={}, has_output={}>",
            self.exit_code,
            DisplayBool(self.output.is_some())
        )
    }
}

#[pyclass(subclass, get_all)]
pub struct FunctionCall {
    input: Py<PyDict>,
    output: Py<PyDict>,
}

#[pyclass(extends = FunctionCall, get_all)]
pub struct FunctionCallFull {
    function: FunctionAbi,
    events: Py<PyList>,
}

#[pymethods]
impl FunctionCallFull {
    fn __repr__(&self) -> String {
        format!("<FunctionCallFull function={}>", self.function.0.name)
    }
}

const DEFAULT_TIMEOUT: u32 = 60;

#[derive(Clone)]
#[pyclass]
pub struct EventAbi(Arc<ton_abi::Event>);

#[pymethods]
impl EventAbi {
    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.abi_version)
    }

    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn id(&self) -> u32 {
        self.0.id
    }

    fn decode_message<'a>(&self, py: Python<'a>, message: &Message) -> PyResult<&'a PyDict> {
        let Some(body) = message.data.body() else {
            return Err(PyValueError::new_err("Message without body"));
        };
        if !message.data.is_outbound_external() {
            return Err(PyValueError::new_err("Message is not an external outbound"));
        }
        let values = self.0.decode_input(body).handle_runtime_error()?;
        convert_tokens(py, values)
    }

    fn decode_message_body<'a>(&self, py: Python<'a>, message_body: &Cell) -> PyResult<&'a PyDict> {
        let body = ton_types::SliceData::load_cell_ref(&message_body.0).handle_value_error()?;
        let values = self.0.decode_input(body).handle_runtime_error()?;
        convert_tokens(py, values)
    }

    fn __repr__(&self) -> String {
        format!("<EventAbi name='{}', id=0x{:08x}>", self.0.name, self.0.id)
    }

    fn __hash__(&self) -> u64 {
        self.0.id as u64
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.0.eq(&other.0),
            pyo3::basic::CompareOp::Ne => !self.0.eq(&other.0),
            pyo3::basic::CompareOp::Lt => self.0.id < other.0.id,
            pyo3::basic::CompareOp::Le => self.0.id <= other.0.id,
            pyo3::basic::CompareOp::Gt => self.0.id > other.0.id,
            pyo3::basic::CompareOp::Ge => self.0.id >= other.0.id,
        }
    }
}

#[pyclass(extends = Message)]
pub struct SignedExternalMessage {
    pub expire_at: u32,
}

#[pymethods]
impl SignedExternalMessage {
    #[new]
    fn new(
        dst: Address,
        expire_at: u32,
        body: Option<Cell>,
        state_init: Option<StateInit>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let mut message =
            ton_block::Message::with_ext_in_header(ton_block::ExternalInboundMessageHeader {
                dst: dst.0,
                ..Default::default()
            });

        if let Some(body) = body {
            message.set_body(ton_types::SliceData::load_cell(body.0).handle_value_error()?);
        }
        if let Some(state_init) = state_init {
            message.set_state_init(state_init.0.clone())
        }

        let hash = message.hash().handle_runtime_error()?;

        Ok(PyClassInitializer::from(Message {
            data: message,
            hash,
        })
        .add_subclass(SignedExternalMessage { expire_at }))
    }

    #[getter]
    fn expire_at(&self) -> u32 {
        self.expire_at
    }

    fn split(slf: PyRef<'_, Self>) -> (Message, u32) {
        let expire_at = slf.expire_at;
        (slf.into_super().clone(), expire_at)
    }

    fn __repr__(slf: PyRef<'_, Self>) -> String {
        let expire_at = slf.expire_at;
        let message = slf.into_super();
        format!(
            "<SignedExternalMessage hash='{:x}', expire_at={}, ExternalIn>",
            message.hash, expire_at
        )
    }
}

#[pyclass]
pub struct UnsignedExternalMessage {
    dst: ton_block::MsgAddressInt,
    state_init: Option<StateInit>,
    body: UnsignedBody,
}

impl UnsignedExternalMessage {
    fn fill_body(&self, py: Python<'_>, body: Cell) -> PyResult<Py<SignedExternalMessage>> {
        let mut message =
            ton_block::Message::with_ext_in_header(ton_block::ExternalInboundMessageHeader {
                dst: self.dst.clone(),
                ..Default::default()
            });

        if let Some(state_init) = &self.state_init {
            message.set_state_init(state_init.0.clone())
        }

        let body = ton_types::SliceData::load_cell(body.0).handle_runtime_error()?;

        message.set_body(body);

        let hash = message.hash().handle_runtime_error()?;

        Py::new(
            py,
            PyClassInitializer::from(Message {
                data: message,
                hash,
            })
            .add_subclass(SignedExternalMessage {
                expire_at: self.body.expire_at(),
            }),
        )
    }
}

#[pymethods]
impl UnsignedExternalMessage {
    #[getter]
    fn hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        self.body.hash(py)
    }

    #[getter]
    fn expire_at(&self) -> u32 {
        self.body.expire_at()
    }

    #[getter]
    fn get_state_init(&self) -> Option<StateInit> {
        self.state_init.clone()
    }

    #[setter]
    fn set_state_init(&mut self, state_init: Option<StateInit>) {
        self.state_init = state_init;
    }

    fn sign(
        &self,
        py: Python<'_>,
        keypair: &KeyPair,
        signature_id: Option<i32>,
    ) -> PyResult<Py<SignedExternalMessage>> {
        self.fill_body(py, self.body.sign(keypair, signature_id)?)
    }

    fn with_signature(
        &self,
        py: Python<'_>,
        signature: &Signature,
    ) -> PyResult<Py<SignedExternalMessage>> {
        self.fill_body(py, self.body.with_signature(signature)?)
    }

    fn with_fake_signature(&self, py: Python<'_>) -> PyResult<Py<SignedExternalMessage>> {
        self.fill_body(py, self.body.with_fake_signature()?)
    }

    fn without_signature(&self, py: Python<'_>) -> PyResult<Py<SignedExternalMessage>> {
        self.fill_body(py, self.body.without_signature()?)
    }

    fn __repr__(&self) -> String {
        format!(
            "<UnsignedExternalMessage hash='{:x}', expire_at={}>",
            self.body.hash, self.body.expire_at
        )
    }
}

#[pyclass]
pub struct UnsignedBody {
    abi_version: ton_abi::contract::AbiVersion,
    payload: ton_types::BuilderData,
    hash: ton_types::UInt256,
    expire_at: u32,
}

impl UnsignedBody {
    fn fill_signature(&self, signature: Option<&[u8]>) -> PyResult<Cell> {
        let payload =
            ton_abi::Function::fill_sign(&self.abi_version, signature, None, self.payload.clone())
                .handle_runtime_error()?;
        payload.into_cell().handle_runtime_error().map(Cell)
    }
}

#[pymethods]
impl UnsignedBody {
    #[getter]
    fn hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.hash.as_slice())
    }

    #[getter]
    fn expire_at(&self) -> u32 {
        self.expire_at
    }

    fn sign(&self, keypair: &KeyPair, signature_id: Option<i32>) -> PyResult<Cell> {
        let signature = keypair.sign_raw(self.hash.as_ref(), signature_id);
        self.fill_signature(Some(signature.0.as_ref()))
    }

    fn with_signature(&self, signature: &Signature) -> PyResult<Cell> {
        self.fill_signature(Some(signature.0.as_ref()))
    }

    fn with_fake_signature(&self) -> PyResult<Cell> {
        self.fill_signature(Some(&[0u8; 64]))
    }

    fn without_signature(&self) -> PyResult<Cell> {
        self.fill_signature(None)
    }

    fn __repr__(&self) -> String {
        format!(
            "<UnsignedBody hash='{:x}', expire_at={}>",
            self.hash, self.expire_at
        )
    }
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct AbiParam {
    pub param: ton_abi::ParamType,
}

macro_rules! define_abi_types {
    ($($ident:ident = |$($arg:ident: $arg_ty:ty),*| $res:expr),*$(,)?) => {$(
        #[pyclass(extends = AbiParam)]
        pub struct $ident;

        #[pymethods]
        impl $ident {
            #[new]
            fn new($($arg: $arg_ty),*) -> (Self, AbiParam) {
                let base = AbiParam {
                    param: $res,
                };
                (Self, base)
            }
        }
    )*};
}

define_abi_types! {
    AbiUint = |size: usize| ton_abi::ParamType::Uint(size),
    AbiInt = |size: usize| ton_abi::ParamType::Int(size),
    AbiVarUint = |size: usize| ton_abi::ParamType::VarUint(size),
    AbiVarInt = |size: usize| ton_abi::ParamType::VarInt(size),
    AbiBool = | | ton_abi::ParamType::Bool,
    AbiTuple = |items: Vec<(String, AbiParam)>| {
        ton_abi::ParamType::Tuple(
            items
                .into_iter()
                .map(|(name, AbiParam { param })| {
                    ton_abi::Param {
                        name,
                        kind: param,
                    }
                })
                .collect()
        )
    },
    AbiArray = |value_type: AbiParam| ton_abi::ParamType::Array(Box::new(value_type.param)),
    AbiFixedArray = |value_type: AbiParam, len: usize| {
        ton_abi::ParamType::FixedArray(Box::new(value_type.param), len)
    },
    AbiCell = | | ton_abi::ParamType::Cell,
    AbiMap = |key_type: AbiParam, value_type: AbiParam| {
        let key_type = Box::new(key_type.param);
        let value_type = Box::new(value_type.param);
        ton_abi::ParamType::Map(key_type, value_type)
    },
    AbiAddress = | | ton_abi::ParamType::Address,
    AbiBytes = | | ton_abi::ParamType::Bytes,
    AbiFixedBytes = |len: usize| ton_abi::ParamType::FixedBytes(len),
    AbiString = | | ton_abi::ParamType::String,
    AbiToken = | | ton_abi::ParamType::Token,
    AbiOptional = |value_type: AbiParam| {
        ton_abi::ParamType::Optional(Box::new(value_type.param))
    },
    AbiRef = |value_type: AbiParam| {
        ton_abi::ParamType::Ref(Box::new(value_type.param))
    },
}

#[derive(Copy, Clone)]
#[pyclass]
pub struct AbiVersion(pub ton_abi::contract::AbiVersion);

#[pymethods]
impl AbiVersion {
    #[new]
    fn new(major: u8, minor: u8) -> Self {
        Self(ton_abi::contract::AbiVersion { major, minor })
    }

    #[getter]
    fn get_major(&self) -> u8 {
        self.0.major
    }

    #[setter]
    fn set_major(&mut self, value: u8) {
        self.0.major = value;
    }

    #[getter]
    fn get_minor(&self) -> u8 {
        self.0.minor
    }

    #[setter]
    fn set_minor(&mut self, value: u8) {
        self.0.minor = value;
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("AbiVersion({}, {})", self.0.major, self.0.minor)
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes([self.0.minor, self.0.major, 0, 0, 0, 0, 0, 0])
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches((self.0.major, self.0.minor).cmp(&(other.0.major, other.0.minor)))
    }
}

pub fn parse_tokens(params: &[ton_abi::Param], value: &PyDict) -> PyResult<Vec<ton_abi::Token>> {
    let mut result = Vec::with_capacity(params.len());
    for param in params {
        let value = match value.get_item(param.name.as_str()) {
            Some(value) => parse_token(&param.kind, value)?,
            None => {
                return Err(PyRuntimeError::new_err(format!(
                    "Param '{}' not found",
                    param.name
                )));
            }
        };
        result.push(ton_abi::Token::new(&param.name, value));
    }
    Ok(result)
}

fn parse_token(param: &ton_abi::ParamType, value: &PyAny) -> PyResult<ton_abi::TokenValue> {
    use pyo3::types::*;

    Ok(match param {
        ton_abi::ParamType::Uint(size) => {
            let number = 'number: {
                match *size {
                    64 | 128 => {
                        if let Ok(tokens) = value.extract::<PyRef<Tokens>>() {
                            break 'number num_bigint::BigUint::try_from(tokens.0)
                                .handle_value_error()?;
                        }
                    }
                    256 => {
                        if let Ok(public_key) = value.extract::<PyRef<PublicKey>>() {
                            break 'number num_bigint::BigUint::from_bytes_be(
                                public_key.0.as_bytes(),
                            );
                        }
                    }
                    _ => {}
                }

                value.extract::<num_bigint::BigUint>()?
            };

            ton_abi::TokenValue::Uint(ton_abi::Uint {
                number,
                size: *size,
            })
        }
        ton_abi::ParamType::Int(size) => {
            let number = 'number: {
                match *size {
                    64 | 128 => {
                        if let Ok(tokens) = value.extract::<PyRef<Tokens>>() {
                            break 'number num_bigint::BigInt::from(tokens.0);
                        }
                    }
                    _ => {}
                }

                value.extract::<num_bigint::BigInt>()?
            };

            ton_abi::TokenValue::Int(ton_abi::Int {
                number,
                size: *size,
            })
        }
        ton_abi::ParamType::VarUint(size) => {
            let number = value.extract::<num_bigint::BigUint>()?;
            ton_abi::TokenValue::VarUint(*size, number)
        }
        ton_abi::ParamType::VarInt(size) => {
            let number = value.extract::<num_bigint::BigInt>()?;
            ton_abi::TokenValue::VarInt(*size, number)
        }
        ton_abi::ParamType::Bool => {
            let value = value.extract::<bool>()?;
            ton_abi::TokenValue::Bool(value)
        }
        ton_abi::ParamType::Tuple(types) => {
            let value = value.extract::<&PyDict>()?;
            ton_abi::TokenValue::Tuple(parse_tokens(types, value)?)
        }
        ton_abi::ParamType::Array(ty) => {
            let list = value.extract::<&PyList>()?;
            let mut values = Vec::with_capacity(list.len());
            for value in list {
                values.push(parse_token(ty.as_ref(), value)?);
            }
            ton_abi::TokenValue::Array(*ty.clone(), values)
        }
        ton_abi::ParamType::FixedArray(ty, len) => {
            let list = value.extract::<&PyList>()?;
            let list_len = list.len();
            if list_len != *len {
                return Err(PyValueError::new_err("Invalid fixed array length"));
            }
            let mut values = Vec::with_capacity(list_len);
            for value in list {
                values.push(parse_token(ty.as_ref(), value)?);
            }
            ton_abi::TokenValue::FixedArray(*ty.clone(), values)
        }
        ton_abi::ParamType::Cell => {
            let Cell(value) = value.extract::<Cell>()?;
            ton_abi::TokenValue::Cell(value)
        }
        ton_abi::ParamType::Map(key_ty, value_ty) => {
            let list = value.extract::<&PyList>()?;
            let mut result = BTreeMap::new();
            for item in list {
                let (key, value) = parse_map_entry_token(key_ty, value_ty, item)?;
                result.insert(key, value);
            }
            ton_abi::TokenValue::Map(*key_ty.clone(), *value_ty.clone(), result)
        }
        ton_abi::ParamType::Address => {
            let Address(addr) = value.extract::<Address>()?;
            ton_abi::TokenValue::Address(match addr {
                ton_block::MsgAddressInt::AddrStd(addr) => ton_block::MsgAddress::AddrStd(addr),
                ton_block::MsgAddressInt::AddrVar(addr) => ton_block::MsgAddress::AddrVar(addr),
            })
        }
        ton_abi::ParamType::Bytes => {
            let bytes = value.extract::<&[u8]>()?;
            ton_abi::TokenValue::Bytes(bytes.to_vec())
        }
        ton_abi::ParamType::FixedBytes(len) => {
            let bytes = value.extract::<&[u8]>()?;
            if bytes.len() != *len {
                return Err(PyValueError::new_err("Invalid fixed bytes length"));
            }
            ton_abi::TokenValue::FixedBytes(bytes.to_vec())
        }
        ton_abi::ParamType::String => {
            let value = value.extract::<String>()?;
            ton_abi::TokenValue::String(value)
        }
        ton_abi::ParamType::Token => {
            let value = 'number: {
                if let Ok(tokens) = value.extract::<PyRef<Tokens>>() {
                    break 'number tokens.0.try_into().handle_value_error()?;
                }

                value.extract::<u128>()?
            };

            let value = ton_block::Grams::new(value).handle_runtime_error()?;
            ton_abi::TokenValue::Token(value)
        }
        ton_abi::ParamType::Time => value.extract::<u64>().map(ton_abi::TokenValue::Time)?,
        ton_abi::ParamType::Expire => value.extract::<u32>().map(ton_abi::TokenValue::Expire)?,
        ton_abi::ParamType::PublicKey => {
            let value = if value.is_none() {
                None
            } else {
                let value = hex::decode(value.extract::<&str>()?).handle_runtime_error()?;
                Some(ed25519_dalek::PublicKey::from_bytes(&value).handle_runtime_error()?)
            };
            ton_abi::TokenValue::PublicKey(value)
        }
        ton_abi::ParamType::Optional(ty) => {
            let value = if value.is_none() {
                None
            } else {
                Some(parse_token(ty.as_ref(), value).map(Box::new)?)
            };
            ton_abi::TokenValue::Optional(*ty.clone(), value)
        }
        ton_abi::ParamType::Ref(ty) => {
            ton_abi::TokenValue::Ref(parse_token(ty.as_ref(), value).map(Box::new)?)
        }
    })
}

fn parse_map_entry_token(
    key_ty: &ton_abi::ParamType,
    value_ty: &ton_abi::ParamType,
    item: &PyAny,
) -> PyResult<(ton_abi::MapKeyTokenValue, ton_abi::TokenValue)> {
    let mut tuple = item.extract::<&PyTuple>()?.into_iter();
    let key = match tuple.next() {
        None => {
            return Err(PyValueError::new_err(
                "Expected mapping key in the first tuple element",
            ))
        }
        Some(value) => match key_ty {
            ton_abi::ParamType::Uint(size) => {
                let number = value.extract::<num_bigint::BigUint>()?;
                ton_abi::MapKeyTokenValue::Uint(ton_abi::Uint {
                    number,
                    size: *size,
                })
            }
            ton_abi::ParamType::Int(size) => {
                let number = value.extract::<num_bigint::BigInt>()?;
                ton_abi::MapKeyTokenValue::Int(ton_abi::Int {
                    number,
                    size: *size,
                })
            }
            ton_abi::ParamType::Address => {
                let Address(addr) = value.extract::<Address>()?;
                ton_abi::MapKeyTokenValue::Address(match addr {
                    ton_block::MsgAddressInt::AddrStd(addr) => ton_block::MsgAddress::AddrStd(addr),
                    ton_block::MsgAddressInt::AddrVar(addr) => ton_block::MsgAddress::AddrVar(addr),
                })
            }
            _ => return Err(PyValueError::new_err("Unsupported mapping key type")),
        },
    };

    let value = match tuple.next() {
        None => {
            return Err(PyValueError::new_err(
                "Expected mapping value in the second tuple element",
            ))
        }
        Some(value) => parse_token(value_ty, value)?,
    };

    Ok((key, value))
}

pub fn convert_tokens(py: Python, tokens: Vec<ton_abi::Token>) -> PyResult<&PyDict> {
    let result = PyDict::new(py);
    for token in tokens {
        result.set_item(&token.name, convert_token(py, token.value)?)?;
    }
    Ok(result)
}

fn convert_token(py: Python, value: ton_abi::TokenValue) -> PyResult<PyObject> {
    use pyo3::types::*;

    Ok(match value {
        ton_abi::TokenValue::Uint(ton_abi::Uint { number, .. }) => number.to_object(py),
        ton_abi::TokenValue::Int(ton_abi::Int { number, .. }) => number.to_object(py),
        ton_abi::TokenValue::VarInt(_, number) => number.to_object(py),
        ton_abi::TokenValue::VarUint(_, number) => number.to_object(py),
        ton_abi::TokenValue::Bool(value) => value.to_object(py),
        ton_abi::TokenValue::Tuple(values) => convert_tokens(py, values)?.to_object(py),
        ton_abi::TokenValue::Array(_, values) | ton_abi::TokenValue::FixedArray(_, values) => {
            let items = values
                .into_iter()
                .map(|item| convert_token(py, item))
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items).to_object(py)
        }
        ton_abi::TokenValue::Cell(cell) => Cell(cell).into_py(py),
        ton_abi::TokenValue::Map(_, _, values) => {
            let items = values
                .into_iter()
                .map(|(key, value)| convert_map_entry_token(py, key, value))
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items).to_object(py)
        }
        ton_abi::TokenValue::Address(addr) => convert_addr_token(py, addr)?,
        ton_abi::TokenValue::Bytes(bytes) | ton_abi::TokenValue::FixedBytes(bytes) => {
            PyBytes::new(py, &bytes).to_object(py)
        }
        ton_abi::TokenValue::String(string) => PyString::new(py, &string).to_object(py),
        ton_abi::TokenValue::Token(number) => number.as_u128().to_object(py),
        ton_abi::TokenValue::Time(number) => number.to_object(py),
        ton_abi::TokenValue::Expire(number) => number.to_object(py),
        ton_abi::TokenValue::PublicKey(pubkey) => match pubkey {
            Some(value) => hex::encode(value.as_bytes()).to_object(py),
            None => py.None(),
        },
        ton_abi::TokenValue::Optional(_, value) => match value {
            Some(value) => convert_token(py, *value)?,
            None => py.None(),
        },
        ton_abi::TokenValue::Ref(value) => convert_token(py, *value)?,
    })
}

fn convert_map_entry_token(
    py: Python,
    key: ton_abi::MapKeyTokenValue,
    value: ton_abi::TokenValue,
) -> PyResult<PyObject> {
    use pyo3::types::*;

    let key = match key {
        ton_abi::MapKeyTokenValue::Uint(ton_abi::Uint { number, .. }) => number.to_object(py),
        ton_abi::MapKeyTokenValue::Int(ton_abi::Int { number, .. }) => number.to_object(py),
        ton_abi::MapKeyTokenValue::Address(addr) => convert_addr_token(py, addr)?,
    };

    Ok(PyTuple::new(py, [key, convert_token(py, value)?]).to_object(py))
}

fn convert_addr_token(py: Python, addr: ton_block::MsgAddress) -> PyResult<PyObject> {
    Ok(Address(match addr {
        ton_block::MsgAddress::AddrStd(addr) => ton_block::MsgAddressInt::AddrStd(addr),
        ton_block::MsgAddress::AddrVar(addr) => ton_block::MsgAddressInt::AddrVar(addr),
        _ => return Err(PyRuntimeError::new_err("Unsupported address type")),
    })
    .into_py(py))
}

pub fn default_headers(
    time: u64,
    expiration: nt::core::models::Expiration,
    public_key: Option<ed25519_dalek::PublicKey>,
) -> (
    nt::core::models::ExpireAt,
    HashMap<String, ton_abi::TokenValue>,
) {
    let expire_at = nt::core::models::ExpireAt::new_from_millis(expiration, time);

    let mut header = HashMap::with_capacity(3);
    header.insert("time".to_string(), ton_abi::TokenValue::Time(time));
    header.insert(
        "expire".to_string(),
        ton_abi::TokenValue::Expire(expire_at.timestamp),
    );
    header.insert(
        "pubkey".to_string(),
        ton_abi::TokenValue::PublicKey(public_key),
    );

    (expire_at, header)
}
