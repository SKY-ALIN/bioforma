use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::types::{IntoPyDict, PyBytes, PyDict};
use pyo3::exceptions::PyValueError;
use bio::alphabets::{
    Alphabet        as _Alphabet,
    RankTransform   as _RankTransform,
};
use bio::alphabets::dna::{
    alphabet        as _dna_alphabet,
    n_alphabet      as _dna_n_alphabet,
    iupac_alphabet  as _dna_iupac_alphabet,
    complement      as _dna_complement,
    revcomp         as _dna_revcomp,
};
use bio::alphabets::protein::{
    alphabet        as _protein_alphabet,
    iupac_alphabet  as _protein_iupac_alphabet,
};
use bio::alphabets::rna::{
    alphabet        as _rna_alphabet,
    n_alphabet      as _rna_n_alphabet,
    iupac_alphabet  as _rna_iupac_alphabet,
    complement      as _rna_complement,
    revcomp         as _rna_revcomp,
};

#[pyclass]
struct Alphabet(_Alphabet);

#[pymethods]
impl Alphabet {
    #[new]
    pub fn new(symbols: &[u8]) -> PyResult<Self> {
        if symbols.is_empty() {
            Err(PyValueError::new_err("Empty alphabet"))
        } else {
            Ok(Alphabet ( _Alphabet::new(symbols) ))
        }
    }

    #[getter]
    pub fn symbols<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        let v: Vec<u8> = self.0.symbols.clone().iter().map(|a| a as u8).collect();
        PyBytes::new(py, v.as_slice())
    }

    pub fn is_word(&self, text: &[u8]) -> bool {
        self.0.is_word(text)
    }

    pub fn __repr__(&self) -> String {
        let v: Vec<u8> = self.0.symbols.clone().iter().map(|a| a as u8).collect();
        format!("<Alphabet: {}>", String::from_utf8(v).unwrap_or("invalid".into()))
    }

    pub fn __bytes__<'p>(&self, py: Python<'p>) -> &'p PyBytes {
        self.symbols(py)
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __and__(&self, other: &Alphabet) -> Self {
        Alphabet ( self.0.intersection(&other.0) )
    }

    pub fn __or__(&self, other: &Alphabet) -> Self {
        Alphabet ( self.0.union(&other.0) )
    }
}

#[pyclass]
struct RankTransform(_RankTransform);

#[pymethods]
impl RankTransform {
    #[new]
    pub fn new(alphabet: &Alphabet) -> Self {
        RankTransform ( _RankTransform::new(&alphabet.0) )
    }

    pub fn get(&self, chr: &[u8]) -> PyResult<u8> {
        if chr.len() != 1 {
            Err(PyValueError::new_err("Can't accept more then 1 byte"))
        } else {
            Ok(self.0.get(chr[0]))
        }
    }

    pub fn transform(&self, text: &[u8]) -> PyResult<Vec<u8>> {
        let res = std::panic::catch_unwind(|| {self.0.transform(text)});
        match res {
            Ok(res) => Ok(res),
            Err(res) => Err(PyValueError::new_err(res.downcast::<String>().unwrap().to_string())),
        }
    }

    pub fn q_grams(&self, q: u32, text: &[u8]) -> Vec<usize> {
        self.0.qgrams(q, text).collect()
    }

    pub fn get_width(&self) -> usize {
        self.0.get_width()
    }

    #[getter]
    pub fn ranks<'p>(&self, py: Python<'p>) -> &'p PyDict {
        let mut res = HashMap::new();
        for (key, value) in self.0.ranks.iter() {
            let chr = char::from(key as u8);
            res.insert(chr.to_string(), *value);
        }
        res.into_py_dict(py)
    }

    pub fn __repr__(&self) -> String {
        let a: Vec<String> = self.0.ranks
            .iter()
            .map(|(key, value)|
                format!("{}-{}", char::from(key as u8), value)
            )
            .collect();
        format!("<RankTransform: {}>", a.join(", "))
    }
}

// #[pyclass]
// struct QGrams(_QGrams<'static, &'static u8, Iter<'static, u8>>);

#[pyfunction]
fn make_dna_alphabet() -> Alphabet {
    Alphabet ( _dna_alphabet() )
}

#[pyfunction]
fn make_dna_n_alphabet() -> Alphabet {
    Alphabet ( _dna_n_alphabet() )
}

#[pyfunction]
fn make_dna_iupac_alphabet() -> Alphabet {
    Alphabet ( _dna_iupac_alphabet() )
}

#[pyfunction]
fn get_dna_symbol_complement<'p>(chr: &[u8], py: Python<'p>) -> PyResult<&'p PyBytes> {
    if chr.len() != 1 {
        Err(PyValueError::new_err("Can't accept more then 1 byte"))
    } else {
        let arr = [_dna_complement(chr[0])];
        Ok(PyBytes::new(py, arr.as_slice()))
    }
}

#[pyfunction]
fn get_dna_complement<'p>(text: &[u8], py: Python<'p>) -> &'p PyBytes {
    PyBytes::new(py, _dna_revcomp(text).as_slice())
}

#[pymodule]
fn dna(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_dna_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(make_dna_n_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(make_dna_iupac_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(get_dna_symbol_complement, m)?)?;
    m.add_function(wrap_pyfunction!(get_dna_complement, m)?)?;
    Ok(())
}

#[pyfunction]
fn make_protein_alphabet() -> Alphabet {
    Alphabet ( _protein_alphabet() )
}

#[pyfunction]
fn make_protein_iupac_alphabet() -> Alphabet {
    Alphabet ( _protein_iupac_alphabet() )
}

#[pymodule]
fn protein(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_protein_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(make_protein_iupac_alphabet, m)?)?;
    Ok(())
}

#[pyfunction]
fn make_rna_alphabet() -> Alphabet {
    Alphabet ( _rna_alphabet() )
}

#[pyfunction]
fn make_rna_n_alphabet() -> Alphabet {
    Alphabet ( _rna_n_alphabet() )
}

#[pyfunction]
fn make_rna_iupac_alphabet() -> Alphabet {
    Alphabet ( _rna_iupac_alphabet() )
}

#[pyfunction]
fn get_rna_symbol_complement<'p>(chr: &[u8], py: Python<'p>) -> PyResult<&'p PyBytes> {
    if chr.len() != 1 {
        Err(PyValueError::new_err("Can't accept more then 1 byte"))
    } else {
        let arr = [_rna_complement(chr[0])];
        Ok(PyBytes::new(py, arr.as_slice()))
    }
}

#[pyfunction]
fn get_rna_complement<'p>(text: &[u8], py: Python<'p>) -> &'p PyBytes {
    PyBytes::new(py, _rna_revcomp(text).as_slice())
}

#[pymodule]
fn rna(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_rna_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(make_rna_n_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(make_rna_iupac_alphabet, m)?)?;
    m.add_function(wrap_pyfunction!(get_rna_symbol_complement, m)?)?;
    m.add_function(wrap_pyfunction!(get_rna_complement, m)?)?;
    Ok(())
}

#[pymodule]
pub fn alphabets(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Alphabet>()?;
    m.add_class::<RankTransform>()?;

    m.add_wrapped(wrap_pymodule!(dna))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alphabets.dna", m.getattr("dna")?)?;

    m.add_wrapped(wrap_pymodule!(protein))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alphabets.protein", m.getattr("protein")?)?;

    m.add_wrapped(wrap_pymodule!(rna))?;
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("bioforma.alphabets.rna", m.getattr("rna")?)?;

    Ok(())
}
