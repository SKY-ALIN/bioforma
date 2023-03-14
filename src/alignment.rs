use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use pyo3::exceptions::PyValueError;
use bio::alignment::distance::{
    hamming     as _hamming,
    levenshtein as _levenshtein,
    simd        as _simd,
};

#[pyfunction]
fn hamming(alpha: &[u8], beta: &[u8]) -> PyResult<u64> {
    if alpha.len() != beta.len() {
        Err(PyValueError::new_err(
            "hamming distance cannot be calculated for texts of different length"
        ))
    } else {
        Ok(_hamming(alpha, beta))
    }
}

#[pyfunction]
fn simd_hamming(alpha: &[u8], beta: &[u8]) -> PyResult<u64> {
    if alpha.len() != beta.len() {
        Err(PyValueError::new_err(
            "hamming distance cannot be calculated for texts of different length"
        ))
    } else {
        Ok(_simd::hamming(alpha, beta))
    }
}

#[pyfunction]
fn levenshtein(alpha: &[u8], beta: &[u8]) -> u32 {
    _levenshtein(alpha, beta)
}

#[pyfunction]
fn simd_levenshtein(alpha: &[u8], beta: &[u8]) -> u32 {
    _simd::levenshtein(alpha, beta)
}

#[pymodule]
fn distance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hamming, m)?)?;
    m.add_function(wrap_pyfunction!(simd_hamming, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(simd_levenshtein, m)?)?;
    Ok(())
}

#[pymodule]
pub fn alignment(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(distance))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alignment.distance", m.getattr("distance")?)?;

    Ok(())
}
