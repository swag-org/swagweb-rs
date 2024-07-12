use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines, Read},
    net::SocketAddrV4,
};

use method::HttpMethod;
use py_socket_addr_v4::PySocketAddrV4;
use pyo3::{
    exceptions::{PyIOError, PyValueError},
    prelude::*,
};

pub mod method;
pub mod py_socket_addr_v4;

fn read_request_info(
    lines: &mut Lines<BufReader<impl Read>>,
) -> PyResult<(HttpMethod, String, String)> {
    fn inner(line: String) -> Option<(HttpMethod, String, String)> {
        let mut split = line.split(" ");
        let method = split.next()?;
        let path = split.next()?.into();
        let ver = split.next()?.into();
        Some((HttpMethod::try_from(method)?, path, ver))
    }
    inner(
        lines
            .next()
            .ok_or(PyValueError::new_err("Empty http request"))?
            .or_else(|e| Err(PyIOError::new_err(format!("Cannot read http request: {e}"))))?,
    )
    .ok_or(PyValueError::new_err("Malformed http request"))
}

fn read_request_headers(
    lines: &mut Lines<BufReader<impl Read>>,
) -> PyResult<HashMap<String, String>> {
    let mut headers = HashMap::new();
    while let Some(i) = lines.next() {
        let i = i.or_else(|e| Err(PyIOError::new_err(format!("Cannot read http request: {e}"))))?;
        if i.is_empty() {
            break;
        }
        if let Some(find) = i.find(": ") {
            headers.insert(i[..find].to_owned(), i[find + 2..].to_owned());
        }
    }
    Ok(headers)
}

fn read_request_content(
    lines: Lines<BufReader<impl Read>>,
    headers: &HashMap<String, String>,
) -> PyResult<String> {
    let mut content = String::with_capacity(
        headers
            .get("Content-Length")
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(0),
    );
    for i in lines {
        content.push_str(
            &i.or_else(|e| Err(PyIOError::new_err(format!("Cannot read http request: {e}"))))?,
        );
    }
    Ok(content)
}

#[pyclass(get_all)]
#[derive(Debug)]
pub struct HttpRequest {
    ip: PySocketAddrV4,
    ver: String,
    path: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    content: String,
}

impl HttpRequest {
    pub fn from_reader(ip: SocketAddrV4, reader: impl Read) -> PyResult<Self> {
        let mut lines = BufReader::new(reader).lines();
        let (method, path, ver) = read_request_info(&mut lines)?;
        let headers = read_request_headers(&mut lines)?;
        let content = read_request_content(lines, &headers)?;
        Ok(HttpRequest {
            ip: ip.into(),
            ver,
            path,
            method,
            headers,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use super::HttpRequest;

    #[test]
    fn head_request() {
        println!(
            "{:?}",
            HttpRequest::from_reader(
                SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080),
                &b"POST / HTTP/1.1
Host: localhost:8080

Hello, world!"[..]
            )
            .unwrap()
        );
    }
}
