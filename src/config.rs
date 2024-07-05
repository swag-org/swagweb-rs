use std::net::ToSocketAddrs;

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(get_all, module = "swagweb_rs")]
#[derive(Clone)]
pub struct Config {
    pub listen_on: String,
}

#[pymethods]
impl Config {
    #[new]
    pub fn new(listen_on: String) -> PyResult<Self> {
        listen_on.to_socket_addrs()
            .map_err(|_| PyValueError::new_err(format!("invalid address \"{listen_on}\". Must have following format: IP:PORT")))?;
        Ok(Self { listen_on })
    }
}
