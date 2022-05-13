use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;

#[pyfunction]
fn say_hello() {
    println!("Say hello from Rust");
}

#[pymodule]
fn fliton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));

    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::{pyfunction, pymodule};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
