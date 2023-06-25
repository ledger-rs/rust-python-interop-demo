use pyo3::prelude::*;

#[pyclass]
struct MyStruct {
    x: i32,
    y: i32,
    z: String
}

#[pymethods]
impl MyStruct {
    #[new]
    fn new(x: i32, y: i32, z: String) -> Self {
        MyStruct { x, y, z }
    }

    #[getter]
    fn x(&self) -> PyResult<i32> {
        Ok(self.x)
    }

    #[getter]
    fn y(&self) -> PyResult<i32> {
        Ok(self.y)
    }
}

#[pyfunction]
fn add(a: &MyStruct, b: &MyStruct) -> MyStruct {
    MyStruct {
        x: a.x + b.x,
        y: a.y + b.y,
        z: "complete".to_string(),
    }
}

#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyStruct>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    
}