use std::net::ToSocketAddrs;

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(get_all, module = "swagweb_rs")]
pub struct Config {
    pub addr: String,
}

#[pymethods]
impl Config {
    #[new]
    fn new(addr: String) -> PyResult<Self> {
        addr.to_socket_addrs()
            .map_err(|_| PyValueError::new_err(format!("invalid address \"{addr}\"")))?;
        Ok(Self { addr })
    }
}
