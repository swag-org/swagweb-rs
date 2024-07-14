use pyo3::{pyclass, pymethods};

#[pyclass(get_all)]
#[derive(Clone)]
pub struct HttpResponse {
    pub status: u16,
}

#[pymethods]
impl HttpResponse {
    #[new]
    pub fn new(status: u16) -> Self {
        HttpResponse { status }
    }
}
