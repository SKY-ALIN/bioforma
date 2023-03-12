use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use bio::scores::{
    blosum62    as _blosum62,
    pam120      as _pam120,
    pam200      as _pam200,
    pam250      as _pam250,
    pam40       as _pam40,
};

const VALUE_ERROR_MSG: &str = "Can't accept more then 1 byte";

#[pyfunction]
fn blosum62(a: &[u8], b: &[u8]) -> PyResult<i32> {
    if a.len() != 1 {
        return Err(PyValueError::new_err(VALUE_ERROR_MSG))
    }

    if b.len() != 1 {
        return Err(PyValueError::new_err(VALUE_ERROR_MSG))
    }

    Ok(_blosum62(a[0], b[0]))
}

#[pymodule]
pub fn scores(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(blosum62, m)?)?;
    Ok(())
}
