use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use ton_block::{Deserializable, Serializable};

use crate::abi::{convert_tokens, parse_tokens, AbiParam, AbiVersion};
use crate::util::{Encoding, HandleError};

#[pyclass]
pub struct Message {
    pub data: ton_block::Message,
    pub hash: ton_types::UInt256,
}

#[pymethods]
impl Message {
    #[staticmethod]
    fn from_bytes(mut bytes: &[u8]) -> PyResult<Self> {
        let cell = ton_types::deserialize_tree_of_cells(&mut bytes).handle_runtime_error()?;
        let hash = cell.repr_hash();
        let data = ton_block::Message::construct_from_cell(cell).handle_value_error()?;
        Ok(Self { data, hash })
    }

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let bytes = encoding.decode_bytes(value)?;
        Self::from_bytes(&bytes)
    }

    #[getter]
    fn hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.hash.as_slice())
    }

    #[getter]
    fn body(&self) -> Option<Cell> {
        self.data
            .body()
            .map(ton_types::SliceData::into_cell)
            .map(Cell)
    }

    #[getter]
    fn state_init(&self) -> Option<StateInit> {
        self.data.state_init().cloned().map(StateInit)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.data.serialize().handle_runtime_error()?;
        encoding.encode_cell(&cell)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.data.serialize().handle_runtime_error()?;
        let bytes = ton_types::serialize_toc(&cell).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        self.data.serialize().handle_runtime_error().map(Cell)
    }
}

#[derive(Clone)]
#[pyclass]
pub struct StateInit(pub ton_block::StateInit);

impl StateInit {
    fn expect_code(&self) -> PyResult<ton_types::Cell> {
        match &self.0.code {
            Some(code) => Ok(code.clone()),
            None => Err(PyRuntimeError::new_err("StateInit has no code")),
        }
    }
}

#[pymethods]
impl StateInit {
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        ton_block::StateInit::construct_from_bytes(bytes)
            .handle_value_error()
            .map(Self)
    }

    #[new]
    fn new(code: Option<Cell>, data: Option<Cell>) -> Self {
        Self(ton_block::StateInit {
            code: code.map(|Cell(code)| code),
            data: data.map(|Cell(cell)| cell),
            ..Default::default()
        })
    }

    #[getter]
    fn code_hash<'a>(&self, py: Python<'a>) -> Option<&'a PyBytes> {
        let code = self.0.code.as_ref()?;
        Some(PyBytes::new(py, code.repr_hash().as_slice()))
    }

    #[getter]
    fn get_code(&self) -> Option<Cell> {
        self.0.code.clone().map(Cell)
    }

    #[setter]
    fn set_code(&mut self, code: Option<Cell>) {
        self.0.code = code.map(|Cell(code)| code);
    }

    #[getter]
    fn get_data(&self) -> Option<Cell> {
        self.0.data.clone().map(Cell)
    }

    #[setter]
    fn set_data(&mut self, data: Option<Cell>) {
        self.0.data = data.map(|Cell(data)| data);
    }

    /// Adds the specified salt to the code of this state init.
    fn set_code_salt(&mut self, salt: &Cell) -> PyResult<()> {
        self.0.code = nt::abi::set_code_salt(self.expect_code()?, salt.0.clone())
            .handle_runtime_error()
            .map(Some)?;
        Ok(())
    }

    /// Tries to extract a salt from the code of this state init.
    fn get_code_salt(&self) -> PyResult<Option<Cell>> {
        let salt = nt::abi::get_code_salt(self.expect_code()?).handle_runtime_error()?;
        Ok(salt.map(Cell))
    }

    fn compute_address(&self, workchain: Option<i8>) -> PyResult<Address> {
        let cell = self.build_cell()?;
        ton_block::MsgAddressInt::with_standart(
            None,
            workchain.unwrap_or_default(),
            cell.0.repr_hash().into(),
        )
        .handle_runtime_error()
        .map(Address)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.0.serialize().handle_runtime_error()?;
        encoding.encode_cell(&cell)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.0.serialize().handle_runtime_error()?;
        let bytes = ton_types::serialize_toc(&cell).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        self.0.serialize().handle_runtime_error().map(Cell)
    }
}

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

    #[getter]
    fn get_workchain(&self) -> i32 {
        self.0.workchain_id()
    }

    #[setter]
    fn set_workchain(&mut self, workchain: i32) -> PyResult<()> {
        match &mut self.0 {
            ton_block::MsgAddressInt::AddrStd(addr) => {
                addr.workchain_id = workchain.try_into().handle_value_error()?
            }
            ton_block::MsgAddressInt::AddrVar(addr) => addr.workchain_id = workchain,
        }
        Ok(())
    }

    #[getter]
    fn account<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        let bytes = self.0.address().get_bytestring_on_stack(0);
        PyBytes::new(py, &bytes)
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
    fn from_bytes(mut bytes: &[u8]) -> PyResult<Self> {
        ton_types::deserialize_tree_of_cells(&mut bytes)
            .handle_value_error()
            .map(Self)
    }

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

    #[staticmethod]
    fn decode(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding.decode_cell(value.trim()).map(Self)
    }

    /// Constructs a new empty cell.
    #[new]
    fn new() -> Self {
        Self(Default::default())
    }

    /// Returns a hex encoded repr hash of the root cell.
    #[getter]
    fn repr_hash<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.repr_hash().as_slice())
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        encoding.encode_cell(&self.0)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let bytes = ton_types::serialize_toc(&self.0).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
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
