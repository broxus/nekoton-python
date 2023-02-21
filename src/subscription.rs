use std::sync::Arc;

use nt::core::models;
use pyo3::prelude::*;
use tokio::sync::oneshot;

use crate::transport::{Transport, TransportHandle};
use crate::util::*;

#[pyclass]
pub struct Subscription {
    address: Address,
    transport: TransportHandle,
    state: Arc<SubscriptionState>,
    contract: tokio::sync::Mutex<nt::core::generic_contract::GenericContract>,
}

impl Subscription {
    pub async fn subscribe_impl(transport: Transport, address: Address) -> PyResult<Self> {
        let clock = transport.clock.0;
        let transport = transport.handle;

        let state = Arc::new(SubscriptionState::default());

        let handler = Arc::new(SubscriptionHandler {
            state: state.clone(),
        });

        let contract = nt::core::generic_contract::GenericContract::subscribe(
            clock,
            transport.clone().into(),
            address.0.clone(),
            handler,
            false,
        )
        .await
        .map(tokio::sync::Mutex::new)
        .handle_runtime_error()?;

        Ok(Self {
            address,
            transport,
            state,
            contract,
        })
    }
}

#[pymethods]
impl Subscription {
    #[staticmethod]
    pub fn subscribe(py: Python, transport: Transport, address: Address) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, Subscription::subscribe_impl(transport, address))
    }
}

#[derive(Default)]
struct SubscriptionState {
    brief_state: std::sync::Mutex<models::ContractState>,
    pending_messages: FastDashMap<ton_types::UInt256, MsgTx>,
}

struct SubscriptionHandler {
    state: Arc<SubscriptionState>,
}

impl nt::core::generic_contract::GenericContractSubscriptionHandler for SubscriptionHandler {
    fn on_message_sent(
        &self,
        pending_transaction: models::PendingTransaction,
        transaction: Option<models::Transaction>,
    ) {
        let pending = &self.state.pending_messages;
        if let Some((_, tx)) = pending.remove(&pending_transaction.message_hash) {
            _ = tx.send(match transaction {
                Some(tx) => ReceivedTransaction::Valid(tx),
                None => ReceivedTransaction::Invalid,
            });
        }
    }

    fn on_message_expired(&self, pending_transaction: models::PendingTransaction) {
        let pending = &self.state.pending_messages;
        if let Some((_, tx)) = pending.remove(&pending_transaction.message_hash) {
            _ = tx.send(ReceivedTransaction::Expired)
        }
    }

    fn on_state_changed(&self, new_state: models::ContractState) {
        *self.state.brief_state.lock().unwrap() = new_state;
    }

    fn on_transactions_found(&self, _: Vec<models::Transaction>, _: models::TransactionsBatchInfo) {
        // TODO
    }
}

enum ReceivedTransaction {
    Expired,
    Invalid,
    Valid(models::Transaction),
}

type MsgTx = oneshot::Sender<ReceivedTransaction>;
type MsgRx = oneshot::Receiver<ReceivedTransaction>;

#[derive(Clone)]
#[pyclass]
pub struct Address(pub ton_block::MsgAddressInt);

#[pymethods]
impl Address {
    #[staticmethod]
    fn validate(addr: &str) -> bool {
        nt::utils::validate_address(addr)
    }

    #[new]
    fn new(addr: &str) -> PyResult<Self> {
        nt::utils::repack_address(addr.trim())
            .map(Self)
            .handle_value_error()
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
