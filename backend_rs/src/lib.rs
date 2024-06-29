use common::compiler::program;
use common::eval::eval;
use common::expr::{Expr, Token};
use common::planar;
use num_bigint::BigInt;
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
fn onestep_3d(program: String, a: i32, b: i32, turn: usize) -> PyResult<(String, Option<i32>)> {
    let mut state: planar::State = Default::default();

    for l in program.lines() {
        let mut row = vec![];
        for c in l.split_whitespace() {
            if let Ok(cell) = c.parse::<planar::Cell>() {
                row.push(cell);
            }
        }
        state.board.0.push(row);
    }

    for _ in 0..turn {
        if state.onestep().is_err() {
            return Ok(("one step evaluaton is failed.".to_owned(), None));
        }
    }

    let i32_max: BigInt = i32::MAX.into();
    let i32_min: BigInt = i32::MIN.into();
    let res: Option<i32> = state
        .output
        .map(|v| v.min(i32_max).max(i32_min).try_into().unwrap());
    Ok((format!("{}", state.board), res))
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_message, m)?)?;
    m.add_function(wrap_pyfunction!(decode_message, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_message, m)?)?;
    Ok(())
}
