use std::{
    net::SocketAddrV4,
    ops::{Deref, DerefMut},
};

use pyo3::{prelude::*, types::PyString};

#[derive(Debug, Clone)]
pub struct PySocketAddrV4(SocketAddrV4);

impl From<SocketAddrV4> for PySocketAddrV4 {
    fn from(value: SocketAddrV4) -> Self {
        PySocketAddrV4(value)
    }
}

impl IntoPy<Py<PyAny>> for PySocketAddrV4 {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        PyString::new_bound(py, &format!("{}:{}", self.0.ip(), self.0.port()))
            .unbind()
            .into_any()
    }
}

impl Deref for PySocketAddrV4 {
    type Target = SocketAddrV4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PySocketAddrV4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
