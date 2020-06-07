use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyBool;
use pyo3::create_exception;
use pyo3::types::IntoPyDict;
use pyo3::exceptions::BaseException;

use expression_parser::{ExpressionFile, Variables};


create_exception!(expression_parser_python, ParseError, BaseException);
create_exception!(expression_parser_python, EvalError, BaseException);

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn parse_and_eval<'a>(text: &str) -> PyResult<Vec<u8>> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let value = match ExpressionFile::parse(text){
        Ok(x) => x,
        Err(e) => {
            // let error_type = py.get_type::<ParseError>();
            // let ctx = [("ParseError", error_type)].into_py_dict(py);
            // return Err(PyErr::from_instance(&ctx))
            let error = Py::from(py.get_type::<ParseError>());
            return Err(PyErr::from_type(error, format!("{}", e)))
        }

    };
    let value = match ExpressionFile::eval(value, &mut Variables::default()){
        Ok(x) => x,
        Err(e) => {
            // let error_type = py.get_type::<EvalError>();
            // let ctx = [("EvalError", error_type)].into_py_dict(py);
            // return Err(PyErr::from_instance(&ctx))
            let error = Py::from(py.get_type::<EvalError>());
            return Err(PyErr::from_type(error, format!("{}", e)))
        }

    };

    let data = serde_pickle::to_vec(&value, true).unwrap();

    Ok(data)

}

/// A Python module implemented in Rust.
#[pymodule]
fn expression_parser_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(parse_and_eval))?;

    Ok(())
}