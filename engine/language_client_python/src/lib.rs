mod errors;
mod parse_py_type;
mod runtime;
mod types;

use pyo3::prelude::{pyfunction, pymodule, PyAnyMethods, PyModule, PyResult};
use pyo3::{wrap_pyfunction, Bound, Python};
use tracing_subscriber::{self, EnvFilter};

#[pyfunction]
fn invoke_runtime_cli(py: Python) -> PyResult<()> {
    baml_cli::run_cli(
        py.import_bound("sys")?
            .getattr("argv")?
            .extract::<Vec<String>>()?,
        baml_runtime::RuntimeCliDefaults {
            output_type: baml_types::GeneratorOutputType::PythonPydantic,
        },
    )
    .map_err(errors::BamlError::from_anyhow)
}
#[pyo3::prelude::pyclass(module = "baml_py.baml_py")]
pub struct LoremIpsum {
    #[allow(dead_code)]
    pub(crate) inner: String,
}

#[pyo3::prelude::pymethods]
impl LoremIpsum {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: "Lorem ipsum dolor sit amet".to_string(),
        }
    }

    // pub fn __getnewargs__<'py>(
    //     &self,
    //     py: Python<'py>,
    // ) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
    //     println!("__getnewargs__ LoremIpsum placeholder");
    //     Ok(pyo3::types::PyTuple::empty_bound(py))
    // }

    #[classmethod]
    pub fn __get_pydantic_core_schema__(
        _cls: Bound<'_, pyo3::types::PyType>,
        _source_type: Bound<'_, pyo3::types::PyAny>,
        _handler: Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<pyo3::PyObject> {
        Python::with_gil(|py| {
            let code = r#"
from pydantic_core import core_schema, SchemaValidator

ret = core_schema.str_schema()
    "#;
            // py.run(code, None, Some(ret_dict));
            let fun: pyo3::Py<pyo3::types::PyAny> =
                PyModule::from_code_bound(py, code, "pretend-file", "pretend-module")?
                    .getattr("ret")?
                    .into();
            use pyo3::ToPyObject;
            Ok(fun.to_object(py))
        })
    }
}

pub(crate) const MODULE_NAME: &str = "baml_py.baml_py";

#[pymodule]
fn baml_py(m: Bound<'_, PyModule>) -> PyResult<()> {
    let use_json = match std::env::var("BAML_LOG_JSON") {
        Ok(val) => val.trim().eq_ignore_ascii_case("true") || val.trim() == "1",
        Err(_) => false,
    };

    if use_json {
        // JSON formatting
        tracing_subscriber::fmt()
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .json()
            .with_env_filter(
                EnvFilter::try_from_env("BAML_LOG").unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .flatten_event(true)
            .with_current_span(false)
            .with_span_list(false)
            .init();
    } else {
        // Regular formatting
        if let Err(e) = env_logger::try_init_from_env(
            env_logger::Env::new()
                .filter("BAML_LOG")
                .write_style("BAML_LOG_STYLE"),
        ) {
            eprintln!("Failed to initialize BAML logger: {:#}", e);
        }
    }

    m.add_class::<LoremIpsum>()?;

    m.add_class::<runtime::BamlRuntime>()?;

    m.add_class::<types::FunctionResult>()?;
    m.add_class::<types::FunctionResultStream>()?;
    m.add_class::<types::SyncFunctionResultStream>()?;
    m.add_class::<types::BamlImagePy>()?;
    m.add_class::<types::BamlAudioPy>()?;
    m.add_class::<types::RuntimeContextManager>()?;
    m.add_class::<types::BamlSpan>()?;
    m.add_class::<types::TypeBuilder>()?;
    m.add_class::<types::EnumBuilder>()?;
    m.add_class::<types::ClassBuilder>()?;
    m.add_class::<types::EnumValueBuilder>()?;
    m.add_class::<types::ClassPropertyBuilder>()?;
    m.add_class::<types::FieldType>()?;
    m.add_class::<types::ClientRegistry>()?;

    m.add_class::<runtime::BamlLogEvent>()?;
    m.add_class::<runtime::LogEventMetadata>()?;

    m.add_wrapped(wrap_pyfunction!(invoke_runtime_cli))?;

    // m.add(
    //     "BamlValidationError",
    //     m.py().get_type_bound::<errors::BamlValidationError>(),
    // )?;
    // m.add_class::<errors::BamlValidationError>()?;
    errors::errors(&m)?;

    Ok(())
}

mod test {
    #[test]
    fn test_inspect() {
        assert_eq!(
            crate::MODULE_NAME,
            format!("baml_py.{}", stringify!(baml_asdfpy))
        );
    }
}
