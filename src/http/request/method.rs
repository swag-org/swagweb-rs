use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub enum HttpMethod {
    Get,
    Put,
    Head,
    Post,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn try_from<T: AsRef<str>>(value: T) -> Option<HttpMethod> {
        match value.as_ref() {
            "GET" => Some(HttpMethod::Get),
            "PUT" => Some(HttpMethod::Put),
            "HEAD" => Some(HttpMethod::Head),
            "POST" => Some(HttpMethod::Post),
            "PATCH" => Some(HttpMethod::Patch),
            "DELETE" => Some(HttpMethod::Delete),
            _ => None,
        }
    }
}
