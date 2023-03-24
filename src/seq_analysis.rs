#[rustfmt::skip]
use bio::seq_analysis::gc::{
    gc3_content as _gc3_content,
    gc_content  as _gc_content,
};
#[rustfmt::skip]
use bio::seq_analysis::orf::{
    Finder      as _Finder,
    Orf         as _Orf,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use std::array::TryFromSliceError;

#[pyfunction]
fn gc_content(sequence: &[u8]) -> f32 {
    _gc_content(sequence)
}

#[pyfunction]
fn gc3_content(sequence: &[u8]) -> f32 {
    _gc3_content(sequence)
}

fn retype_vec<'a>(source_vec: Vec<&'a [u8]>) -> PyResult<Vec<&'a [u8; 3]>> {
    if source_vec.is_empty() {
        return Err(PyValueError::new_err("Start and end codons can't be empty"));
    }

    let mut target_vec: Vec<&'a [u8; 3]> = Vec::new();
    for &codon in source_vec.iter() {
        let res: Result<&[u8; 3], TryFromSliceError> = codon.try_into();
        if res.is_err() {
            return Err(PyValueError::new_err(
                "Every start and end codon must have only 3 bytes",
            ));
        }
        target_vec.push(res.unwrap());
    }
    Ok(target_vec)
}

#[pyclass]
struct Finder(_Finder);

#[pymethods]
impl Finder {
    #[new]
    pub fn new(start: Vec<&[u8]>, stop: Vec<&[u8]>, min_len: usize) -> PyResult<Self> {
        let start_codons_res = retype_vec(start);
        let stop_codons_res = retype_vec(stop);

        if let Err(err) = start_codons_res {
            Err(err)
        } else if let Err(err) = stop_codons_res {
            Err(err)
        } else {
            let _finder: _Finder =
                _Finder::new(start_codons_res.unwrap(), stop_codons_res.unwrap(), min_len);
            Ok(Finder(_finder))
        }
    }

    pub fn find_all(&self, sequence: &[u8]) -> Vec<Orf> {
        self.0.find_all(sequence).map(Orf).collect()
    }
}

#[pyclass]
struct Orf(_Orf);

#[pymethods]
impl Orf {
    #[getter]
    pub fn start(&self) -> usize {
        self.0.start
    }

    #[getter]
    pub fn end(&self) -> usize {
        self.0.end
    }

    #[getter]
    pub fn offset(&self) -> i8 {
        self.0.offset
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<Orf: start={}, end={}, offset={}>",
            self.0.start, self.0.end, self.0.offset
        )
    }
}

#[pymodule]
fn orf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Finder>()?;
    m.add_class::<Orf>()?;
    Ok(())
}

#[pymodule]
fn gc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gc_content, m)?)?;
    m.add_function(wrap_pyfunction!(gc3_content, m)?)?;
    Ok(())
}

#[pymodule]
pub fn seq_analysis(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(gc))?;
    m.add_wrapped(wrap_pymodule!(orf))?;

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;

    sys_modules.set_item("bioforma.seq_analysis.gc", m.getattr("gc")?)?;
    sys_modules.set_item("bioforma.seq_analysis.orf", m.getattr("orf")?)?;

    Ok(())
}
