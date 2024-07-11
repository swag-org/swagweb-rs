use pyo3::prelude::*;
use pyo3::types::{PyDict,PyAny};
use pyo3::wrap_pyfunction;

use crate::http::context::HttpContext;
use crate::http::response::HttpResponse;


pub type Handler = dyn Fn(HttpContext) -> HttpResponse;

pub trait MiddlewareBehaviour {
  fn execute(&self, context: HttpContext, call_next: Option<Box<dyn MiddlewareBehaviour>>) -> PyResult<dyn HttpResponse>;
}

pub struct HandlerMiddleware {
    context: HttpContext,
    handler: Handler,
}

impl MiddlewareBehaviour for HandlerMiddleware {
    fn execute(&self, context: HttpContext, _: Option<Box<dyn MiddlewareBehaviour>>) -> PyResult<HttpResponse> {
        PyResult::Ok((self.handler)(context)) 
    }
}

pub struct PyMiddleware {
    py_fn: PyObject
}

impl MiddlewareBehaviour for PyMiddleware {
    fn execute(&self, context: HttpContext, call_next: Option<Box<dyn MiddlewareBehaviour>>) -> PyResult<HttpResponse> {
        Python::with_gil(|py| {
            let py_func = self.py_fn.bind(py);
            let context_dict = PyDict::new(py);
            //context_dict.set_item("context", context)?;
            
            let result = py_func.call1((context_dict,))?;
            let http_response: HttpResponse = result.extract().expect();
            
            if let Some(next_middleware) = call_next {
                next_middleware.execute(context, None)
            } else {
                Ok(http_response)
            }
        })
    }
}
