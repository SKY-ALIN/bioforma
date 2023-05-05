use bio::alignment::pairwise::{
    Scoring             as _Scoring,
    Aligner             as _Aligner,
    MatchParams         as _MatchParams,
    MatchFunc           as _MatchFunc,
};
#[rustfmt::skip]
use bio::alignment::distance::{
    hamming             as _hamming,
    levenshtein         as _levenshtein,
    simd                as _simd,
};
#[rustfmt::skip]
use bio::scores::{
    blosum62    as _blosum62,
    pam120      as _pam120,
    pam200      as _pam200,
    pam250      as _pam250,
    pam40       as _pam40,
};
#[rustfmt::skip]
use bio_types::alignment::{
    Alignment           as _Alignment,
    AlignmentMode       as _AlignmentMode,
    AlignmentOperation  as _AlignmentOperation,
};
use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use pyo3::wrap_pymodule;
use std::collections::hash_map::DefaultHasher;

#[pyclass]
struct Scoring(_Scoring<Box<dyn Fn(u8, u8) -> i32 + Send>>);

#[pymethods]
impl Scoring {
    #[new]
    pub fn new(gap_open: i32, gap_extend: i32, match_func: &str) -> PyResult<Self> {
        // TODO: check gap_open and gap_extend
        let func: PyResult<fn(u8, u8) -> i32> = match match_func {
            "blosum62" => Ok(_blosum62),
            "pam120" => Ok(_pam120),
            "pam200" => Ok(_pam200),
            "pam250" => Ok(_pam250),
            "pam40" => Ok(_pam40),
            _ => Err(PyValueError::new_err("")),
        };

        Ok(Scoring(_Scoring::new(gap_open, gap_extend, Box::new(func?))))
    }

    #[classmethod]
    pub fn from_scores(
        _cls: &PyType,
        gap_open: i32,
        gap_extend: i32,
        match_score: i32,
        mismatch_score: i32,
    ) -> Self {
        // TODO: check gap_open, gap_extend, match_score, mismatch_score
        let func = move |a: u8, b: u8| {
            if a == b {
                match_score
            } else {
                mismatch_score
            }
        };
        Scoring(_Scoring::new(gap_open, gap_extend, Box::new(func)))
    }
}

#[pyclass]
struct PairwiseAligner(_Aligner<Box<dyn Fn(u8, u8) -> i32 + Send>>);

#[pymethods]
impl PairwiseAligner {
    #[classmethod]
    pub fn from_scoring(
        _cls: &PyType,
        scoring: &Scoring,
    ) -> Self {
        // PairwiseAligner(_Aligner::with_scoring(scoring.0))
        todo!()
    }
}

#[pymodule]
pub fn alignment(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<Scoring>()?;
    m.add_class::<Scoring>()?;
    m.add_class::<PairwiseAligner>()?;
    Ok(())
}
