use pyo3::exceptions::PyTypeError;
use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyDict, PyList};

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;

fn process_numbers(input_numbers: Vec<Vec<i32>>) -> Vec<Vec<u64>> {
    let mut buffer: Vec<Vec<u64>> = Vec::new();
    for i in input_numbers {
        buffer.push(fibonacci_numbers(i));
    }

    return buffer;
}

#[pyfunction]
pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict> {
    match config.get_item("number") {
        Some(data) => match data.downcast::<PyList>() {
            Ok(raw_data) => {
                let processed_results: Vec<i32> = raw_data.extract::<Vec<i32>>().unwrap();
                let fib_numbers: Vec<u64> = processed_results
                    .iter()
                    .map(|x| fibonacci_number(*x))
                    .collect();
                config.set_item("NUMBER RESULT", fib_numbers).unwrap();
            }
            Err(_) => Err(PyTypeError::new_err(
                "Parameter 'number' should have the type of List[int]",
            ))
            .unwrap(),
        },
        None => {
            println!("Parameter 'number' can't be found in the config file")
        }
    }

    match config.get_item("numbers") {
        Some(data) => match data.downcast::<PyList>() {
            Ok(raw_data) => {
                let processed_results: Vec<Vec<i32>> = raw_data.extract::<Vec<Vec<i32>>>().unwrap();
                config
                    .set_item("NUMBERS RESULT", process_numbers(processed_results))
                    .unwrap();
            }
            Err(_) => Err(PyTypeError::new_err(
                "Parameter 'numbers' should have the type of List[List[int]]",
            ))
            .unwrap(),
        },
        None => {
            println!("Parameter 'numbers' can't be found in the config file")
        }
    }

    return Ok(config);
}
