use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;
mod interface;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;

use interface::config::__pyo3_get_function_run_config;

use interface::object::__pyo3_get_function_object_interface;

#[pyfunction]
fn say_hello() {
    println!("Say hello from Rust");
}

#[pymodule]
fn fliton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_number)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers)).unwrap();
    m.add_wrapped(wrap_pyfunction!(run_config)).unwrap();
    m.add_wrapped(wrap_pyfunction!(object_interface)).unwrap();

    Ok(())
}
