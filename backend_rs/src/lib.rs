use common::eval::eval;
use common::expr::{Expr, Token};
use pyo3::prelude::*;

#[pyfunction]
fn encode_message(input: String) -> PyResult<String> {
    Ok(Token::String(input).encoded().to_string())
}

#[pyfunction]
fn decode_message(input: String) -> PyResult<String> {
    if let Ok(Token::String(s)) = input.parse() {
        return Ok(s);
    }
    Ok("".to_string())
}

#[pyfunction]
fn evaluate_message(input: String) -> PyResult<String> {
    if let Ok(expr) = input.parse() {
        if let Ok(expr) = eval(&expr) {
            if let Expr::String(s) = expr {
                return Ok(s.to_string());
            }
        }
    }
    Ok("".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_message, m)?)?;
    m.add_function(wrap_pyfunction!(decode_message, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_message, m)?)?;
    Ok(())
}
