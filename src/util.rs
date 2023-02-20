use std::collections::HashMap;

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
