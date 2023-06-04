use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod alignment;
mod alphabets;
mod seq_analysis;

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1", this is not full compatibility,
    // but it's good enough for now
    // see https://docs.rs/semver/1.0.9/semver/struct.Version.html#method.parse for rust spec
    // see https://peps.python.org/pep-0440/ for python spec
    // it seems the dot after "alpha/beta" e.g. "-alpha.1" is not necessary, hence why this works
    version.replace("-alpha", "a").replace("-beta", "b")
}

pub fn get_authors() -> Vec<String> {
    let authors = env!("CARGO_PKG_AUTHORS");
    authors.split(':').map(str::to_string).collect()
}

#[pymodule]
fn bioforma(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", get_version())?;
    m.add("__authors__", get_authors())?;
    m.add("build_profile", env!("PROFILE"))?;

    m.add_wrapped(wrap_pymodule!(alignment::alignment))?;
    m.add_wrapped(wrap_pymodule!(alphabets::alphabets))?;
    m.add_wrapped(wrap_pymodule!(seq_analysis::seq_analysis))?;

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;

    sys_modules.set_item("bioforma.alignment", m.getattr("alignment")?)?;
    sys_modules.set_item("bioforma.alphabets", m.getattr("alphabets")?)?;
    sys_modules.set_item("bioforma.seq_analysis", m.getattr("seq_analysis")?)?;

    Ok(())
}
