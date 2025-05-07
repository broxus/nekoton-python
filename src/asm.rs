use pyo3::prelude::*;

use crate::models::Cell;
use crate::util::HandleError;

#[pyclass]
pub struct Asm;

#[pymethods]
impl Asm {
    #[staticmethod]
    pub fn compile(asm: &str) -> PyResult<Cell> {
        let code = everscale_asm::Code::assemble(asm).handle_runtime_error()?;
        let code = everscale_types::boc::Boc::encode(code);
        ton_types::deserialize_tree_of_cells(&mut code.as_slice())
            .handle_runtime_error()
            .map(Cell)
    }
}
