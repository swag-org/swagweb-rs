use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use pyo3::prelude::*;

use crate::utils::request_reader;

use super::error::{Error, Result};

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
    pub fn from_reader(reader: &mut request_reader::Reader) -> Result<Self> {
        let mut map = HashMap::new();
        loop {
            let line = reader.next().ok_or(Error::RequestUnexpectedlyEnded)??;
            if line == "" {
                break;
            }
            let semi = line
                .find(": ")
                .ok_or(Error::InvalidHeader(line.clone(), "<no colon>".into()))?;
            let header_name = line[..semi].to_owned();
            let header_value = line[semi + 2..].to_owned();
            map.insert(header_name, header_value);
        }
        Ok(Self(map))
    }
}
