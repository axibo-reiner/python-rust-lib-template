// use pyo3::prelude::*;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn rust_python_module(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::IntoPyDict;

#[pyclass]
struct MyClass {
    #[pyo3(get, set)]
    name: String,
    value: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(name: String, value: i32) -> Self {
        MyClass { name, value }
    }

    fn add(&self, other: i32) -> i32 {
        self.value + other
    }

    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[setter]
    fn set_name(&mut self, value: String) -> PyResult<()> {
        self.name = value;
        Ok(())
    }
}

/// A function that initializes the Python module.
#[pymodule]
fn py_class_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}

