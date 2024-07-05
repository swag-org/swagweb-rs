use pyo3::{exceptions::PyValueError, prelude::*};
use crate::config::Config;

#[pyclass(get_all, module = "swagweb_rs")]
pub struct App {
    config: Config
}

#[pymethods]
impl App {
    #[new]
    fn new() -> PyResult<Self> {
        let config = Config::new(String::from("localhost:8000"))?;
        Ok(Self { config })
    }
    fn config(mut pyself: PyRefMut<'_, Self>, addr: String) -> PyResult<PyRefMut<'_, Self>> {
        addr.to_socket_addrs()
            .map_err(|_| PyValueError::new_err(format!("invalid address \"{addr}\"")))?;
        pyself.addr = addr;
        Ok(pyself)
    }

}
