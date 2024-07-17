use content::Content;
use getset::Getters;
use headers::Headers;
use pyo3::pyclass;

mod content;
mod headers;
pub mod parser;

#[pyclass]
#[derive(Debug, Getters)]
pub struct Request {
    #[getset(get = "pub")]
    path: String,
    #[getset(get = "pub")]
    method: String,
    #[getset(get = "pub")]
    headers: Headers,
    #[getset(get = "pub")]
    content: Content,
}

impl Request {
    pub(crate) fn new(path: String, method: String, headers: Headers, content: Content) -> Self {
        Self {
            path,
            method,
            headers,
            content,
        }
    }
}
