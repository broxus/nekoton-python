use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ton_block::Deserializable;

use crate::cell::Cell;
use crate::util::{Encoding, HandleError};

#[pyclass]
pub struct StateInit {
    code: Cell,
    data: Cell,
}

#[pymethods]
impl StateInit {
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        let state_init = ton_block::StateInit::construct_from_bytes(bytes).handle_value_error()?;
        Ok(Self {
            code: Cell(state_init.code.unwrap_or_default()),
            data: Cell(state_init.data.unwrap_or_default()),
        })
    }

    #[new]
    fn new(code: Option<&Cell>, data: Option<&Cell>) -> Self {
        Self {
            code: code.cloned().unwrap_or_default(),
            data: data.cloned().unwrap_or_default(),
        }
    }

    #[getter]
    fn get_code(&self) -> Cell {
        self.code.clone()
    }

    #[setter]
    fn set_code(&mut self, code: Cell) {
        self.code = code;
    }

    #[getter]
    fn get_data(&self) -> Cell {
        self.code.clone()
    }

    #[setter]
    fn set_data(&mut self, data: Cell) {
        self.data = data;
    }

    /// Adds the specified salt to the code of this state init.
    fn set_code_salt(&mut self, salt: &Cell) -> PyResult<()> {
        self.code = nt::abi::set_code_salt(self.code.0.clone(), salt.0.clone())
            .handle_runtime_error()
            .map(Cell)?;
        Ok(())
    }

    /// Tries to extract a salt from the code of this state init.
    fn get_code_salt(&self) -> PyResult<Option<Cell>> {
        let salt = nt::abi::get_code_salt(self.code.0.clone()).handle_runtime_error()?;
        Ok(salt.map(Cell))
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Base64)?;
        let cell = self.build_cell()?;
        encoding.encode_cell(&cell.0)
    }

    fn to_bytes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        let cell = self.build_cell()?;
        let bytes = ton_types::serialize_toc(&cell.0).handle_runtime_error()?;
        Ok(PyBytes::new(py, &bytes))
    }

    fn build_cell(&self) -> PyResult<Cell> {
        Cell::try_from_struct(&ton_block::StateInit {
            code: Some(self.code.0.clone()),
            data: Some(self.data.0.clone()),
            ..Default::default()
        })
    }
}
