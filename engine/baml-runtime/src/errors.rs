pub enum ExposedError {
    /// Error in parsing post calling the LLM
    ValidationError {
        prompt: String,
        raw_output: String,
        message: String,
    },
    FinishReasonError {
        prompt: String,
        raw_output: String,
        message: String,
        finish_reason: Option<String>,
    },
}

impl std::error::Error for ExposedError {}

impl std::fmt::Display for ExposedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExposedError::ValidationError {
                prompt,
                raw_output,
                message,
            } => {
                write!(
                    f,
                    "Parsing error: {}\nPrompt: {}\nRaw Response: {}",
                    message, prompt, raw_output
                )
            }
            ExposedError::FinishReasonError {
                prompt,
                raw_output,
                message,
                finish_reason,
            } => {
                write!(
                    f,
                    "Finish reason error: {}\nPrompt: {}\nRaw Response: {}\nFinish Reason: {}",
                    message,
                    prompt,
                    raw_output,
                    finish_reason.as_ref().map_or("<none>", |f| f.as_str())
                )
            }
        }
    }
}

impl std::fmt::Debug for ExposedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
