use std::{collections::HashMap, mem::forget, path::PathBuf};

use futures_util::{future, Stream, StreamExt};
use http::{header::CONTENT_LENGTH, request::Parts};
use http_body_util::BodyStream;
use hyper::{
    body::{Bytes, Incoming},
    header::CONTENT_TYPE,
};
use multer::Multipart;
use pyo3::{ffi::PyObject, pyclass, types::PyAnyMethods, Py, PyAny, Python};
use tempfile::tempdir;
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Transport failed: {0}")]
    TransportFail(#[from] hyper::Error),
    #[error("Multipart failed: {0}")]
    MultipartFail(#[from] multer::Error),
    #[error("Invalid multipart data: {0}")]
    MalformedMultipart(String),
    #[error("~ For future")]
    ConstraintViolation,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
#[pyclass(get_all)]
pub struct Request {
    pub uri: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub fields: Option<HashMap<String, String>>,
    pub files: Option<HashMap<String, PathBuf>>,
}

impl Request {
    pub async fn parse(request: hyper::Request<Incoming>) -> Result<Self> {
        let (parts, body) = request.into_parts();

        let uri = parts.uri.to_string();
        let method = parts.method.to_string();
        let headers = convert_headers(&parts);

        let boundary = extract_boundary(&parts);
        let stream = body_to_stream(body);
        if let Some(boundary) = boundary {
            let mut fields = HashMap::new();
            let mut files = HashMap::new();
            let dir = tempdir().unwrap();
            let mut m = Multipart::new(stream, boundary);
            while let Some(mut field) = m.next_field().await? {
                if let Some(fname) = field.file_name() {
                    let fname = fname.to_string();
                    if fname.contains('/') || fname.contains('\\') {
                        Err(Error::MalformedMultipart("Filename contains slashes, so it might be misinterpreted by path resolver".into()))?;
                    }
                    let path = dir.path().join(&fname);
                    let mut file = File::create_new(&path).await.unwrap();
                    while let Some(chunk) = field.chunk().await? {
                        file.write(&chunk).await.unwrap();
                    }
                    files.insert(fname, path);
                } else {
                    if let Some(name) = field.name() {
                        let name = name.into();
                        let value = field.bytes().await?;
                        fields.insert(
                            name,
                            String::from_utf8(value.to_vec()).map_err(|_| {
                                Error::MalformedMultipart("Field value is not valid utf8".into())
                            })?,
                        );
                    }
                }
            }
            forget(dir);
            Ok(Request {
                uri,
                method,
                headers,
                content: None,
                fields: Some(fields),
                files: Some(files),
            })
        } else {
            Ok(Request {
                uri,
                method,
                headers,
                content: Some(read_plain(&parts, stream).await?),
                fields: None,
                files: None,
            })
        }
    }
}

fn convert_headers(parts: &Parts) -> HashMap<String, String> {
    let mut r = HashMap::new();
    for (name, value) in &parts.headers {
        if let Ok(value) = value.to_str() {
            r.insert(name.to_string(), value.into());
        }
    }
    r
}

fn extract_boundary(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get(CONTENT_TYPE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| multer::parse_boundary(c).ok())
}

fn body_to_stream(body: Incoming) -> impl Stream<Item = hyper::Result<Bytes>> {
    BodyStream::new(body).filter_map(|x| async move { x.map(|f| f.into_data().ok()).transpose() })
}

async fn read_plain(
    parts: &Parts,
    stream: impl Stream<Item = hyper::Result<Bytes>>,
) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(
        parts
            .headers
            .get(CONTENT_LENGTH)
            .and_then(|c| c.to_str().ok())
            .and_then(|c| c.parse::<usize>().ok())
            .unwrap_or(0),
    );
    let mut fail = None::<hyper::Error>;
    stream
        .for_each(|item| {
            if fail.is_none() {
                match item {
                    Ok(b) => bytes.extend(b),
                    Err(e) => fail = Some(e),
                }
            }
            future::ready(())
        })
        .await;
    if let Some(fail) = fail {
        Err(fail)?
    }
    Ok(bytes)
}
