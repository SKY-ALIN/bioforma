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
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[pyclass(subclass)]
struct AlignmentOperation(_AlignmentOperation);

impl AlignmentOperation {
    fn get_operation(&self) -> _AlignmentOperation {
        self.0
    }
}

#[pyclass(extends=AlignmentOperation)]
struct Match(_AlignmentOperation);

#[pyclass(extends=AlignmentOperation)]
struct Subst(_AlignmentOperation);

#[pyclass(extends=AlignmentOperation)]
struct Del(_AlignmentOperation);

#[pyclass(extends=AlignmentOperation)]
struct Ins(_AlignmentOperation);

#[pyclass(extends=AlignmentOperation)]
struct Xclip(_AlignmentOperation);

#[pyclass(extends=AlignmentOperation)]
struct Yclip(_AlignmentOperation);

fn hash(slf: _AlignmentOperation) -> u64 {
    let mut hasher = DefaultHasher::new();
    slf.hash(&mut hasher);
    hasher.finish()
}

fn richcmp(slf: _AlignmentOperation, other: _AlignmentOperation, op: CompareOp) -> PyResult<bool> {
    match op {
        CompareOp::Eq => Ok(slf == other),
        CompareOp::Ne => Ok(slf != other),
        _ => Err(PyNotImplementedError::new_err(
            "Operation isn't supported for this type",
        )),
    }
}

#[pymethods]
impl Match {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (
            Match(_AlignmentOperation::Match),
            AlignmentOperation(_AlignmentOperation::Match),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> String {
        "<Match>".into()
    }
}

#[pymethods]
impl Subst {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (
            Subst(_AlignmentOperation::Subst),
            AlignmentOperation(_AlignmentOperation::Subst),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> String {
        "<Subst>".into()
    }
}

#[pymethods]
impl Del {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (
            Del(_AlignmentOperation::Del),
            AlignmentOperation(_AlignmentOperation::Del),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> String {
        "<Del>".into()
    }
}

#[pymethods]
impl Ins {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (
            Ins(_AlignmentOperation::Ins),
            AlignmentOperation(_AlignmentOperation::Ins),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> String {
        "<Ins>".into()
    }
}

#[pymethods]
impl Xclip {
    #[new]
    pub fn new(x: usize) -> (Self, AlignmentOperation) {
        (
            Xclip(_AlignmentOperation::Xclip(x)),
            AlignmentOperation(_AlignmentOperation::Xclip(x)),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> Option<String> {
        match self.0 {
            _AlignmentOperation::Xclip(x) => Some(format!("<Xclip: x={}>", x)),
            _ => None,
        }
    }
}

#[pymethods]
impl Yclip {
    #[new]
    pub fn new(y: usize) -> (Self, AlignmentOperation) {
        (
            Yclip(_AlignmentOperation::Yclip(y)),
            AlignmentOperation(_AlignmentOperation::Yclip(y)),
        )
    }

    fn __hash__(&self) -> u64 {
        hash(self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        richcmp(self.0, other.0, op)
    }

    pub fn __repr__(&self) -> Option<String> {
        match self.0 {
            _AlignmentOperation::Yclip(y) => Some(format!("<Yclip: y={}>", y)),
            _ => None,
        }
    }
}

fn rust_bio_alignment_operation_into_py_object(
    _operation: _AlignmentOperation,
    py: Python,
) -> Option<PyObject> {
    match _operation {
        _AlignmentOperation::Match => PyCell::new(py, Match::new()).ok().map(|o| o.to_object(py)),
        _AlignmentOperation::Subst => PyCell::new(py, Subst::new()).ok().map(|o| o.to_object(py)),
        _AlignmentOperation::Del => PyCell::new(py, Del::new()).ok().map(|o| o.to_object(py)),
        _AlignmentOperation::Ins => PyCell::new(py, Ins::new()).ok().map(|o| o.to_object(py)),
        _AlignmentOperation::Xclip(x) => {
            PyCell::new(py, Xclip::new(x)).ok().map(|o| o.to_object(py))
        },
        _AlignmentOperation::Yclip(y) => {
            PyCell::new(py, Yclip::new(y)).ok().map(|o| o.to_object(py))
        },
    }
}

#[pyclass]
struct Alignment(_Alignment);

#[pymethods]
impl Alignment {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (score, x_start, y_start, x_end, y_end, x_len, y_len, operations, mode="global"))]
    pub fn new(
        score: i32,
        x_start: usize,
        y_start: usize,
        x_end: usize,
        y_end: usize,
        x_len: usize,
        y_len: usize,
        operations: Vec<&PyCell<AlignmentOperation>>,
        mode: &str,
    ) -> PyResult<Self> {
        let _operations: Vec<_AlignmentOperation> = operations
            .iter()
            .map(|o| o.borrow().get_operation())
            .collect();

        let _mode: PyResult<_AlignmentMode> = match mode {
            "local" => Ok(_AlignmentMode::Local),
            "semiglobal" => Ok(_AlignmentMode::Semiglobal),
            "global" => Ok(_AlignmentMode::Global),
            "custom" => Ok(_AlignmentMode::Custom),
            _ => Err(PyValueError::new_err(format!(
                "{} can't be used as the mode",
                mode
            ))),
        };

        Ok(Alignment(_Alignment {
            score,
            xstart: x_start,
            ystart: y_start,
            xend: x_end,
            yend: y_end,
            xlen: x_len,
            ylen: y_len,
            operations: _operations,
            mode: _mode?,
        }))
    }

    pub fn cigar(&self, hard_clip: bool) -> String {
        self.0.cigar(hard_clip)
    }

    pub fn pretty(&self, x: &[u8], y: &[u8]) -> String {
        self.0.pretty(x, y)
    }

    pub fn path(&self, py: Python) -> Vec<(usize, usize, Option<PyObject>)> {
        self.0
            .path()
            .into_iter()
            .map(|(a, b, _alignment_operation)| {
                let operation =
                    rust_bio_alignment_operation_into_py_object(_alignment_operation, py);
                (a, b, operation)
            })
            .collect()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<Alignment: score={}, x_start={}, y_start={}, x_end={}, y_end={}, x_len={}, y_len={}, operations={:?}, mode={:?}>",
            self.0.score, self.0.xstart, self.0.ystart, self.0.xend, self.0.yend, self.0.xlen, self.0.ylen, self.0.operations, self.0.mode
        )
    }
}

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
fn distance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hamming, m)?)?;
    m.add_function(wrap_pyfunction!(simd_hamming, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(simd_levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(simd_bounded_levenshtein, m)?)?;
    Ok(())
}

#[pymodule]
pub fn alignment(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AlignmentOperation>()?;
    m.add_class::<Match>()?;
    m.add_class::<Subst>()?;
    m.add_class::<Del>()?;
    m.add_class::<Ins>()?;
    m.add_class::<Xclip>()?;
    m.add_class::<Yclip>()?;
    m.add_class::<Alignment>()?;

    // m.add_class::<Scoring>()?;
    m.add_class::<Scoring>()?;
    m.add_class::<PairwiseAligner>()?;

    m.add_wrapped(wrap_pymodule!(distance))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alignment.distance", m.getattr("distance")?)?;

    Ok(())
}
