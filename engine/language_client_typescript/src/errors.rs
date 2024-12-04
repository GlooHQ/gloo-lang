use baml_runtime::{
    errors::ExposedError, internal::llm_client::LLMResponse, scope_diagnostics::ScopeStack,
};

// napi::Error::new(napi::Status::GenericFailure, e.to_string()))

pub fn invalid_argument_error(message: &str) -> napi::Error {
    napi::Error::new(
        napi::Status::InvalidArg,
        format!("BamlError: BamlInvalidArgumentError: {}", message),
    )
}

// Creating custom errors in JS is still not supported https://github.com/napi-rs/napi-rs/issues/1205
pub fn from_anyhow_error(err: anyhow::Error) -> napi::Error {
    if let Some(er) = err.downcast_ref::<ExposedError>() {
        match er {
            ExposedError::ValidationError {
                prompt,
                message,
                raw_output: raw_response,
            } => throw_baml_validation_error(prompt, raw_response, message),
            ExposedError::FinishReasonError {
                prompt,
                message,
                raw_output: raw_response,
                finish_reason,
            } => throw_baml_client_finish_reason_error(
                prompt,
                raw_response,
                message,
                finish_reason.as_ref().map(|f| f.as_str()),
            ),
        }
    } else if let Some(er) = err.downcast_ref::<ScopeStack>() {
        invalid_argument_error(&format!("{}", er))
    } else if let Some(er) = err.downcast_ref::<LLMResponse>() {
        match er {
            LLMResponse::Success(_) => napi::Error::new(
                napi::Status::GenericFailure,
                format!("BamlError: Unexpected error from BAML: {}", err),
            ),
            LLMResponse::LLMFailure(failed) => match &failed.code {
                baml_runtime::internal::llm_client::ErrorCode::Other(2) => napi::Error::new(
                    napi::Status::GenericFailure,
                    format!(
                        "BamlError: BamlClientError: Something went wrong with the LLM client: {}",
                        failed.message
                    ),
                ),
                baml_runtime::internal::llm_client::ErrorCode::Other(_)
                | baml_runtime::internal::llm_client::ErrorCode::InvalidAuthentication
                | baml_runtime::internal::llm_client::ErrorCode::NotSupported
                | baml_runtime::internal::llm_client::ErrorCode::RateLimited
                | baml_runtime::internal::llm_client::ErrorCode::ServerError
                | baml_runtime::internal::llm_client::ErrorCode::ServiceUnavailable
                | baml_runtime::internal::llm_client::ErrorCode::UnsupportedResponse(_) => {
                    napi::Error::new(
                        napi::Status::GenericFailure,
                        format!("BamlError: BamlClientError: BamlClientHttpError: {}", err),
                    )
                }
            },
            LLMResponse::UserFailure(msg) => napi::Error::new(
                napi::Status::GenericFailure,
                format!("BamlError: BamlInvalidArgumentError: {}", msg),
            ),
            LLMResponse::InternalFailure(_) => napi::Error::new(
                napi::Status::GenericFailure,
                format!(
                    "BamlError: BamlClientError: Something went wrong with the LLM client: {}",
                    err
                ),
            ),
        }
    } else {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("BamlError: {:?}", err),
        )
    }
}

pub fn throw_baml_validation_error(prompt: &str, raw_output: &str, message: &str) -> napi::Error {
    let error_json = serde_json::json!({
        "type": "BamlValidationError",
        "prompt": prompt,
        "raw_output": raw_output,
        "message": format!("BamlValidationError: {}", message),
    });
    napi::Error::new(napi::Status::GenericFailure, error_json.to_string())
}

pub fn throw_baml_client_finish_reason_error(prompt: &str, raw_output: &str, message: &str, finish_reason: Option<&str>) -> napi::Error {
    let error_json = serde_json::json!({
        "type": "BamlClientFinishReasonError",
        "prompt": prompt,
        "raw_output": raw_output,
        "message": format!("BamlClientFinishReasonError: {}", message),
        "finish_reason": finish_reason,
    });
    napi::Error::new(napi::Status::GenericFailure, error_json.to_string())
}
