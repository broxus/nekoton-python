use std::collections::HashMap;
use std::str::FromStr;

use once_cell::sync::OnceCell;
use pyo3::exceptions::*;
use pyo3::prelude::*;

pub type FastHashMap<K, V> = HashMap<K, V, ahash::RandomState>;
pub type FastDashMap<K, V> = dashmap::DashMap<K, V, ahash::RandomState>;

impl<T, E> HandleError for Result<T, E>
where
    E: ToString,
{
    type Output = T;

    fn handle_value_error(self) -> PyResult<Self::Output> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    fn handle_runtime_error(self) -> PyResult<Self::Output> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }
}

pub trait HandleError {
    type Output;

    fn handle_value_error(self) -> PyResult<Self::Output>;
    fn handle_runtime_error(self) -> PyResult<Self::Output>;
}

pub trait HashExt: Sized {
    fn from_bytes(bytes: &[u8], name: &str) -> PyResult<Self>;
}

impl HashExt for ton_types::UInt256 {
    fn from_bytes(bytes: &[u8], name: &str) -> PyResult<Self> {
        if bytes.len() == 32 {
            Ok(ton_types::UInt256::from_le_bytes(bytes))
        } else {
            Err(PyValueError::new_err(format!("Invalid {name}")))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Encoding {
    Hex,
    #[default]
    Base64,
}

impl Encoding {
    pub fn from_optional_param(encoding: Option<&str>, default: Encoding) -> PyResult<Self> {
        match encoding {
            None => Ok(default),
            Some(s) => s.parse(),
        }
    }

    pub fn decode_pubkey(&self, pubkey: &str) -> PyResult<ed25519_dalek::PublicKey> {
        let bytes = self.decode_bytes(pubkey)?;
        ed25519_dalek::PublicKey::from_bytes(&bytes).handle_value_error()
    }

    pub fn encode_pubkey(&self, pubkey: &ed25519_dalek::PublicKey) -> String {
        self.encode_bytes(pubkey.as_bytes())
    }

    pub fn decode_cell(&self, boc: &str) -> PyResult<ton_types::Cell> {
        let boc = boc.trim();
        if boc.is_empty() {
            return Ok(Default::default());
        }

        let bytes = self.decode_bytes(boc)?;
        ton_types::deserialize_tree_of_cells(&mut bytes.as_slice()).handle_value_error()
    }

    pub fn encode_cell(&self, cell: &ton_types::Cell) -> PyResult<String> {
        let cell = ton_types::serialize_toc(cell).handle_runtime_error()?;
        Ok(self.encode_bytes(&cell))
    }

    pub fn decode_bytes(&self, data: &str) -> PyResult<Vec<u8>> {
        use base64::engine::general_purpose::STANDARD;
        use base64::engine::Engine;

        let data = data.trim();
        match self {
            Self::Hex => hex::decode(data).handle_value_error(),
            Self::Base64 => STANDARD.decode(data).handle_value_error(),
        }
    }

    pub fn encode_bytes(&self, data: &[u8]) -> String {
        use base64::engine::general_purpose::STANDARD;
        use base64::engine::Engine;

        match self {
            Self::Hex => hex::encode(data),
            Self::Base64 => STANDARD.encode(data),
        }
    }
}

impl FromStr for Encoding {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hex" => Ok(Self::Hex),
            "base64" => Ok(Self::Base64),
            _ => Err(PyValueError::new_err("Unknown encoding")),
        }
    }
}

#[derive(Copy, Clone)]
pub struct DisplayBool(pub bool);

impl std::fmt::Display for DisplayBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(if self.0 { "True" } else { "False" })
    }
}

pub fn py_none() -> PyObject {
    static TRUE: OnceCell<PyObject> = OnceCell::new();
    TRUE.get_or_init(|| Python::with_gil(|py| py.None()))
        .clone()
}

pub fn serialize_state_init_data_key(key: u64) -> ton_types::SliceData {
    use ton_block::Serializable;

    key.serialize()
        .and_then(ton_types::SliceData::load_cell)
        .unwrap()
}

pub fn make_hasher() -> ahash::RandomState {
    ahash::RandomState::with_seed(0)
}
