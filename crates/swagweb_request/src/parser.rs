use std::collections::HashMap;

use futures_util::{future, StreamExt};
use http_body_util::BodyStream;
use hyper::{
    body::Incoming,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use tempfile::tempdir;
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    content::{Content, Multipart},
    headers::Headers,
    Request,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(#[from] hyper::Error),
    #[error("Multipart error: {0}")]
    Multipart(#[from] multer::Error),
    #[error("Field \"{0}\" value is not valid UTF-8")]
    FieldInvalidUtf8(String),
    #[error("Request body is not valid UTF-8")]
    PlainInvalidUtf8,
}

pub async fn parse(request: hyper::Request<Incoming>) -> Result<Request, Error> {
    let (parts, body) = request.into_parts();
    let path = parts.uri.path().into();
    let method = parts.method.to_string();
    let headers = Headers(parts.headers);
    let content = extract_incoming(&headers, body).await?;
    Ok(Request::new(path, method, headers, content))
}

fn extract_boundary(headers: &Headers) -> Option<String> {
    headers
        .0
        .get(CONTENT_TYPE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| multer::parse_boundary(c).ok())
}

fn extract_content_length(headers: &Headers) -> usize {
    headers
        .0
        .get(CONTENT_LENGTH)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| c.parse::<usize>().ok())
        .unwrap_or(0)
}

async fn extract_incoming(headers: &Headers, body: Incoming) -> Result<Content, Error> {
    let stream = BodyStream::new(body)
        .filter_map(|x| async move { x.map(|f| f.into_data().ok()).transpose() });
    if let Some(boundary) = extract_boundary(headers) {
        let mut fields = HashMap::new();
        let mut files = Vec::new();
        let files_dir = tempdir().unwrap().into_path();
        let mut files_index = 0;
        let mut m = multer::Multipart::new(stream, boundary);
        while let Some(mut field) = m.next_field().await? {
            if let Some(filename) = field.file_name() {
                files_index += 1;
                let filename = filename.into();
                let filepath = files_dir.join(files_index.to_string());
                let mut file = File::create_new(&filepath).await.unwrap();
                while let Some(chunk) = field.chunk().await? {
                    file.write(&chunk).await.unwrap();
                }
                files.push((filename, filepath))
            } else if let Some(name) = field.name() {
                let name = name.into();
                if let Ok(value) = String::from_utf8(field.bytes().await?.to_vec()) {
                    fields.insert(name, value);
                } else {
                    Err(Error::FieldInvalidUtf8(name))?
                }
            }
        }
        Ok(Content::Multipart(Multipart::new(fields, files, files_dir)))
    } else {
        let mut plain = Vec::with_capacity(extract_content_length(headers));
        let mut err = None::<hyper::Error>;
        stream
            .for_each(|x| {
                if err.is_none() {
                    match x {
                        Ok(x) => plain.extend(x),
                        Err(x) => err = Some(x),
                    }
                }
                future::ready(())
            })
            .await;
        if let Some(err) = err {
            Err(err)?
        }
        Ok(Content::Plain(
            String::from_utf8(plain).map_err(|_| Error::PlainInvalidUtf8)?,
        ))
    }
}
