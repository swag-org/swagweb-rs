use std::collections::HashMap;

use crate::http::request::Request;
use pyo3::{pyclass, PyObject};

#[pyclass(get_all)]
#[derive(Clone)]
pub struct HttpContext {
    pub request: Request,
    pub vars: HashMap<String, PyObject>,
}
