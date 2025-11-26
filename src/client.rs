use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::tweet::Tweet;
use std::time::{SystemTime, UNIX_EPOCH};

#[pyclass]
pub struct Client {
    token: Option<String>,
}

#[pymethods]
impl Client {
    #[new]
    pub fn new() -> Self {
        Client { token: None }
    }

    #[staticmethod]
    pub fn with_token(token: String) -> Self {
        Client { token: Some(token) }
    }

    /// Post a tweet (mock) — returns a `Tweet` instance.
    pub fn post_tweet(&self, text: String, author: String) -> PyResult<Tweet> {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Ok(Tweet::new(id, text, author))
    }

    /// Retrieve a tweet by id (mock response)
    pub fn get_tweet(&self, id: u64) -> PyResult<Tweet> {
        Ok(Tweet::new(id, format!("This is tweet {}", id), "author".to_string()))
    }

    /// Return some client info
    pub fn info<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let d = PyDict::new(py);
        d.set_item("has_token", self.token.is_some())?;
        Ok(d)
    }
}
