mod callback;
mod pythonize;

use pyo3::prelude::*;
use std::sync::Arc;

use pyo3::PyAny;

use self::callback::Callback;
use crate::http::context::HttpContext;
use crate::http::response::HttpResponse;

pub trait MiddlewareBehaviour: Send + Sync {
    fn execute(&self, context: &mut HttpContext) -> PyResult<HttpResponse>;
    fn get_next(&self) -> Option<NextMiddleware>;
    fn set_next(&mut self, next: Option<NextMiddleware>);
}

#[pyclass]
#[derive(Clone)]
pub struct NextMiddleware {
    middleware: Box<MiddlewareEnum>,
}

#[pyclass]
#[derive(Clone)]
pub struct HandlerMiddleware {
    handler: Arc<dyn Fn(&mut HttpContext) -> HttpResponse + Send + Sync>,
}

impl MiddlewareBehaviour for HandlerMiddleware {
    fn execute(&self, context: &mut HttpContext) -> PyResult<HttpResponse> {
        Ok((self.handler)(context))
    }
    fn get_next(&self) -> Option<NextMiddleware> {
        None
    }
    fn set_next(&mut self, _: Option<NextMiddleware>) {}
}

#[pyclass]
#[derive(Clone)]
pub struct PyMiddleware {
    py_fn: Py<PyAny>,
    next: Option<NextMiddleware>,
}

#[pymethods]
impl PyMiddleware {
    #[new]
    pub fn new(py_fn: Py<PyAny>, next: Option<NextMiddleware>) -> Self {
        PyMiddleware { py_fn, next }
    }
}

impl MiddlewareBehaviour for PyMiddleware {
    fn get_next(&self) -> Option<NextMiddleware> {
        self.next.clone()
    }
    fn set_next(&mut self, next: Option<NextMiddleware>) {
        self.next = next;
    }
    fn execute(&self, _context: &mut HttpContext) -> PyResult<HttpResponse> {
        let cb: Box<dyn Fn(&mut HttpContext) -> PyResult<HttpResponse> + Send + Sync> =
            match self.get_next() {
                None => Box::new(|_context: &mut HttpContext| {
                    Err(pyo3::exceptions::PyValueError::new_err(
                        "Can't call next middleware, because it is None",
                    ))
                }),
                Some(next) => match *next.middleware {
                    MiddlewareEnum::Handler(middleware) => {
                        Box::new(move |context: &mut HttpContext| middleware.execute(context))
                    }
                    MiddlewareEnum::PyMiddleware(middleware) => {
                        Box::new(move |context: &mut HttpContext| middleware.execute(context))
                    }
                },
            };

        Python::with_gil(|py| {
            let callback = Callback::new(cb);
            let py_func = self.py_fn.bind(py);
            let result = py_func.call1((callback.into_py(py),))?;
            let http_response: HttpResponse = result.extract()?;
            Ok(http_response)
        })
    }
}

#[derive(Clone)]
pub enum MiddlewareEnum {
    Handler(HandlerMiddleware),
    PyMiddleware(PyMiddleware),
}
