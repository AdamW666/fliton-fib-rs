use super::fib_number::fibonacci_number;
use pyo3::prelude::pyfunction;
use std::vec::Vec;

#[pyfunction]
pub fn fibonacci_numbers(numbers: Vec<i32>) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for n in numbers.iter() {
        vec.push(fibonacci_number(*n));
    }

    return vec;
}
