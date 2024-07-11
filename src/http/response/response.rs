use pyo3::pyclass;

pub trait HttpResponse {}


#[pyclass(get_all)]
pub struct PlainTextResponse {
    
}
