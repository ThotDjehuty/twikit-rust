use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct User {
    store: HashMap<String, String>,
}

#[pymethods]
impl User {
    #[new]
    pub fn new() -> Self {
        User { store: HashMap::new() }
    }

    pub fn create(&mut self, username: String) {
        self.store.insert(username.clone(), format!("info:{}", username));
    }

    pub fn get(&self, username: String) -> Option<String> {
        self.store.get(&username).cloned()
    }
}
