use std::{fmt::Debug, ops::Deref};

use hyper::HeaderMap;
use pyo3::{
    types::{PyDict, PyDictMethods},
    IntoPy, PyObject,
};

pub struct Headers(pub(crate) HeaderMap);

impl Deref for Headers {
    type Target = HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Headers(")?;
        self.0.fmt(f)?;
        f.write_str(")")
    }
}

impl IntoPy<PyObject> for Headers {
    fn into_py(self, py: pyo3::Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);
        self.0
            .into_iter()
            .filter_map(|(k, v)| k.map(|k| (k, v)))
            .for_each(|(k, v)| {
                if let Ok(v) = v.to_str() {
                    dict.set_item(
                        <&str as IntoPy<PyObject>>::into_py(k.as_str(), py),
                        <&str as IntoPy<PyObject>>::into_py(v, py),
                    )
                    .unwrap()
                }
            });
        dict.into_any().unbind()
    }
}
