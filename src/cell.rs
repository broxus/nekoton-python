use std::str::FromStr;

use pyo3::exceptions::*;
use pyo3::prelude::*;

use crate::util::HandleError;

#[derive(Default, Clone)]
#[pyclass]
pub struct Cell(pub ton_types::Cell);

impl Cell {
    pub fn try_from_struct(value: &dyn ton_block::Serializable) -> PyResult<Self> {
        value.serialize().handle_runtime_error().map(Self)
    }
}

#[pymethods]
impl Cell {
    /// Constructs a new cell from base64 encoded BOC.
    #[new]
    fn new(value: Option<&str>, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding)?;
        encoding.decode(value.unwrap_or_default().trim()).map(Self)
    }

    /// Returns a hex encoded repr hash of the root cell.
    #[getter]
    fn repr_hash(&self) -> String {
        self.0.repr_hash().to_hex_string()
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding)?;
        encoding.encode(&self.0)
    }

    fn encode_raw(&self) -> PyResult<Vec<u8>> {
        ton_types::serialize_toc(&self.0).handle_runtime_error()
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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
enum Encoding {
    Hex,
    #[default]
    Base64,
}

impl Encoding {
    fn from_optional_param(encoding: Option<&str>) -> PyResult<Self> {
        match encoding {
            None => Ok(Self::default()),
            Some(s) => s.parse(),
        }
    }

    fn decode(&self, boc: &str) -> PyResult<ton_types::Cell> {
        let boc = boc.trim();
        if boc.is_empty() {
            return Ok(Default::default());
        }

        let bytes = match self {
            Self::Hex => hex::decode(boc).handle_value_error(),
            Self::Base64 => base64::decode(boc).handle_value_error(),
        }?;
        ton_types::deserialize_tree_of_cells(&mut bytes.as_slice()).handle_value_error()
    }

    fn encode(&self, cell: &ton_types::Cell) -> PyResult<String> {
        let cell = ton_types::serialize_toc(cell).handle_runtime_error()?;
        Ok(match self {
            Self::Hex => hex::encode(cell),
            Self::Base64 => base64::encode(cell),
        })
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
