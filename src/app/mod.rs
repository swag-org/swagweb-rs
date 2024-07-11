use pyo3::prelude::*;

#[pyclass(module = "swagweb_rs")]
pub struct App {}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        App {}
    }
}
