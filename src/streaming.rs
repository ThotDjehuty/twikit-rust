use pyo3::prelude::*;
use pyo3::types::PyAny;
use crate::tweet::Tweet;

#[pyclass]
pub struct Stream {}

#[pymethods]
impl Stream {
    #[new]
    pub fn new() -> Self {
        Stream {}
    }

    /// Start the stream and synchronously call the provided Python callback once with a mock Tweet.
    /// The callback should accept one argument: a `Tweet` instance.
    pub fn start(&self, py: Python, callback: PyObject) -> PyResult<()> {
        // Create a mock tweet and pass it to the Python callback
        let tweet = Tweet::new(1, "streamed tweet".to_string(), "stream_user".to_string());
        let tweet_py = Py::new(py, tweet)?;
        callback.call1(py, (tweet_py,))?;
        Ok(())
    }
}
