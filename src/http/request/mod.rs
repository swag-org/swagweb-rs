use std::collections::HashMap;

use futures_util::{future, StreamExt};
use http_body_util::{BodyExt, BodyStream};
use hyper::{
    body::Incoming,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use multer::Multipart;
use pyo3::pyclass;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
#[pyclass(get_all)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub text: Option<String>,
    pub fields: Option<HashMap<String, String>>,
}

pub async fn parse(req: hyper::Request<Incoming>) -> Result<Request> {
    let boundary = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| multer::parse_boundary(c).ok());

    let (parts, body) = req.into_parts();

    let body_stream = BodyStream::new(body)
        .filter_map(|x| async move { x.map(|f| f.into_data().ok()).transpose() });

    let mut text = None;
    let mut fields = None;
    if let Some(boundary) = boundary {
        let mut fields_map = HashMap::new();
        let mut m = Multipart::new(body_stream, boundary);
        while let Ok(Some(field)) = m.next_field().await {
            if field.file_name().is_some() {
                // Not support files yet
                continue;
            }
            if let Some(name) = field.name() {
                let name = name.to_owned();
                if let Ok(value) = field.bytes().await {
                    if let Ok(value) = String::from_utf8(value.to_vec()) {
                        fields_map.insert(name.into(), value);
                    }
                }
            }
        }
        fields = Some(fields_map);
    } else {
        text = Some(String::with_capacity(
            parts
                .headers
                .get(CONTENT_LENGTH)
                .and_then(|x| std::str::from_utf8(x.as_bytes()).ok())
                .and_then(|x| x.parse().ok())
                .unwrap_or(0),
        ));
        body_stream
            .filter_map(|x| async move { x.ok().and_then(|x| String::from_utf8(x.to_vec()).ok()) })
            .for_each(|x| {
                text.as_mut().unwrap().push_str(&x);
                future::ready(())
            })
            .await;
    }

    let mut headers = HashMap::new();
    for (k, v) in parts.headers.into_iter() {
        if let (Some(k), Ok(v)) = (k, String::from_utf8(v.as_bytes().to_owned())) {
            headers.insert(k.to_string(), v);
        }
    }

    Ok(Request {
        method: parts.method.to_string(),
        uri: parts.uri.to_string(),
        version: format!("{:?}", parts.version),
        headers,
        text,
        fields,
    })
}
