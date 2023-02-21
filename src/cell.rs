use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::abi::{convert_tokens, parse_tokens, AbiParam, AbiVersion};
use crate::util::{Encoding, HandleError};

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

    /// Constructs a new cell from base64 encoded BOC.
    #[new]
    fn new(value: Option<&str>, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding
            .decode_cell(value.unwrap_or_default().trim())
            .map(Self)
    }

    /// Returns a hex encoded repr hash of the root cell.
    #[getter]
    fn repr_hash(&self) -> String {
        self.0.repr_hash().to_hex_string()
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding.encode_cell(&self.0)
    }

    fn encode_raw(&self) -> PyResult<Vec<u8>> {
        ton_types::serialize_toc(&self.0).handle_runtime_error()
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
