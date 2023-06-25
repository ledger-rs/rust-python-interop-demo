/*!
 * Library that is provided to Python
 */

mod demo1;

use pyo3::prelude::*;

#[pyfunction]
pub fn test() -> String {
    "Hello from Rust".to_owned()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_interop(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}