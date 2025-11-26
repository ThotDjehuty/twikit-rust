use pyo3::prelude::*;

#[pyclass]
pub struct Media {}

#[pymethods]
impl Media {
    #[new]
    pub fn new() -> Self {
        Media {}
    }

    /// Mock upload — returns a fake URL for the uploaded path
    pub fn upload(&self, path: String) -> PyResult<String> {
        Ok(format!("https://media.example.com/{}", path))
    }
}
