use app::App;
use pyo3::prelude::*;

mod app;
mod http;

#[pymodule]
fn swagweb_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<App>()?;
    Ok(())
}
