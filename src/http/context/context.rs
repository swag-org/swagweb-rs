use std::net::SocketAddrV4;
use pyo3::pyclass;
use crate::http::request::HttpRequest;

#[pyclass(get_all)]
#[derive(Clone)]
pub struct HttpContext {
    request: HttpRequest,
    path: String,
}
