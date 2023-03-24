#[rustfmt::skip]
use bio::scores::{
    blosum62    as _blosum62,
    pam120      as _pam120,
    pam200      as _pam200,
    pam250      as _pam250,
    pam40       as _pam40,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

const VALUE_ERROR_MSG: &str = "Can't accept more then 1 byte";

fn score(a: &[u8], b: &[u8], func: fn(u8, u8) -> i32) -> PyResult<i32> {
    if a.len() != 1 {
        return Err(PyValueError::new_err(VALUE_ERROR_MSG));
    }

    if b.len() != 1 {
        return Err(PyValueError::new_err(VALUE_ERROR_MSG));
    }

    Ok(func(a[0], b[0]))
}

#[pyfunction]
fn blosum62(a: &[u8], b: &[u8]) -> PyResult<i32> {
    score(a, b, _blosum62)
}

#[pyfunction]
fn pam120(a: &[u8], b: &[u8]) -> PyResult<i32> {
    score(a, b, _pam120)
}

#[pyfunction]
fn pam200(a: &[u8], b: &[u8]) -> PyResult<i32> {
    score(a, b, _pam200)
}

#[pyfunction]
fn pam250(a: &[u8], b: &[u8]) -> PyResult<i32> {
    score(a, b, _pam250)
}

#[pyfunction]
fn pam40(a: &[u8], b: &[u8]) -> PyResult<i32> {
    score(a, b, _pam40)
}

#[pymodule]
pub fn scores(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(blosum62, m)?)?;
    m.add_function(wrap_pyfunction!(pam120, m)?)?;
    m.add_function(wrap_pyfunction!(pam200, m)?)?;
    m.add_function(wrap_pyfunction!(pam250, m)?)?;
    m.add_function(wrap_pyfunction!(pam40, m)?)?;
    Ok(())
}
