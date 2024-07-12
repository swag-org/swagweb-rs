use pyo3::prelude::*;

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

impl IntoPy<Py<PyAny>> for HttpMethod {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Put => "PUT",
            HttpMethod::Head => "HEAD",
            HttpMethod::Post => "POST",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }.into_py(py)
    }
}
