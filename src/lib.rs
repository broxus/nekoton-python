use pyo3::prelude::*;

use self::abi::*;
use self::crypto::{Bip39Seed, KeyPair, LegacySeed, PublicKey, Seed, Signature};
use self::models::{Address, Cell, Message, StateInit};
use self::subscription::Subscription;
use self::transport::{Clock, GqlTransport, JrpcTransport, Transport};

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
    m.add_class::<Message>()?;
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
