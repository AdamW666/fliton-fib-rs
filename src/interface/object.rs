use super::config::run_config;
use pyo3::exceptions::PyLookupError;
use pyo3::prelude::{pyfunction, PyResult, Python};
use pyo3::types::{PyAny, PyDict};

fn extract_data<'a>(
    input_object: &'a PyAny,
    attribute: &'a str,
    config_dict: &'a PyDict,
) -> &'a PyDict {
    match input_object.getattr(attribute) {
        Ok(data) => {
            config_dict.set_item(attribute, data).unwrap();
        }
        Err(_) => Err(PyLookupError::new_err(format!(
            "Attribute {} is missing",
            attribute
        )))
        .unwrap(),
    }

    return config_dict;
}

fn overwrite_data<'a>(
    input_object: &'a PyAny,
    attribute: &'a str,
    config_dict: &'a PyDict,
    dict_item: &'a str,
) -> () {
    let value = config_dict.get_item(dict_item).unwrap();
    input_object.setattr(attribute, value).unwrap();
}

#[pyfunction]
pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let mut config_dict: &PyDict = PyDict::new(py);

    config_dict = extract_data(input_object, "number", config_dict);
    config_dict = extract_data(input_object, "numbers", config_dict);

    let output_dict: &PyDict = run_config(config_dict).unwrap();

    overwrite_data(input_object, "number_results", output_dict, "NUMBER RESULT");
    overwrite_data(
        input_object,
        "numbers_results",
        output_dict,
        "NUMBERS RESULT",
    );

    return Ok(input_object);
}
