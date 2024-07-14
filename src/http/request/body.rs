use pyo3::prelude::*;

use crate::utils::request_reader;

use super::error::{Error, Result};
use super::HttpHeaders;

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct HttpRequestBody {
    pub text: String,
    pub fields: Option<Vec<String>>,
    pub files: Option<Vec<()>>,
}

impl HttpRequestBody {
    pub fn from_reader(reader: &mut request_reader::Reader, headers: &HttpHeaders) -> Result<Self> {
        let content_type = headers.get("Content-Type").ok_or(Error::InvalidHeader(
            "Content-Type".into(),
            "<not present>".into(),
        ))?;
        let mut parts = content_type.split("; ");
        let mime = parts.next().ok_or(Error::InvalidHeader(
            "Content-Type".into(),
            "<empty>".into(),
        ))?;
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
                .map_err(|e| request_reader::Error::ReadFailed(e))?;
            return Ok(Self {
                text: b,
                fields: None,
                files: None,
            });
        } else {
            todo!()
        }
    }
}
