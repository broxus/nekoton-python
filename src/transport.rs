use std::collections::VecDeque;
use std::sync::{Arc, Weak};
use std::time::Duration;

use nt::core::models;
use pyo3::exceptions::{PyRuntimeError, PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyString;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, oneshot, watch, Notify};
use tokio_util::sync::{CancellationToken, DropGuard};
use ton_block::Deserializable;

use crate::abi::SignedExternalMessage;
use crate::models::{AccountState, Address, BlockchainConfig, Message, Transaction};
use crate::util::*;

#[pyclass(subclass)]
pub struct Transport(Arc<TransportState>);

struct TransportState {
    clock: Clock,
    handle: TransportHandle,
    subscriptions: Arc<tokio::sync::Mutex<SubscriptionsMap>>,
    _drop_guard: DropGuard,
}

impl TransportState {
    fn new(clock: Clock, handle: TransportHandle) -> Arc<Self> {
        let cancellation_token = CancellationToken::new();

        let shared = Arc::new(Self {
            clock,
            handle,
            subscriptions: Default::default(),
            _drop_guard: cancellation_token.clone().drop_guard(),
        });

        let weak = Arc::downgrade(&shared);
        pyo3_asyncio::tokio::get_runtime().spawn(async move {
            const GC_INTERVAL: Duration = Duration::from_secs(2);

            tokio::pin!(let cancelled = cancellation_token.cancelled(););
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(GC_INTERVAL) => {},
                    _ = &mut cancelled => return,
                }

                let Some(shared) = weak.upgrade() else {
                    return;
                };

                let mut subscriptions = shared.subscriptions.lock().await;
                subscriptions.retain(|_, subscription| subscription.strong_count() > 0);
            }
        });

        shared
    }

    async fn get_subscription(
        &self,
        address: ton_block::MsgAddressInt,
    ) -> PyResult<Arc<SharedSubscription>> {
        use std::collections::hash_map;

        log::debug!("Requesting subscription for {address}");

        let mut subscriptions = self.subscriptions.lock().await;
        let subscription = match subscriptions.entry(address.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                if let Some(subscription) = entry.get().upgrade() {
                    log::debug!("Subscription for {address} already exists");
                    subscription
                } else {
                    log::debug!("Recreating subscription for {address}");
                    let subscription = SharedSubscription::subscribe(
                        self.clock.clone(),
                        self.handle.clone(),
                        address,
                    )
                    .await
                    .handle_runtime_error()?;
                    entry.insert(Arc::downgrade(&subscription));
                    subscription
                }
            }
            hash_map::Entry::Vacant(entry) => {
                log::debug!("Creating subscription for {address}");
                let subscription =
                    SharedSubscription::subscribe(self.clock.clone(), self.handle.clone(), address)
                        .await
                        .handle_runtime_error()?;
                entry.insert(Arc::downgrade(&subscription));
                subscription
            }
        };

        Ok(subscription)
    }
}

type SubscriptionsMap = FastHashMap<ton_block::MsgAddressInt, Weak<SharedSubscription>>;

#[pymethods]
impl Transport {
    #[getter]
    pub fn clock(&self) -> Clock {
        self.0.clock.clone()
    }

    pub fn check_connection<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handle.check_local_node_connection().await
        })
    }

    pub fn send_external_message<'a>(
        &self,
        py: Python<'a>,
        message: PyRef<'a, SignedExternalMessage>,
    ) -> PyResult<&'a PyAny> {
        let expire_at = message.expire_at;
        let message = message.into_super().clone();

        let dst = {
            let ton_block::CommonMsgInfo::ExtInMsgInfo(info) = message.data.header() else {
                return Err(PyValueError::new_err("Expected external outbound message"));
            };
            info.dst.clone()
        };

        let shared = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let subscription = shared.get_subscription(dst).await?;

            // TODO: add watchdog timer to delay subscription drop

            subscription.send_message(&message, expire_at).await
        })
    }

    pub fn get_signature_id<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let capabilities = state
                .handle
                .as_ref()
                .get_capabilities(state.clock.as_ref())
                .await
                .handle_runtime_error()?;
            Ok(capabilities.signature_id())
        })
    }

    pub fn get_blockchain_config<'a>(
        &self,
        py: Python<'a>,
        force: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let force = force.unwrap_or_default();
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let config = state
                .handle
                .as_ref()
                .get_blockchain_config(state.clock.as_ref(), force)
                .await
                .handle_runtime_error()?;

            Ok(BlockchainConfig::from(config))
        })
    }

    pub fn get_account_state<'a>(&self, py: Python<'a>, address: Address) -> PyResult<&'a PyAny> {
        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let state = handle
                .as_ref()
                .get_contract_state(&address.0)
                .await
                .handle_runtime_error()?;

            Ok(match state {
                nt::transport::models::RawContractState::NotExists { .. } => None,
                nt::transport::models::RawContractState::Exists(state) => {
                    Some(AccountState(state.account))
                }
            })
        })
    }

    pub fn get_accounts_by_code_hash<'a>(
        &self,
        py: Python<'a>,
        code_hash: &[u8],
        continuation: Option<Address>,
        limit: Option<u8>,
    ) -> PyResult<&'a PyAny> {
        const DEFAULT_LIMIT: u8 = 50;

        let code_hash = ton_types::UInt256::from_bytes(code_hash, "code hash")?;

        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let addresses = handle
                .as_ref()
                .get_accounts_by_code_hash(
                    &code_hash,
                    limit.unwrap_or(DEFAULT_LIMIT),
                    &continuation.map(|Address(addr)| addr),
                )
                .await
                .handle_runtime_error()?;

            Ok(addresses.into_iter().map(Address).collect::<Vec<_>>())
        })
    }

    pub fn get_transaction<'a>(
        &self,
        py: Python<'a>,
        transaction_hash: &[u8],
    ) -> PyResult<&'a PyAny> {
        let transaction_hash =
            ton_types::UInt256::from_bytes(transaction_hash, "transaction hash")?;

        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match handle
                .as_ref()
                .get_transaction(&transaction_hash)
                .await
                .handle_runtime_error()?
            {
                None => Ok(None),
                Some(tx) => Transaction::try_from(tx).map(Some),
            }
        })
    }

    pub fn get_dst_transaction<'a>(
        &self,
        py: Python<'a>,
        message_hash: MessageOrHash<'a>,
    ) -> PyResult<&'a PyAny> {
        let message_hash = message_hash.try_into()?;

        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match handle
                .as_ref()
                .get_dst_transaction(&message_hash)
                .await
                .handle_runtime_error()?
            {
                None => Ok(None),
                Some(tx) => Transaction::try_from(tx).map(Some),
            }
        })
    }

    pub fn get_transactions<'a>(
        &self,
        py: Python<'a>,
        address: Address,
        lt: Option<u64>,
        limit: Option<u8>,
    ) -> PyResult<&'a PyAny> {
        const DEFAULT_LIMIT: u8 = 50;

        let handle = self.0.handle.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let raw_transactions = handle
                .as_ref()
                .get_transactions(
                    &address.0,
                    lt.unwrap_or(u64::MAX),
                    limit.unwrap_or(DEFAULT_LIMIT),
                )
                .await
                .handle_runtime_error()?;

            raw_transactions
                .into_iter()
                .map(Transaction::try_from)
                .collect::<PyResult<Vec<_>>>()
        })
    }

    pub fn account_states(&self, address: Address) -> AccountStatesAsyncIter {
        AccountStatesAsyncIter(Arc::new(tokio::sync::Mutex::new(
            AccountStatesAsyncIterState::Uninit {
                transport: self.0.clone(),
                address: address.0,
            },
        )))
    }

    pub fn account_transactions(&self, address: Address) -> AccountTransactionsAsyncIter {
        AccountTransactionsAsyncIter(Arc::new(tokio::sync::Mutex::new(
            AccountTransactionsAsyncIterState::Uninit {
                transport: self.0.clone(),
                address: address.0,
            },
        )))
    }

    pub fn trace_transaction<'a>(
        &self,
        py: Python<'a>,
        transaction_hash: TransactionOrHash<'a>,
        yield_root: Option<bool>,
    ) -> PyResult<TraceTransaction> {
        let yield_root = yield_root.unwrap_or_default();
        let mut queue = Default::default();
        let root_hash;
        let root = match &transaction_hash {
            TransactionOrHash::Transaction(tx) => {
                TraceTransactionState::extract_messages(&tx.0.data, &mut queue)?;
                root_hash = Some(tx.0.hash);
                yield_root.then(|| tx.into_py(py))
            }
            TransactionOrHash::Hash(bytes) => {
                root_hash = Some(ton_types::UInt256::from_bytes(bytes, "transaction hash")?);
                None
            }
        };

        Ok(TraceTransaction(Arc::new(tokio::sync::Mutex::new(
            TraceTransactionState {
                transport: self.0.clone(),
                yield_root,
                root_hash,
                root,
                queue,
            },
        ))))
    }
}

#[derive(FromPyObject)]
pub enum MessageOrHash<'a> {
    #[pyo3(transparent, annotation = "bytes")]
    Hash(&'a [u8]),
    #[pyo3(transparent, annotation = "Message")]
    Message(PyRef<'a, Message>),
}

impl TryFrom<MessageOrHash<'_>> for ton_types::UInt256 {
    type Error = PyErr;

    fn try_from(value: MessageOrHash<'_>) -> Result<Self, Self::Error> {
        match value {
            MessageOrHash::Hash(hash) => ton_types::UInt256::from_bytes(hash, "message hash"),
            MessageOrHash::Message(msg) => Ok(msg.hash),
        }
    }
}

#[derive(FromPyObject)]
pub enum TransactionOrHash<'a> {
    #[pyo3(transparent, annotation = "bytes")]
    Hash(&'a [u8]),
    #[pyo3(transparent, annotation = "Transaction")]
    Transaction(PyRef<'a, Transaction>),
}

impl TryFrom<TransactionOrHash<'_>> for ton_types::UInt256 {
    type Error = PyErr;

    fn try_from(value: TransactionOrHash<'_>) -> Result<Self, Self::Error> {
        match value {
            TransactionOrHash::Hash(hash) => {
                ton_types::UInt256::from_bytes(hash, "transaction hash")
            }
            TransactionOrHash::Transaction(tx) => Ok(tx.0.hash),
        }
    }
}

#[derive(Clone)]
#[pyclass(subclass, extends = Transport)]
pub struct GqlTransport {
    client: Arc<nekoton_transport::gql::GqlClient>,
}

impl GqlTransport {
    async fn query_items<T>(
        client: &nekoton_transport::gql::GqlClient,
        query: String,
    ) -> PyResult<Vec<T::Item>>
    where
        T: GqlBocResponse,
    {
        use nt::external::{GqlConnection, GqlRequest};

        #[derive(Serialize)]
        pub struct QueryBody {
            pub query: String,
        }
        let data = serde_json::to_string(&QueryBody { query }).unwrap();

        let response = client
            .post(GqlRequest {
                data,
                long_query: false,
            })
            .await
            .handle_runtime_error()?;

        T::extract(&response)
    }

    fn build_query(
        py: Python<'_>,
        field: &str,
        with_id: bool,
        filter: GqlExprArg,
        order_by: Option<GqlExprArg>,
        limit: Option<usize>,
    ) -> PyResult<String> {
        use std::fmt::Write;

        let mut query = format!("query{{{field}(filter:{{");
        filter.write(py, &mut query)?;

        let mut closing = '}';
        if let Some(order_by) = order_by {
            write!(&mut query, "{closing},orderBy:[").unwrap();
            order_by.write(py, &mut query)?;
            closing = ']';
        }

        match limit {
            Some(limit) => write!(&mut query, "{closing},limit:{limit}){{boc"),
            None => write!(&mut query, "{closing}){{boc"),
        }
        .unwrap();

        if with_id {
            write!(&mut query, ",id}}}}")
        } else {
            write!(&mut query, "}}}}")
        }
        .unwrap();

        Ok(query)
    }
}

trait GqlBocResponse {
    type Item;

    fn extract(response: &str) -> PyResult<Vec<Self::Item>>;
}

#[pymethods]
impl GqlTransport {
    #[new]
    fn new(
        endpoints: Vec<String>,
        clock: Option<Clock>,
        local: Option<bool>,
    ) -> PyResult<PyClassInitializer<Self>> {
        use nekoton_transport::gql::*;

        let client = GqlClient::new(GqlNetworkSettings {
            endpoints,
            local: local.unwrap_or_default(),
            ..Default::default()
        })
        .handle_value_error()?;

        let gql = GqlTransport {
            client: client.clone(),
        };

        let transport = Arc::new(nt::transport::gql::GqlTransport::new(client));
        let handle = TransportHandle::GraphQl(transport);
        let clock = clock.unwrap_or_default();

        Ok(
            PyClassInitializer::from(Transport(TransportState::new(clock, handle)))
                .add_subclass(gql),
        )
    }

    fn query_transactions<'a>(
        &self,
        py: Python<'a>,
        filter: GqlExprArg,
        order_by: Option<GqlExprArg>,
        limit: Option<usize>,
    ) -> PyResult<&'a PyAny> {
        let query = Self::build_query(py, "transactions", false, filter, order_by, limit)?;
        log::debug!("Transactions query: {query}");

        struct TransactionsResponse;

        impl GqlBocResponse for TransactionsResponse {
            type Item = Transaction;

            fn extract(response: &str) -> PyResult<Vec<Self::Item>> {
                #[derive(Deserialize)]
                struct Response<'a> {
                    #[serde(default)]
                    errors: Option<serde_json::Value>,
                    #[serde(default, borrow = "'a")]
                    data: Option<Transactions<'a>>,
                }

                #[derive(Deserialize)]
                struct Transactions<'a> {
                    #[serde(borrow = "'a")]
                    transactions: Option<Vec<Item<'a>>>,
                }

                #[derive(Deserialize)]
                struct Item<'a> {
                    #[serde(borrow)]
                    boc: &'a str,
                }

                let Response { errors, data } =
                    serde_json::from_str(response).handle_runtime_error()?;
                if let Some(errors) = errors {
                    return Err(PyRuntimeError::new_err(
                        serde_json::to_string_pretty(&errors).unwrap_or_default(),
                    ));
                }

                let Some(Transactions {
                    transactions: Some(transactions),
                }) = data
                else {
                    return Err(PyRuntimeError::new_err("Invalid response"));
                };

                transactions
                    .into_iter()
                    .map(|Item { boc }| {
                        let tx_cell = Encoding::Base64.decode_cell(boc)?;
                        Transaction::try_from(tx_cell)
                    })
                    .collect()
            }
        }

        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Self::query_items::<TransactionsResponse>(&client, query).await
        })
    }

    fn query_messages<'a>(
        &self,
        py: Python<'a>,
        filter: GqlExprArg,
        order_by: Option<GqlExprArg>,
        limit: Option<usize>,
    ) -> PyResult<&'a PyAny> {
        let query = Self::build_query(py, "messages", false, filter, order_by, limit)?;
        log::debug!("Messages query: {query}");

        struct MessagesResponse;

        impl GqlBocResponse for MessagesResponse {
            type Item = Message;

            fn extract(response: &str) -> PyResult<Vec<Self::Item>> {
                #[derive(Deserialize)]
                struct Response<'a> {
                    #[serde(default)]
                    errors: Option<serde_json::Value>,
                    #[serde(default, borrow = "'a")]
                    data: Option<Messages<'a>>,
                }

                #[derive(Deserialize)]
                struct Messages<'a> {
                    #[serde(borrow = "'a")]
                    messages: Option<Vec<Item<'a>>>,
                }

                #[derive(Deserialize)]
                struct Item<'a> {
                    #[serde(borrow)]
                    boc: &'a str,
                }

                let Response { errors, data } =
                    serde_json::from_str(response).handle_runtime_error()?;
                if let Some(errors) = errors {
                    return Err(PyRuntimeError::new_err(
                        serde_json::to_string_pretty(&errors).unwrap_or_default(),
                    ));
                }

                let Some(Messages {
                    messages: Some(messages),
                }) = data
                else {
                    return Err(PyRuntimeError::new_err("Invalid response"));
                };

                messages
                    .into_iter()
                    .map(|Item { boc }| {
                        let msg_cell = Encoding::Base64.decode_cell(boc)?;
                        Message::try_from(msg_cell)
                    })
                    .collect()
            }
        }

        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Self::query_items::<MessagesResponse>(&client, query).await
        })
    }

    fn query_accounts<'a>(
        &self,
        py: Python<'a>,
        filter: GqlExprArg,
        order_by: Option<GqlExprArg>,
        limit: Option<usize>,
    ) -> PyResult<&'a PyAny> {
        let query = Self::build_query(py, "accounts", true, filter, order_by, limit)?;
        log::debug!("Accounts query: {query}");

        struct AccountsResponse;

        impl GqlBocResponse for AccountsResponse {
            type Item = (Address, Option<AccountState>);

            fn extract(response: &str) -> PyResult<Vec<Self::Item>> {
                #[derive(Deserialize)]
                struct Response<'a> {
                    #[serde(default)]
                    errors: Option<serde_json::Value>,
                    #[serde(default, borrow = "'a")]
                    data: Option<Accounts<'a>>,
                }

                #[derive(Deserialize)]
                struct Accounts<'a> {
                    #[serde(borrow = "'a")]
                    accounts: Option<Vec<Item<'a>>>,
                }

                #[derive(Deserialize)]
                struct Item<'a> {
                    #[serde(borrow)]
                    boc: Option<&'a str>,
                    #[serde(borrow)]
                    id: &'a str,
                }

                let Response { errors, data } =
                    serde_json::from_str(response).handle_runtime_error()?;
                if let Some(errors) = errors {
                    return Err(PyRuntimeError::new_err(
                        serde_json::to_string_pretty(&errors).unwrap_or_default(),
                    ));
                }

                let Some(Accounts {
                    accounts: Some(accounts),
                }) = data
                else {
                    return Err(PyRuntimeError::new_err("Invalid response"));
                };

                accounts
                    .into_iter()
                    .map(|Item { id, boc }| {
                        let id = Address::new(id)?;
                        let state = match boc {
                            Some(boc) => {
                                let cell = Encoding::Base64.decode_cell(boc)?;
                                match ton_block::Account::construct_from_cell(cell)
                                    .handle_runtime_error()?
                                {
                                    ton_block::Account::Account(state) => Some(AccountState(state)),
                                    ton_block::Account::AccountNone => None,
                                }
                            }
                            None => None,
                        };
                        Ok((id, state))
                    })
                    .collect()
            }
        }

        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Self::query_items::<AccountsResponse>(&client, query).await
        })
    }
}

#[derive(FromPyObject)]
enum GqlExprArg<'a> {
    #[pyo3(transparent, annotation = "str")]
    String(&'a str),
    #[pyo3(transparent, annotation = "GqlExprPart")]
    SingleExpr(GqlExprPart),
    #[pyo3(transparent, annotation = "List[GqlExprPart]")]
    MultipleExpr(Vec<GqlExprPart>),
}

impl GqlExprArg<'_> {
    fn write(&self, py: Python<'_>, target: &mut String) -> PyResult<()> {
        use std::fmt::Write;

        match self {
            Self::String(str) => {
                target.write_str(str).unwrap();
            }
            Self::SingleExpr(expr) => {
                target.write_str(expr.try_as_str(py)?).unwrap();
            }
            Self::MultipleExpr(exprs) => {
                let mut exprs = exprs.iter();
                if let Some(first) = exprs.next() {
                    target.write_str(first.try_as_str(py)?).unwrap();
                    for expr in exprs {
                        write!(target, ",{}", expr.try_as_str(py)?).unwrap();
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct GqlExprPart(Py<PyString>);

impl GqlExprPart {
    fn try_as_str<'a>(&'a self, py: Python<'a>) -> PyResult<&'a str> {
        self.0.as_ref(py).to_str()
    }
}

#[pymethods]
impl GqlExprPart {
    #[new]
    fn new(value: Py<PyString>) -> Self {
        Self(value)
    }

    fn __str__(&self) -> Py<PyString> {
        self.0.clone()
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("GqlExprPart(\"{}\")", self.try_as_str(py)?))
    }
}

#[derive(Copy, Clone)]
#[pyclass(subclass, extends = Transport)]
pub struct JrpcTransport;

#[pymethods]
impl JrpcTransport {
    #[new]
    fn new(endpoint: &str, clock: Option<Clock>) -> PyResult<PyClassInitializer<Self>> {
        use nekoton_transport::jrpc::JrpcClient;

        let client = JrpcClient::new(endpoint).handle_value_error()?;

        let transport = Arc::new(nt::transport::jrpc::JrpcTransport::new(client));
        let handle = TransportHandle::Jrpc(transport);
        let clock = clock.unwrap_or_default();

        Ok(
            PyClassInitializer::from(Transport(TransportState::new(clock, handle)))
                .add_subclass(Self),
        )
    }
}

#[derive(Copy, Clone)]
#[pyclass(subclass, extends = Transport)]
pub struct ProtoTransport;

#[pymethods]
impl ProtoTransport {
    #[new]
    fn new(endpoint: &str, clock: Option<Clock>) -> PyResult<PyClassInitializer<Self>> {
        use nekoton_transport::proto::ProtoClient;

        let client = ProtoClient::new(endpoint).handle_value_error()?;

        let transport = Arc::new(nt::transport::proto::ProtoTransport::new(client));
        let handle = TransportHandle::Proto(transport);
        let clock = clock.unwrap_or_default();

        Ok(
            PyClassInitializer::from(Transport(TransportState::new(clock, handle)))
                .add_subclass(Self),
        )
    }
}

#[pyclass]
pub struct AccountStatesAsyncIter(Arc<tokio::sync::Mutex<AccountStatesAsyncIterState>>);

enum AccountStatesAsyncIterState {
    Uninit {
        transport: Arc<TransportState>,
        address: ton_block::MsgAddressInt,
    },
    Active {
        watch: watch::Receiver<PyObject>,
        initial: bool,
        subscription: Arc<SharedSubscription>,
    },
    Closed,
}

#[pymethods]
impl AccountStatesAsyncIter {
    fn close<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            if let AccountStatesAsyncIterState::Active { subscription, .. } = &*state {
                log::debug!(
                    "Closed account states iterator for {}",
                    subscription.address
                );
            }
            *state = AccountStatesAsyncIterState::Closed;
            Ok(())
        })
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = slf.0.clone();
        let slf = slf.into_py(py);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            match &*state {
                AccountStatesAsyncIterState::Active { .. } => Ok(slf),
                AccountStatesAsyncIterState::Closed => Err(PyRuntimeError::new_err(
                    "Entering closed states subscription",
                )),
                AccountStatesAsyncIterState::Uninit { transport, address } => {
                    let subscription = transport.get_subscription(address.clone()).await?;
                    let watch = subscription.state.account_state.subscribe();
                    log::debug!("Created account states iterator for {address}");
                    *state = AccountStatesAsyncIterState::Active {
                        watch,
                        initial: true,
                        subscription,
                    };
                    Ok(slf)
                }
            }
        })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &'a PyAny,
        _exc_value: &'a PyAny,
        _traceback: &'a PyAny,
    ) -> PyResult<&'a PyAny> {
        self.close(py)
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __anext__<'a>(&'a mut self, py: Python<'a>) -> PyResult<Option<&'a PyAny>> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            match &*state {
                AccountStatesAsyncIterState::Closed => {
                    return Err(PyStopAsyncIteration::new_err(()))
                }
                AccountStatesAsyncIterState::Uninit { transport, address } => {
                    let subscription = transport.get_subscription(address.clone()).await?;
                    let watch = subscription.state.account_state.subscribe();
                    log::debug!("Created account states iterator for {address}");
                    *state = AccountStatesAsyncIterState::Active {
                        watch,
                        initial: true,
                        subscription,
                    };
                }
                _ => {}
            };

            match &mut *state {
                AccountStatesAsyncIterState::Active {
                    watch,
                    initial,
                    subscription,
                } => {
                    if std::mem::take(initial) {
                        return Ok(watch.borrow_and_update().clone());
                    }

                    if watch.changed().await.is_err() {
                        log::debug!(
                            "Closed account states iterator for {}",
                            subscription.address
                        );
                        *state = AccountStatesAsyncIterState::Closed;
                        return Err(PyStopAsyncIteration::new_err(()));
                    }

                    Ok(watch.borrow_and_update().clone())
                }
                _ => unreachable!(),
            }
        })
        .map(Some)
    }
}

#[pyclass]
pub struct AccountTransactionsAsyncIter(Arc<tokio::sync::Mutex<AccountTransactionsAsyncIterState>>);

enum AccountTransactionsAsyncIterState {
    Uninit {
        transport: Arc<TransportState>,
        address: ton_block::MsgAddressInt,
    },
    Active {
        transactions: broadcast::Receiver<PyObject>,
        subscription: Arc<SharedSubscription>,
    },
    Closed,
}

#[pymethods]
impl AccountTransactionsAsyncIter {
    fn close<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            if let AccountTransactionsAsyncIterState::Active { subscription, .. } = &*state {
                log::debug!("Closed transactions iterator for {}", subscription.address);
            }
            *state = AccountTransactionsAsyncIterState::Closed;
            Ok(())
        })
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = slf.0.clone();
        let slf = slf.into_py(py);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            match &*state {
                AccountTransactionsAsyncIterState::Active { .. } => Ok(slf),
                AccountTransactionsAsyncIterState::Closed => Err(PyRuntimeError::new_err(
                    "Entering closed transactions subscription",
                )),
                AccountTransactionsAsyncIterState::Uninit { transport, address } => {
                    let subscription = transport.get_subscription(address.clone()).await?;
                    let transactions = subscription.state.transactions.subscribe();
                    log::debug!("Created transactions iterator for {address}");
                    *state = AccountTransactionsAsyncIterState::Active {
                        transactions,
                        subscription,
                    };
                    Ok(slf)
                }
            }
        })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &'a PyAny,
        _exc_value: &'a PyAny,
        _traceback: &'a PyAny,
    ) -> PyResult<&'a PyAny> {
        self.close(py)
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __anext__<'a>(&'a mut self, py: Python<'a>) -> PyResult<Option<&'a PyAny>> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            match &*state {
                AccountTransactionsAsyncIterState::Closed => {
                    return Err(PyStopAsyncIteration::new_err(()))
                }
                AccountTransactionsAsyncIterState::Uninit { transport, address } => {
                    let subscription = transport.get_subscription(address.clone()).await?;
                    let transactions = subscription.state.transactions.subscribe();
                    log::debug!("Created transactions iterator for {address}");
                    *state = AccountTransactionsAsyncIterState::Active {
                        transactions,
                        subscription,
                    };
                }
                _ => {}
            };

            match &mut *state {
                AccountTransactionsAsyncIterState::Active {
                    transactions,
                    subscription,
                } => match transactions.recv().await {
                    Ok(batch) => Ok(batch),
                    Err(_) => {
                        log::debug!("Closed transactions iterator for {}", subscription.address);
                        *state = AccountTransactionsAsyncIterState::Closed;
                        Err(PyStopAsyncIteration::new_err(()))
                    }
                },
                _ => unreachable!(),
            }
        })
        .map(Some)
    }
}

#[pyclass]
pub struct TraceTransaction(Arc<tokio::sync::Mutex<TraceTransactionState>>);

struct TraceTransactionState {
    transport: Arc<TransportState>,
    yield_root: bool,
    root_hash: Option<ton_types::UInt256>,
    root: Option<PyObject>,
    queue: VecDeque<ton_types::UInt256>,
}

impl TraceTransactionState {
    fn extract_messages(
        tx: &ton_block::Transaction,
        queue: &mut VecDeque<ton_types::UInt256>,
    ) -> PyResult<()> {
        let mut hashes = Vec::new();
        tx.out_msgs
            .iterate_slices(|slice| {
                let Some(msg_cell) = slice.reference_opt(0) else {
                    return Ok(true);
                };

                let message = ton_block::Message::construct_from_cell(msg_cell.clone())?;
                if message.is_internal() {
                    hashes.push(msg_cell.repr_hash());
                }

                Ok(true)
            })
            .handle_runtime_error()?;

        queue.extend(hashes);
        Ok(())
    }

    async fn next(&mut self) -> PyResult<Option<nt::transport::models::RawTransaction>> {
        const MIN_INTERVAL_MS: u64 = 500;
        const MAX_INTERVAL_MS: u64 = 3000;
        const FACTOR: u64 = 2;

        let transport = self.transport.handle.as_ref();

        if let Some(root_hash) = &self.root_hash {
            let Some(tx) = transport
                .get_transaction(root_hash)
                .await
                .handle_runtime_error()?
            else {
                return Err(PyRuntimeError::new_err("Root transaction not found"));
            };

            Self::extract_messages(&tx.data, &mut self.queue)?;

            self.root_hash = None;
            if std::mem::take(&mut self.yield_root) {
                return Ok(Some(tx));
            }
        }

        let Some(message_hash) = self.queue.front() else {
            return Ok(None);
        };

        let mut interval_ms = MIN_INTERVAL_MS;
        let tx = loop {
            if let Ok(Some(tx)) = transport.get_dst_transaction(message_hash).await {
                break tx;
            }

            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
            interval_ms = std::cmp::min(interval_ms * FACTOR, MAX_INTERVAL_MS);
        };

        Self::extract_messages(&tx.data, &mut self.queue)?;
        self.queue.pop_front();

        Ok(Some(tx))
    }
}

#[pymethods]
impl TraceTransaction {
    fn close<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            state.yield_root = false;
            state.root_hash = None;
            state.root = None;
            state.queue.clear();
            Ok(())
        })
    }

    fn wait<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            state.yield_root = false;
            state.root = None;
            while state.next().await?.is_some() {}
            Ok(())
        })
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<&'a PyAny> {
        let slf = slf.into_py(py);
        pyo3_asyncio::tokio::future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &'a PyAny,
        _exc_value: &'a PyAny,
        _traceback: &'a PyAny,
    ) -> PyResult<&'a PyAny> {
        self.close(py)
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __anext__<'a>(&'a mut self, py: Python<'a>) -> PyResult<Option<&'a PyAny>> {
        let state = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut state = state.lock().await;
            if let Some(root) = state.root.take() {
                state.yield_root = false;
                return Ok(root);
            }

            match state.next().await? {
                Some(tx) => {
                    let tx = Transaction::try_from(tx)?;
                    Python::with_gil(|py| Ok(tx.into_py(py)))
                }
                None => Err(PyStopAsyncIteration::new_err(())),
            }
        })
        .map(Some)
    }
}

#[pyclass(get_all)]
pub struct TransactionsBatchInfo {
    min_lt: u64,
    max_lt: u64,
}

#[pymethods]
impl TransactionsBatchInfo {
    fn __repr__(&self) -> String {
        format!(
            "<TransactionsBatchInfo min_lt={}, max_lt={}>",
            self.min_lt, self.max_lt
        )
    }
}

#[derive(Default, Clone)]
#[pyclass]
pub struct Clock(pub Arc<nt::utils::ClockWithOffset>);

#[pymethods]
impl Clock {
    /// Creates a new clock with the specified offset in milliseconds.
    #[new]
    pub fn new(offset: Option<i64>) -> Self {
        Self(Arc::new(nt::utils::ClockWithOffset::new(
            offset.unwrap_or_default(),
        )))
    }

    #[getter]
    pub fn now_sec(&self) -> u64 {
        nt::utils::Clock::now_sec_u64(self.0.as_ref())
    }

    #[getter]
    pub fn now_ms(&self) -> u64 {
        nt::utils::Clock::now_ms_u64(self.0.as_ref())
    }

    #[getter]
    pub fn get_offset(&self) -> i64 {
        self.0.offset_ms()
    }

    #[setter]
    pub fn set_offset(&self, offset: i64) {
        self.0.update_offset(offset)
    }

    fn __repr__(&self) -> String {
        format!("Clock(offset={})", self.get_offset())
    }
}

impl<'a> AsRef<dyn nt::utils::Clock + 'a> for Clock {
    fn as_ref(&self) -> &(dyn nt::utils::Clock + 'a) {
        self.0.as_ref()
    }
}

#[derive(Clone)]
pub enum TransportHandle {
    GraphQl(Arc<nt::transport::gql::GqlTransport>),
    Jrpc(Arc<nt::transport::jrpc::JrpcTransport>),
    Proto(Arc<nt::transport::proto::ProtoTransport>),
}

impl<'a> AsRef<dyn nt::transport::Transport + 'a> for TransportHandle {
    fn as_ref(&self) -> &(dyn nt::transport::Transport + 'a) {
        match self {
            Self::GraphQl(transport) => transport.as_ref(),
            Self::Jrpc(transport) => transport.as_ref(),
            Self::Proto(transport) => transport.as_ref(),
        }
    }
}

impl From<TransportHandle> for Arc<dyn nt::transport::Transport> {
    fn from(handle: TransportHandle) -> Self {
        match handle {
            TransportHandle::GraphQl(transport) => transport,
            TransportHandle::Jrpc(transport) => transport,
            TransportHandle::Proto(transport) => transport,
        }
    }
}

impl TransportHandle {
    pub async fn check_connection(&self) -> PyResult<()> {
        let transport = self.as_ref();
        if transport.info().has_key_blocks {
            self.check_default_connection().await
        } else {
            self.check_local_node_connection().await
        }
    }

    async fn check_default_connection(&self) -> PyResult<()> {
        self.as_ref()
            .get_contract_state(
                &ton_block::MsgAddressInt::with_standart(None, -1, ton_types::UInt256::ZERO.into())
                    .unwrap(),
            )
            .await
            .handle_runtime_error()?;
        Ok(())
    }

    async fn check_local_node_connection(&self) -> PyResult<()> {
        static GIVER_CODE_HASH: ton_types::UInt256 = ton_types::UInt256::with_array([
            0x4e, 0x92, 0x71, 0x6d, 0xe6, 0x1d, 0x45, 0x6e, 0x58, 0xf1, 0x6e, 0x4e, 0x86, 0x7e,
            0x3e, 0x93, 0xa7, 0x54, 0x83, 0x21, 0xea, 0xce, 0x86, 0x30, 0x1b, 0x51, 0xc8, 0xb8,
            0x0c, 0xa6, 0x23, 0x9b,
        ]);

        self.as_ref()
            .get_accounts_by_code_hash(&GIVER_CODE_HASH, 1, &None)
            .await
            .handle_runtime_error()?;
        Ok(())
    }
}

struct SharedSubscription {
    address: ton_block::MsgAddressInt,
    state: SubscriptionState,
    skip_iteration_signal: Arc<Notify>,
    subscription: tokio::sync::Mutex<nt::core::ContractSubscription>,
}

impl SharedSubscription {
    async fn subscribe(
        clock: Clock,
        transport: TransportHandle,
        address: ton_block::MsgAddressInt,
    ) -> PyResult<Arc<Self>> {
        const TX_CAPACITY: usize = 10;

        let (account_state, _) = watch::channel(py_none());
        let (transactions_tx, _) = broadcast::channel(TX_CAPACITY);

        let state = SubscriptionState {
            account_state,
            transactions: transactions_tx,
            pending_messages: Default::default(),
        };

        let subscription = tokio::sync::Mutex::new(
            nt::core::ContractSubscription::subscribe(
                clock.0,
                transport.into(),
                address.clone(),
                &mut |account_state| state.on_state_changed(account_state.clone()),
                None,
            )
            .await
            .handle_runtime_error()?,
        );

        let shared = Arc::new(SharedSubscription {
            address,
            state,
            skip_iteration_signal: Arc::new(Default::default()),
            subscription,
        });

        tokio::spawn(subscription_loop(shared.clone()));

        Ok(shared)
    }

    async fn send_message(
        &self,
        message: &Message,
        expire_at: u32,
    ) -> PyResult<Option<Transaction>> {
        use dashmap::mapref::entry;

        let (tx, rx) = oneshot::channel();
        match self.state.pending_messages.entry(message.hash) {
            entry::Entry::Occupied(_) => return Err(PyRuntimeError::new_err("Duplicate message")),
            entry::Entry::Vacant(entry) => {
                entry.insert(tx);
            }
        }

        let pending_message = {
            let mut subscription = self.subscription.lock().await;
            subscription.send(&message.data, expire_at).await
        };

        match pending_message {
            Ok(tx) => {
                if tx.message_hash != message.hash {
                    // TODO: panic instead?
                    self.state.pending_messages.remove(&message.hash);
                    return Err(PyRuntimeError::new_err("Pending message mismatch"));
                }
            }
            Err(e) => {
                self.state.pending_messages.remove(&message.hash);
                return Err(e).handle_runtime_error();
            }
        }

        let result = rx.await.handle_runtime_error();
        self.state.pending_messages.remove(&message.hash);

        match result.handle_runtime_error()? {
            ReceivedTransaction::Expired => Ok(None),
            ReceivedTransaction::Invalid => {
                // TODO: panic instead?
                Err(PyRuntimeError::new_err("Failed to parse transaction"))
            }
            ReceivedTransaction::Valid(tx) => Ok(Some(tx)),
        }
    }
}

async fn subscription_loop(shared: Arc<SharedSubscription>) {
    fn split_shared(
        shared: Arc<SharedSubscription>,
    ) -> (
        ton_block::MsgAddressInt,
        Arc<Notify>,
        Weak<SharedSubscription>,
    ) {
        (
            shared.address.clone(),
            shared.skip_iteration_signal.clone(),
            Arc::downgrade(&shared),
        )
    }

    const INTERVAL: Duration = Duration::from_secs(5);
    const SHORT_INTERVAL: Duration = Duration::from_secs(1);

    let (address, skip_iteration_signal, shared) = split_shared(shared);

    let mut polling_method = models::PollingMethod::Manual;
    loop {
        let interval = match polling_method {
            models::PollingMethod::Manual => INTERVAL,
            models::PollingMethod::Reliable => SHORT_INTERVAL,
        };

        let signal = skip_iteration_signal.notified();
        tokio::select! {
            _ = signal => {},
            _ = tokio::time::sleep(interval) => {}
        }

        let Some(shared) = shared.upgrade() else {
            log::debug!("Stopped subscription for {address}");
            return;
        };

        // TODO: add support for block traversal

        let mut subscription = shared.subscription.lock().await;
        let res = subscription
            .refresh(
                &mut |state| shared.state.on_state_changed(state.clone()),
                &mut |transactions, _| shared.state.on_transactions_found(transactions),
                &mut |pending_transaction, transaction| {
                    shared
                        .state
                        .on_message_sent(pending_transaction, transaction)
                },
                &mut |pending_transaction| shared.state.on_message_expired(pending_transaction),
            )
            .await;

        if let Err(e) = res {
            log::error!("Subscription loop error for {address}: {e:?}");
        }

        polling_method = subscription.polling_method();
    }
}

struct SubscriptionState {
    account_state: watch::Sender<PyObject>,
    transactions: broadcast::Sender<PyObject>,
    pending_messages: FastDashMap<ton_types::UInt256, ResultTx>,
}

impl SubscriptionState {
    fn on_state_changed(&self, new_state: nt::transport::models::RawContractState) {
        let value = Python::with_gil(|py| match new_state.into_account() {
            ton_block::Account::AccountNone => py.None(),
            ton_block::Account::Account(stuff) => AccountState(stuff).into_py(py),
        });
        self.account_state.send_replace(value);
    }

    fn on_transactions_found(&self, mut transactions: Vec<nt::transport::models::RawTransaction>) {
        // Arrange transactions in ascending order
        transactions.reverse();

        let transactions = transactions
            .into_iter()
            .filter_map(|tx| Transaction::try_from(tx).ok())
            .collect::<Vec<_>>();

        let batch_info = match (transactions.first(), transactions.last()) {
            (Some(first), Some(last)) => TransactionsBatchInfo {
                min_lt: first.lt(),
                max_lt: last.lt(),
            },
            _ => return,
        };

        let value = Python::with_gil(|py| (transactions, batch_info).into_py(py));
        self.transactions.send(value).ok();
    }

    fn on_message_sent(
        &self,
        pending_transaction: models::PendingTransaction,
        transaction: nt::transport::models::RawTransaction,
    ) {
        if let Some((_, tx)) = self
            .pending_messages
            .remove(&pending_transaction.message_hash)
        {
            _ = tx.send(match Transaction::try_from(transaction) {
                Ok(transaction) => ReceivedTransaction::Valid(transaction),
                Err(_) => ReceivedTransaction::Invalid,
            });
        }
    }

    fn on_message_expired(&self, pending_transaction: models::PendingTransaction) {
        if let Some((_, tx)) = self
            .pending_messages
            .remove(&pending_transaction.message_hash)
        {
            _ = tx.send(ReceivedTransaction::Expired);
        }
    }
}

enum ReceivedTransaction {
    Expired,
    Invalid,
    Valid(Transaction),
}

type ResultTx = oneshot::Sender<ReceivedTransaction>;
