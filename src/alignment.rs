use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use pyo3::exceptions::PyValueError;
use bio::alignment::distance::{
    hamming             as _hamming,
    levenshtein         as _levenshtein,
    simd                as _simd,
};
use bio_types::alignment::{
    Alignment           as _Alignment,
    AlignmentMode       as _AlignmentMode,
    AlignmentOperation  as _AlignmentOperation,
};

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

#[pymethods]
impl Match {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (Match(_AlignmentOperation::Match), AlignmentOperation(_AlignmentOperation::Match))
    }

    pub fn __repr__(&self) -> String {
        "<Match>".into()
    }
}

#[pymethods]
impl Subst {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (Subst(_AlignmentOperation::Subst), AlignmentOperation(_AlignmentOperation::Subst))
    }

    pub fn __repr__(&self) -> String {
        "<Subst>".into()
    }
}

#[pymethods]
impl Del {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (Del(_AlignmentOperation::Del), AlignmentOperation(_AlignmentOperation::Del))
    }

    pub fn __repr__(&self) -> String {
        "<Del>".into()
    }
}

#[pymethods]
impl Ins {
    #[new]
    pub fn new() -> (Self, AlignmentOperation) {
        (Ins(_AlignmentOperation::Ins), AlignmentOperation(_AlignmentOperation::Ins))
    }

    pub fn __repr__(&self) -> String {
        "<Ins>".into()
    }
}

#[pymethods]
impl Xclip {
    #[new]
    pub fn new(x: usize) -> (Self, AlignmentOperation) {
        (Xclip(_AlignmentOperation::Xclip(x)), AlignmentOperation(_AlignmentOperation::Xclip(x)))
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
    pub fn new(x: usize) -> (Self, AlignmentOperation) {
        (Yclip(_AlignmentOperation::Yclip(x)), AlignmentOperation(_AlignmentOperation::Yclip(x)))
    }

    pub fn __repr__(&self) -> Option<String> {
        match self.0 {
            _AlignmentOperation::Yclip(y) => Some(format!("<Yclip: y={}>", y)),
            _ => None,
        }
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
            _ => Err(PyValueError::new_err(format!("{} can't be used as the mode", mode))),
        };

        Ok(Alignment(_Alignment{
            score,
            xstart: x_start,
            ystart: y_start,
            xend: x_end,
            yend: y_end,
            ylen: x_len,
            xlen: y_len,
            operations: _operations,
            mode: _mode?,
        }))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<Alignment: score={}, x_start={}, y_start={}, x_end={}, y_end={}, x_len={}, y_len={}, operations={:?}, mode={:?}>",
            self.0.score, self.0.xstart, self.0.ystart, self.0.xend, self.0.yend, self.0.xlen, self.0.ylen, self.0.operations, self.0.mode
        )
    }
}

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

    m.add_wrapped(wrap_pymodule!(distance))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alignment.distance", m.getattr("distance")?)?;

    Ok(())
}
