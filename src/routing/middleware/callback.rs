use crate::http::context::HttpContext;
use crate::http::response::HttpResponse;
use pyo3::prelude::*;

#[pyclass]
pub struct Callback {
    #[allow(dead_code)]
    callback_function: Box<dyn Fn(&mut HttpContext) -> PyResult<HttpResponse> + Send + Sync>,
}

#[pymethods]
impl Callback {
    fn __call__(&self, context: &mut HttpContext) -> PyResult<HttpResponse> {
        (self.callback_function)(context)
    }
}

impl Callback {
    pub fn new(
        callback_function: Box<dyn Fn(&mut HttpContext) -> PyResult<HttpResponse> + Send + Sync>,
    ) -> Self {
        Callback { callback_function }
    }
}
