use std::collections::HashMap;

use crate::http::request::HttpRequest;
use pyo3::{pyclass, PyObject};

#[pyclass(get_all)]
#[derive(Clone)]
pub struct HttpContext {
    pub request: HttpRequest,
    pub vars: HashMap<String, PyObject>,
}
