use std::{
    io::{BufReader, Read},
    net::SocketAddrV4,
};

use body::HttpRequestBody;
use error::{Error, Result};
use headers::HttpHeaders;
use method::HttpMethod;
use py_addr::PySocketAddrV4;
use pyo3::prelude::*;

use crate::utils::request_reader;

pub mod body;
pub mod error;
pub mod headers;
pub mod method;
pub mod py_addr;

fn read_request_info(lines: &mut request_reader::Reader) -> Result<(HttpMethod, String, String)> {
    fn inner(line: String) -> Option<(HttpMethod, String, String)> {
        let mut split = line.split(" ");
        let method = split.next()?;
        let path = split.next()?.into();
        let ver = split.next()?.into();
        Some((HttpMethod::try_from(method)?, path, ver))
    }
    let line = lines.next().ok_or(Error::EmptyHttpRequest)??;
    inner(line).ok_or(Error::MalformedRequest)
}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub ip: PySocketAddrV4,
    pub ver: String,
    pub path: String,
    pub method: HttpMethod,
    pub headers: HttpHeaders,
    pub body: HttpRequestBody,
}

impl HttpRequest {
    pub fn from_reader(ip: SocketAddrV4, reader: Box<dyn Read>) -> Result<Self> {
        let mut reader = request_reader::Reader::new(Box::new(BufReader::new(reader)));
        let (method, path, ver) = read_request_info(&mut reader)?;
        let headers = HttpHeaders::from_reader(&mut reader)?;
        let body = HttpRequestBody::from_reader(&mut reader, &headers)?;
        Ok(HttpRequest {
            ip: ip.into(),
            ver,
            path,
            method,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        net::{Ipv4Addr, SocketAddrV4},
    };

    use pyo3::Python;

    use crate::{
        app::extractors::{Extractor, Headers},
        http::context::HttpContext,
    };

    use super::HttpRequest;

    #[test]
    fn head_request() {
        pyo3::prepare_freethreaded_python();
        let req = HttpRequest::from_reader(
            SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080),
            Box::new(
                &b"POST / HTTP/1.1\r
Host: localhost:8080\r
Content-Type: text/plain\r
\r
Hello, world!"[..],
            ),
        )
        .unwrap();
        Python::with_gil(move |py| {
            let body = Headers::new().0;
            let body = body.extract(
                py,
                &HttpContext {
                    request: req,
                    vars: HashMap::new(),
                },
            );
            println!("{body}");
        });
    }
}
