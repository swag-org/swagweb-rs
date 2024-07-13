use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot read request: {0}")]
    ReadFailed(#[from] io::Error),
    #[error("No CRLF at the end of line")]
    NoCRLF,
}

pub type Result = std::result::Result<String, Error>;

pub struct Reader(Box<dyn BufRead>);

impl Iterator for Reader {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let mut b = String::new();
        return match self.0.read_line(&mut b) {
            Ok(0) => None,
            Err(e) => Some(Err(e.into())),
            _ if !b.ends_with("\r\n") => {
                return Some(Err(Error::NoCRLF));
            }
            _ => {
                b.pop();
                b.pop();
                return Some(Ok(b));
            }
        };
    }
}

impl Reader {
    pub fn new(inner: Box<dyn BufRead>) -> Self {
        Self(inner)
    }

    pub fn inner(&mut self) -> &mut Box<dyn BufRead> {
        &mut self.0
    }
}
