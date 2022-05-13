use pyo3::prelude::pyfunction;

#[pyfunction]
pub fn fibonacci_number(n: i32) -> u64 {
    if n < 0 {
        panic!("Fibonacci series can't start from negative position")
    }

    match n {
        0 => panic!("Argument should be greater than 0"),
        1 | 2 => 1,
        _ => fibonacci_number(n - 1) + fibonacci_number(n - 2),
    }
}
