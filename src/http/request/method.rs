use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    pub fn try_from<T: AsRef<str>>(value: T) -> Option<HttpMethod> {
        match value.as_ref() {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            _ => None,
        }
    }
}
