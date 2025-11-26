use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct Tweet {
    id: u64,
    text: String,
    author: String,
}

#[pymethods]
impl Tweet {
    #[new]
    pub fn new(id: u64, text: String, author: String) -> Self {
        Tweet { id, text, author }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn to_dict<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id)?;
        dict.set_item("text", &self.text)?;
        dict.set_item("author", &self.author)?;
        Ok(dict)
    }
}
