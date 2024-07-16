use std::ops::Deref;

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyBytes, PyString, PyStringMethods, PyType},
    Bound, IntoPy, PyObject, PyResult, Python,
};

use crate::http::context::HttpContext;

pub trait Extractor {
    const TAG: usize;

    fn extract(&self, py: Python, request: &HttpContext) -> PyObject;
}

trait Based<E: Extractor> {
    fn based(self) -> (E, Base);
}

impl<E: Extractor> Based<E> for E {
    fn based(self) -> (E, Base) {
        (self, Base { tag: E::TAG })
    }
}

#[pyclass]
pub struct Base {
    pub tag: usize,
}

pub enum BodyKind {
    String,
    Bytes,
}

#[pyclass(extends = Base)]
pub struct Body(BodyKind);

#[pymethods]
impl Body {
    #[new]
    pub fn new(ty: &Bound<'_, PyType>) -> PyResult<(Self, Base)> {
        let str_ty = ty.py().get_type_bound::<PyString>();
        let byte_ty = ty.py().get_type_bound::<PyBytes>();
        let kind = if ty.is(&str_ty) {
            BodyKind::String
        } else if ty.is(&byte_ty) {
            BodyKind::Bytes
        } else {
            Err(PyValueError::new_err("Expected 'str' or 'bytes'"))?
        };
        Ok(Self(kind).based())
    }
}

impl Extractor for Body {
    fn extract(&self, py: Python, ctx: &HttpContext) -> PyObject {
        match self.0 {
            BodyKind::String => if ctx.request.content_valid_utf8 {
                ctx.request
                    .content
                    .as_ref()
                    .map(|x| unsafe { std::str::from_utf8_unchecked(&x) })
                    .unwrap()
            } else {
                "<invalid utf-8>"
            }
            .into_py(py),
            BodyKind::Bytes => ctx.request.content.as_ref().map(Deref::deref).into_py(py),
        }
    }

    const TAG: usize = 0;
}

#[pyclass(extends = Base)]
pub struct Headers;

#[pymethods]
impl Headers {
    #[new]
    pub fn new() -> (Self, Base) {
        (Self, Base { tag: 1 })
    }
}

impl Extractor for Headers {
    fn extract(&self, py: Python, ctx: &HttpContext) -> PyObject {
        ctx.request.headers.clone().into_py(py)
    }

    const TAG: usize = 1;
}

#[pyclass(extends = Base)]
pub struct Header(String);

#[pymethods]
impl Header {
    #[new]
    pub fn new(str: &Bound<'_, PyString>) -> (Self, Base) {
        Self(str.to_str().unwrap().to_owned()).based()
    }
}

impl Extractor for Header {
    fn extract(&self, py: Python, ctx: &HttpContext) -> PyObject {
        ctx.request.headers.get(&self.0).into_py(py)
    }

    const TAG: usize = 2;
}

#[pyclass(extends = Base)]
pub struct ContextVar(String);

#[pymethods]
impl ContextVar {
    #[new]
    pub fn new(str: &Bound<'_, PyString>) -> (Self, Base) {
        Self(str.to_str().unwrap().to_owned()).based()
    }
}

impl Extractor for ContextVar {
    const TAG: usize = 3;

    fn extract(&self, py: Python, request: &HttpContext) -> PyObject {
        request.vars.get(&self.0).into_py(py)
    }
}
