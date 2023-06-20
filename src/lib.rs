/*!
 * Library that is provided to Python
 */

use pyo3::prelude::*;

#[pyfunction]
pub fn test() -> String {
    "Hello from Rust".to_owned()
}
