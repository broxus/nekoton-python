use pyo3::prelude::*;

use self::abi::*;
use self::crypto::*;
use self::models::*;
use self::subscription::*;
use self::transport::*;

mod abi;
mod crypto;
mod models;
mod subscription;
mod transport;
mod util;

/// Rust bindings to the nekoton.
#[pymodule]
fn nekoton(_py: Python, m: &PyModule) -> PyResult<()> {
    // Transport
    m.add_class::<Clock>()?;
    m.add_class::<Transport>()?;
    m.add_class::<GqlTransport>()?;
    m.add_class::<JrpcTransport>()?;

    // Models
    m.add_class::<AccountState>()?;
    m.add_class::<StorageUsed>()?;
    m.add_class::<Transaction>()?;
    m.add_class::<TransactionType>()?;
    m.add_class::<TransactionStoragePhase>()?;
    m.add_class::<TransactionCreditPhase>()?;
    m.add_class::<TransactionComputePhase>()?;
    m.add_class::<TransactionActionPhase>()?;
    m.add_class::<TransactionBouncePhase>()?;
    m.add_class::<AccountStatus>()?;
    m.add_class::<AccountStatusChange>()?;
    m.add_class::<Message>()?;
    m.add_class::<MessageHeader>()?;
    m.add_class::<InternalMessageHeader>()?;
    m.add_class::<ExternalInMessageHeader>()?;
    m.add_class::<ExternalOutMessageHeader>()?;
    m.add_class::<MessageType>()?;
    m.add_class::<StateInit>()?;
    m.add_class::<Address>()?;
    m.add_class::<Cell>()?;

    // Subscription
    m.add_class::<Subscription>()?;

    // Abi
    m.add_class::<ContractAbi>()?;
    m.add_class::<FunctionAbi>()?;
    m.add_class::<EventAbi>()?;
    m.add_class::<AbiVersion>()?;
    m.add_class::<UnsignedBody>()?;
    m.add_class::<UnsignedExternalMessage>()?;
    m.add_class::<AbiUint>()?;
    m.add_class::<AbiInt>()?;
    m.add_class::<AbiVarUint>()?;
    m.add_class::<AbiVarInt>()?;
    m.add_class::<AbiBool>()?;
    m.add_class::<AbiTuple>()?;
    m.add_class::<AbiArray>()?;
    m.add_class::<AbiFixedArray>()?;
    m.add_class::<AbiCell>()?;
    m.add_class::<AbiMap>()?;
    m.add_class::<AbiAddress>()?;
    m.add_class::<AbiBytes>()?;
    m.add_class::<AbiFixedBytes>()?;
    m.add_class::<AbiString>()?;
    m.add_class::<AbiToken>()?;
    m.add_class::<AbiOptional>()?;
    m.add_class::<AbiRef>()?;

    // Crypto
    m.add_class::<PublicKey>()?;
    m.add_class::<KeyPair>()?;
    m.add_class::<Signature>()?;
    m.add_class::<Seed>()?;
    m.add_class::<LegacySeed>()?;
    m.add_class::<Bip39Seed>()?;

    Ok(())
}
