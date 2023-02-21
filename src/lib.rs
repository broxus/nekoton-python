use pyo3::prelude::*;

use self::abi::*;
use self::cell::Cell;
use self::crypto::{Bip39Seed, KeyPair, LegacySeed, PublicKey, Seed, Signature};
use self::state_init::StateInit;
use self::subscription::{Address, Subscription};
use self::transport::{Clock, GqlTransport, JrpcTransport, Transport};
use self::util::HandleError;

mod abi;
mod cell;
mod crypto;
mod state_init;
mod subscription;
mod transport;
mod util;

/// Rust bindings to the nekoton.
#[pymodule]
fn nekoton(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cell>()?;
    m.add_class::<StateInit>()?;
    m.add_class::<Clock>()?;
    m.add_class::<Transport>()?;
    m.add_class::<GqlTransport>()?;
    m.add_class::<JrpcTransport>()?;
    m.add_class::<Address>()?;
    m.add_class::<Subscription>()?;

    m.add_class::<ContractAbi>()?;
    m.add_class::<FunctionAbi>()?;
    m.add_class::<EventAbi>()?;

    m.add_class::<AbiVersion>()?;

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

    m.add_class::<PublicKey>()?;
    m.add_class::<KeyPair>()?;
    m.add_class::<Signature>()?;
    m.add_class::<Seed>()?;
    m.add_class::<LegacySeed>()?;
    m.add_class::<Bip39Seed>()?;

    m.add_function(wrap_pyfunction!(check_address, m)?)?;
    m.add_function(wrap_pyfunction!(repack_address, m)?)?;
    m.add_function(wrap_pyfunction!(set_code_salt, m)?)?;
    m.add_function(wrap_pyfunction!(get_code_salt, m)?)?;
    Ok(())
}

/// Checks the validity of the provided address.
#[pyfunction]
fn check_address(address: &str) -> bool {
    nt::utils::validate_address(address)
}

/// Repacks address to the raw format.
#[pyfunction]
fn repack_address(address: &str) -> PyResult<String> {
    nt::utils::repack_address(address)
        .map(|x| x.to_string())
        .handle_value_error()
}

/// Adds the specified salt to the code returning a new cell.
#[pyfunction]
fn set_code_salt(code: &Cell, salt: &Cell) -> PyResult<Cell> {
    nt::abi::set_code_salt(code.0.clone(), salt.0.clone())
        .handle_runtime_error()
        .map(Cell)
}

/// Tries to extract a salt from the code cell.
#[pyfunction]
fn get_code_salt(code: &Cell) -> PyResult<Option<Cell>> {
    let salt = nt::abi::get_code_salt(code.0.clone()).handle_runtime_error()?;
    Ok(salt.map(Cell))
}
