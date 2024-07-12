use std::io::BufRead;

use pyo3::{
    exceptions::{PyIOError, PyValueError},
    PyResult,
};

pub struct RequestReader(Box<dyn BufRead>);

impl Iterator for RequestReader {
    type Item = PyResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut b = String::new();
        return match self.0.read_line(&mut b) {
            Ok(0) => None,
            Err(e) => Some(Err(PyIOError::new_err(format!(
                "Request reading failed: {e}"
            )))),
            _ if !b.ends_with("\r\n") => {
                return Some(Err(PyValueError::new_err("No CRLF at the end of line")));
            }
            _ => {
                b.pop();
                b.pop();
                return Some(Ok(b));
            }
        };
    }
}

impl RequestReader {
    pub fn new(inner: Box<dyn BufRead>) -> Self {
        Self(inner)
    }

    pub fn inner(&mut self) -> &mut Box<dyn BufRead> {
        &mut self.0
    }
}
