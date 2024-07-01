use common::compiler::program;
use common::eval::eval;
use common::expr::{Expr, Token};
use common::planar;
use num_bigint::BigInt;
use pyo3::exceptions::PyValueError;
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

#[pyfunction]
fn onestep_3d(program: String, a: i32, b: i32, turn: usize) -> PyResult<(String, Option<i32>, i32)> {
    let state = planar::State::new(&program, a.into(), b.into());
    if state.is_err() {
        return Err(PyErr::new::<PyValueError, _>(format!(
            "failed to load program"
        )));
    }
    let mut state = state.unwrap();

    if state.resolve_label().is_err() {
        return Err(PyErr::new::<PyValueError, _>(format!(
            "failed to resolve label"
        )));
    }

    for t in 0..turn {
        if state.onestep().is_err() {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "eval failed at turn {}:\n{}",
                t, state.board
            )));
        }
    }

    let i32_max: BigInt = i32::MAX.into();
    let i32_min: BigInt = i32::MIN.into();
    let score = state.score();
    let res: Option<i32> = state
        .output
        .map(|v| v.min(i32_max).max(i32_min).try_into().unwrap());
    Ok((format!("{}", state.board), res, score))
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_message, m)?)?;
    m.add_function(wrap_pyfunction!(decode_message, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_message, m)?)?;
    m.add_function(wrap_pyfunction!(onestep_3d, m)?)?;
    Ok(())
}
