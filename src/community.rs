use pyo3::prelude::*;

#[pyclass]
pub struct Community {
    names: Vec<String>,
}

#[pymethods]
impl Community {
    #[new]
    pub fn new() -> Self {
        Community { names: Vec::new() }
    }

    pub fn create(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn list(&self) -> Vec<String> {
        self.names.clone()
    }
}
