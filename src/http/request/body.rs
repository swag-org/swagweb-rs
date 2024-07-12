use pyo3::{
    exceptions::{PyIOError, PyValueError},
    prelude::*,
};

use crate::utils::RequestReader;

use super::HttpHeaders;

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct HttpRequestBody {
    text: Option<String>,
    fields: Option<Vec<String>>,
    files: Option<Vec<()>>,
}

impl HttpRequestBody {
    pub fn from_reader(reader: &mut RequestReader, headers: &HttpHeaders) -> PyResult<Self> {
        let content_type = headers
            .get("Content-Type")
            .ok_or(PyValueError::new_err("No Content-Type header"))?;
        let mut parts = content_type.split("; ");
        let mime = parts
            .next()
            .ok_or(PyValueError::new_err("Empty Content-Type header"))?;
        if mime.starts_with("text/") || mime.starts_with("application/") {
            let mut b = String::with_capacity(
                headers
                    .get("Content-Length")
                    .and_then(|x| x.parse::<usize>().ok())
                    .unwrap_or(0),
            );
            reader
                .inner()
                .read_to_string(&mut b)
                .map_err(|e| PyIOError::new_err(format!("Request reading failed: {e}")))?;
            return Ok(Self {
                text: Some(b),
                fields: None,
                files: None,
            });
        } else {
            todo!()
        }
    }
}
