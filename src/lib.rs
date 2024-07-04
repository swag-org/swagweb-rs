use config::Config;
use pyo3::prelude::*;
mod config;

#[pymodule]
fn swagweb_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Config>()?;
    Ok(())
}
