use pyo3::prelude::*;
use bio::seq_analysis::gc::{
    gc_content  as _gc_content,
    gc3_content as _gc3_content,
};
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

#[pyfunction]
fn gc_content(sequence: &[u8]) -> f32 {
    _gc_content(sequence)
}

#[pyfunction]
fn gc3_content(sequence: &[u8]) -> f32 {
    _gc3_content(sequence)
}

#[pymodule]
pub fn gc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gc_content, m)?)?;
    m.add_function(wrap_pyfunction!(gc3_content, m)?)?;
    Ok(())
}

#[pymodule]
pub fn seq_analysis(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(gc))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.seq_analysis.gc", m.getattr("gc")?)?;

    Ok(())
}
