use std::{collections::HashMap, path::PathBuf};

use getset::Getters;
use pyo3::{pyclass, IntoPy, PyObject};

#[derive(Debug)]
pub enum Content {
    Plain(String),
    Multipart(Multipart),
}

impl IntoPy<PyObject> for Content {
    fn into_py(self, py: pyo3::Python<'_>) -> PyObject {
        match self {
            Content::Plain(t) => t.into_py(py),
            Content::Multipart(m) => m.into_py(py),
        }
    }
}

#[pyclass]
#[derive(Debug, Getters)]
pub struct Multipart {
    #[pyo3(get)]
    #[getset(get = "pub")]
    fields: HashMap<String, String>,
    #[pyo3(get)]
    #[getset(get = "pub")]
    files: Vec<(String, PathBuf)>,
    #[getset(get = "pub")]
    files_dir: PathBuf,
}

impl Multipart {
    pub(crate) fn new(
        fields: HashMap<String, String>,
        files: Vec<(String, PathBuf)>,
        files_dir: PathBuf,
    ) -> Self {
        Self {
            fields,
            files,
            files_dir,
        }
    }
}
