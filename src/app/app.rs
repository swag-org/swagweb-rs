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
    fn config(mut pyself: PyRefMut<'_, Self>, config: Config) -> PyRefMut<'_, Self> {
        pyself.config = config;
        pyself
    }

}
