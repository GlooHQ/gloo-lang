use baml_types::{BamlValueWithMeta, ResponseCheck};
use pyo3::prelude::{pymethods, PyResult};
use pyo3::types::{PyAnyMethods, PyDict, PyModule, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, IntoPyObjectExt, PyAny, PyObject, Python};

use crate::errors::BamlError;

use super::{BamlAudioPy, BamlImagePy};

crate::lang_wrapper!(FunctionResult, baml_runtime::FunctionResult);

#[pymethods]
impl FunctionResult {
    fn __str__(&self) -> String {
        format!("{:#}", self.inner)
    }

    fn is_ok(&self) -> bool {
        self.inner.result_with_constraints_content().is_ok()
    }

    /// This is a debug function that returns the internal representation of the response
    /// This is not to be relied upon and is subject to change
    fn unstable_internal_repr(&self) -> String {
        serde_json::json!(self.inner.llm_response()).to_string()
    }

    // Cast the parsed value to a specific type
    // the module is the module that the type is defined in
    fn cast_to(
        &self,
        py: Python<'_>,
        enum_module: Bound<'_, PyModule>,
        cls_module: Bound<'_, PyModule>,
    ) -> PyResult<PyObject> {
        let parsed = self
            .inner
            .result_with_constraints_content()
            .map_err(BamlError::from_anyhow)?;

        let parsed = pythonize_strict(py, parsed.clone(), &enum_module, &cls_module)?;

        Ok(parsed)
    }
}

fn pythonize_checks<'a>(
    py: Python<'a>,
    types_module: &Bound<'_, PyModule>,
    checks: &[ResponseCheck],
) -> PyResult<Bound<'a, PyDict>> {
    let dict = PyDict::new(py);
    let check_class = types_module.getattr("Check")?;
    let check_class = check_class.downcast::<PyType>()?;
    checks.iter().try_for_each(
        |ResponseCheck {
             name,
             expression,
             status,
         }| {
            // Construct the Check.
            let check_properties_dict = pyo3::types::PyDict::new(py);
            check_properties_dict.set_item("name", name)?;
            check_properties_dict.set_item("expression", expression)?;
            check_properties_dict.set_item("status", status)?;
            let check_instance =
                check_class.call_method("model_validate", (check_properties_dict,), None)?;
            dict.set_item(name, check_instance)?;
            PyResult::Ok(())
        },
    )?;
    Ok(dict)
}

fn pythonize_strict(
    py: Python<'_>,
    parsed: BamlValueWithMeta<Vec<ResponseCheck>>,
    enum_module: &Bound<'_, PyModule>,
    cls_module: &Bound<'_, PyModule>,
) -> PyResult<PyObject> {
    let meta = parsed.meta().clone();
    let py_value_without_constraints = match parsed {
        BamlValueWithMeta::String(val, _) => val.into_py_any(py),
        BamlValueWithMeta::Int(val, _) => val.into_py_any(py),
        BamlValueWithMeta::Float(val, _) => val.into_py_any(py),
        BamlValueWithMeta::Bool(val, _) => val.into_py_any(py),
        BamlValueWithMeta::Map(index_map, _) => {
            let dict = pyo3::types::PyDict::new(py);
            for (key, value) in index_map {
                let key = key.into_pyobject(py)?;
                let value = pythonize_strict(py, value, enum_module, cls_module)?;
                dict.set_item(key, value)?;
            }
            Ok(dict.into())
        }
        BamlValueWithMeta::List(vec, _) => pyo3::types::PyList::new(
            py,
            vec.into_iter()
                .map(|v| pythonize_strict(py, v, enum_module, cls_module))
                .collect::<PyResult<Vec<_>>>()?,
        )?
        .into_py_any(py),
        BamlValueWithMeta::Media(baml_media, _) => match baml_media.media_type {
            baml_types::BamlMediaType::Image => {
                BamlImagePy::from(baml_media.clone()).into_py_any(py)
            }
            baml_types::BamlMediaType::Audio => {
                BamlAudioPy::from(baml_media.clone()).into_py_any(py)
            }
        },
        BamlValueWithMeta::Enum(enum_name, ref value, _) => {
            let enum_type = match enum_module.getattr(enum_name.as_str()) {
                Ok(e) => e,
                // This can be true in the case of dynamic types.
                /*
                   tb = TypeBuilder()
                   tb.add_enum("Foo")
                */
                Err(_) => return value.into_py_any(py),
            };

            // Call the constructor with the value
            let instance = match enum_type.call1((value,)) {
                Ok(instance) => instance,
                Err(_) => {
                    // This can happen if the enum value is dynamic
                    /*
                       enum Foo {
                           @@dynamic
                       }
                    */
                    return value.into_py_any(py);
                }
            };
            Ok(instance.into())
        }
        BamlValueWithMeta::Class(class_name, index_map, _) => {
            let properties = index_map
                .into_iter()
                .map(|(key, value)| {
                    let value = pythonize_strict(py, value, enum_module, cls_module)?;
                    Ok((key.clone(), value))
                })
                .collect::<PyResult<Vec<_>>>()?;

            let properties_dict = pyo3::types::PyDict::new(py);
            for (key, value) in properties {
                // For each field, try to call pydantic's `model_dump` on the
                // field. This is necessary in case the field is `Checked[_,_]`,
                // because pydantic requires to parse such fields from json,
                // rather than from a Python object. The python object is an
                // untyped Dict, but python expects a typed `Checked`.
                // By turning such values into `json`, we allow pydantic's
                // parser to more flexibly accept input at its expected
                // type.
                //
                // This has the side-effect of calling `model_dump` on all
                // classes inheriting `BaseModel`, which probably incurs some
                // performance penalty. So we should consider testing whether
                // the field is a `Checked` before doing a `model_dump`.
                let value_model = value.call_method0(py, "model_dump");
                match value_model {
                    Err(_) => {
                        properties_dict.set_item(key, value)?;
                    }
                    Ok(m) => {
                        properties_dict.set_item(key, m)?;
                    }
                }
            }

            let class_type = match cls_module.getattr(class_name.as_str()) {
                Ok(class) => class,
                // This can be true in the case of dynamic types.
                /*
                    tb = TypeBuilder()
                    tb.add_class("Foo")
                */
                Err(_) => return Ok(properties_dict.into()),
            };

            let instance =
                class_type.call_method("model_validate", (properties_dict.clone(),), None)?;

            Ok(instance.into())
        }
        BamlValueWithMeta::Null(_) => Ok(py.None()),
    }?;

    if meta.is_empty() {
        Ok(py_value_without_constraints)
    } else {
        // Generate the Python checks
        let python_checks = pythonize_checks(py, cls_module, &meta)?;

        // Get the type of the original value
        let value_type = py_value_without_constraints.bind(py).get_type();

        // Import the necessary modules and objects
        let typing = py.import("typing")?;
        let literal = typing.getattr("Literal")?;

        // Collect check names as &str and turn them into a Python tuple
        let check_names: Vec<&str> = meta.iter().map(|check| check.name.as_str()).collect();
        let literal_args = PyTuple::new(py, check_names)?;

        // Call Literal[...] dynamically
        let literal_check_names = literal.get_item(literal_args)?;

        // Prepare the properties dictionary
        let properties_dict = pyo3::types::PyDict::new(py);
        properties_dict.set_item("value", py_value_without_constraints)?;
        properties_dict.set_item("checks", python_checks)?;

        let class_checked_type_constructor = cls_module.getattr("Checked")?;

        // Prepare type parameters for Checked[...]
        let type_parameters_tuple = PyTuple::new(py, [value_type.as_ref(), &literal_check_names])?;

        // Create the Checked type using __class_getitem__
        let class_checked_type: Bound<'_, PyAny> = class_checked_type_constructor
            .call_method1("__class_getitem__", (type_parameters_tuple,))?;

        // Validate the model with the constructed type
        let checked_instance =
            class_checked_type.call_method("model_validate", (properties_dict.clone(),), None)?;

        Ok(checked_instance.into())
    }
}
