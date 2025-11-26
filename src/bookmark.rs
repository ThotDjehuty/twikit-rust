use pyo3::prelude::*;

#[pyclass]
pub struct Bookmark {
    bookmarks: Vec<String>,
}

#[pymethods]
impl Bookmark {
    #[new]
    pub fn new() -> Self {
        Bookmark {
            bookmarks: Vec::new(),
        }
    }

    pub fn add(&mut self, url: String) {
        self.bookmarks.push(url);
    }

    pub fn list(&self) -> Vec<String> {
        self.bookmarks.clone()
    }
}
