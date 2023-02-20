use pyo3::prelude::*;

use crate::cell::Cell;
use crate::util::HandleError;

#[pyclass]
pub struct StateInit {
    code: Cell,
    data: Cell,
}

#[pymethods]
impl StateInit {
    #[new]
    fn new(code: Option<&Cell>, data: Option<&Cell>) -> Self {
        Self {
            code: code.cloned().unwrap_or_default(),
            data: data.cloned().unwrap_or_default(),
        }
    }

    #[getter]
    fn code(&self) -> Cell {
        self.code.clone()
    }

    #[getter]
    fn data(&self) -> Cell {
        self.code.clone()
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

    fn serialize(&self) -> PyResult<Cell> {
        Cell::try_from_struct(&ton_block::StateInit {
            code: Some(self.code.0.clone()),
            data: Some(self.data.0.clone()),
            ..Default::default()
        })
    }
}
