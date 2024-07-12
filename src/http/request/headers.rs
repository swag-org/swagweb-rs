use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use pyo3::{
    exceptions::{PyValueError},
    prelude::*,
};

use crate::utils::RequestReader;

#[derive(Debug, Clone)]
pub struct HttpHeaders(HashMap<String, String>);

impl IntoPy<PyObject> for HttpHeaders {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.into_py(py)
    }
}

impl Deref for HttpHeaders {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HttpHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl HttpHeaders {
    pub fn from_reader(reader: &mut RequestReader) -> PyResult<Self> {
        let mut map = HashMap::new();
        loop {
            let line = reader
                .next()
                .ok_or(PyValueError::new_err("Request unexpectedly ended"))??;
            if line == "" {
                break;
            }
            let semi = line
                .find(": ")
                .ok_or(PyValueError::new_err("Invalid header"))?;
            let header_name = line[..semi].to_owned();
            let header_value = line[semi + 2..].to_owned();
            map.insert(header_name, header_value);
        }
        Ok(Self(map))
    }
}
