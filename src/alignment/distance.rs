#[rustfmt::skip]
use bio::alignment::distance::{
    hamming             as _hamming,
    levenshtein         as _levenshtein,
    simd                as _simd,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn hamming(alpha: &[u8], beta: &[u8]) -> PyResult<u64> {
    if alpha.len() != beta.len() {
        Err(PyValueError::new_err(
            "hamming distance cannot be calculated for texts of different length",
        ))
    } else {
        Ok(_hamming(alpha, beta))
    }
}

#[pyfunction]
fn simd_hamming(alpha: &[u8], beta: &[u8]) -> PyResult<u64> {
    if alpha.len() != beta.len() {
        Err(PyValueError::new_err(
            "hamming distance cannot be calculated for texts of different length",
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

#[pyfunction]
fn simd_bounded_levenshtein(alpha: &[u8], beta: &[u8], k: u32) -> Option<u32> {
    _simd::bounded_levenshtein(alpha, beta, k)
}

#[pymodule]
pub fn distance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hamming, m)?)?;
    m.add_function(wrap_pyfunction!(simd_hamming, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(simd_levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(simd_bounded_levenshtein, m)?)?;
    Ok(())
}
