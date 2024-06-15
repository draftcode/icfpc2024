use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from icfpc2024!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
