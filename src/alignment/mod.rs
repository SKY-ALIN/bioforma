use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod alignment_type;
mod distance;

#[pymodule]
pub fn alignment(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<alignment_type::AlignmentOperation>()?;
    m.add_class::<alignment_type::Match>()?;
    m.add_class::<alignment_type::Subst>()?;
    m.add_class::<alignment_type::Del>()?;
    m.add_class::<alignment_type::Ins>()?;
    m.add_class::<alignment_type::Xclip>()?;
    m.add_class::<alignment_type::Yclip>()?;
    m.add_class::<alignment_type::Alignment>()?;

    m.add_wrapped(wrap_pymodule!(distance::distance))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alignment.distance", m.getattr("distance")?)?;

    Ok(())
}
