/*!
 * Library that is provided to Python
 */

mod demo1;

use pyo3::prelude::*;

#[pyclass]
pub struct Model {
    name: String,
}

#[pymethods]
impl Model {
    #[new]
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[pyfunction]
pub fn test() -> String {
    "Hello from Rust".to_owned()
}

#[pyfunction]
pub fn use_shared_object() -> Vec<Model> {
    let mut result = vec![];

    result.push(Model::new("first".to_owned()));
    result.push(Model::new("second".to_owned()));

    println!("Objects: {}", result.len());

    result
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_interop(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Model>()?;

    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_function(wrap_pyfunction!(use_shared_object, m)?)?;
    Ok(())
}