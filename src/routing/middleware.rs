use hyper::body::Incoming;
use hyper::Request;
use pyo3::prelude::*;
use std::sync::Arc;

use pyo3::PyAny;

use crate::http;
use crate::http::context::HttpContext;
use crate::http::response::HttpResponse;

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
    fn new(
        callback_function: Box<dyn Fn(&mut HttpContext) -> PyResult<HttpResponse> + Send + Sync>,
    ) -> Self {
        Callback { callback_function }
    }
}

pub trait MiddlewareBehaviour: Send + Sync {
    fn execute(&self, context: &mut HttpContext) -> PyResult<HttpResponse>;
    fn get_next(&self) -> Option<NextMiddleware>;
    fn set_next(&mut self, next: Option<NextMiddleware>);
}

#[derive(Clone)]
pub enum MiddlewareEnum {
    Handler(HandlerMiddleware),
    PyMiddleware(PyMiddleware),
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
        //let cb = match self.get_next() {
        //    None => Box::new(|context: &mut HttpContext| Err(pyo3::exceptions::PyValueError::new_err("Can't call next middleware, because it is None"))),
        //    Some(next) => {
        //        match *next.middleware {
        //            MiddlewareEnum::Handler(middleware) => Box::new(|context: &mut HttpContext| middleware.execute(context)),
        //            MiddlewareEnum::PyMiddleware(middleware) => Box::new(|context: &mut HttpContext| middleware.execute(context))
        //        }
        //    }
        //};
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

//impl<'source> FromPyObject<'source> for NextMiddleware {
//    fn extract(ob: &'source PyAny) -> PyResult<Self> {
//        let dict = ob.downcast::<PyDict>()?;
//        let middleware = dict.get_item("middleware")
//            .ok_or_else(|| pyo3::exceptions::PyTypeError::new_err("Missing middleware field"))?;
//        let middleware: &PyAny = middleware.extract()?;
//        let middleware = middleware.extract::<Box<MiddlewareEnum>>()?;
//        Ok(NextMiddleware { middleware })
//    }
//}
