use pyo3::prelude::*;

pub mod extractors;

#[pyclass(module = "swagweb_rs")]
pub struct App {}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        App {}
    }
}
