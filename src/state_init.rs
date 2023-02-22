use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ton_block::{Deserializable, Serializable};

use crate::cell::Cell;
use crate::subscription::Address;
use crate::util::{Encoding, HandleError};

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
