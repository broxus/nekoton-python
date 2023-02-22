use std::sync::Arc;

use nt::core::models;
use pyo3::prelude::*;
use tokio::sync::oneshot;

use crate::models::Address;
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
