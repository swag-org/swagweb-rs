use pyo3::prelude::*;
use pyo3::PyAny;

use crate::routing::middleware::{HandlerMiddleware, MiddlewareEnum, PyMiddleware};

impl IntoPy<Py<PyAny>> for MiddlewareEnum {
    fn into_py(self, py: Python) -> Py<PyAny> {
        match self {
            MiddlewareEnum::Handler(m) => m.into_py(py),
            MiddlewareEnum::PyMiddleware(m) => m.into_py(py),
        }
    }
}

impl IntoPy<Py<HandlerMiddleware>> for HandlerMiddleware {
    fn into_py(self, py: Python) -> Py<HandlerMiddleware> {
        Py::new(py, self).unwrap()
    }
}

impl IntoPy<Py<PyMiddleware>> for PyMiddleware {
    fn into_py(self, py: Python) -> Py<PyMiddleware> {
        Py::new(py, self).unwrap()
    }
}
