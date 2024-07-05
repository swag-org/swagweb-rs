use config::Config;
use app::App;
use pyo3::prelude::*;
mod config;
mod app;

#[pymodule]
fn swagweb_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Config>()?;
    m.add_class::<App>()?;
    Ok(())
}
