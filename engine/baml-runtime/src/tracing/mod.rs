pub mod api_wrapper;

use crate::on_log_event::LogEventCallbackSync;
use crate::InnerTraceStats;
use anyhow::{Context, Result};
use baml_types::{BamlMap, BamlMediaType, BamlValue};
use cfg_if::cfg_if;
use colored::{ColoredString, Colorize};
use internal_baml_jinja::RenderedPrompt;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{
    client_registry::ClientRegistry, internal::llm_client::LLMResponse,
    tracing::api_wrapper::core_types::Role, type_builder::TypeBuilder, FunctionResult,
    RuntimeContext, RuntimeContextManager, SpanCtx, TestResponse, TraceStats,
};

use self::api_wrapper::{
    core_types::{
        ContentPart, EventChain, IOValue, LLMChat, LLMEventInput, LLMEventInputPrompt,
        LLMEventSchema, LLMOutputModel, LogSchema, LogSchemaContext, MetadataType, Template,
        TypeSchema, IO,
    },
    APIWrapper,
};
use ::tracing as rust_tracing;
use valuable::Valuable;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm_tracer;
        use self::wasm_tracer::NonThreadedTracer as TracerImpl;
    } else {
        mod threaded_tracer;
        use self::threaded_tracer::ThreadedTracer as TracerImpl;
    }
}

#[derive(Debug, Clone)]
pub struct TracingSpan {
    span_id: Uuid,
    params: BamlMap<String, BamlValue>,
    start_time: web_time::SystemTime,
}

pub struct BamlTracer {
    options: APIWrapper,
    tracer: Option<TracerImpl>,
    trace_stats: TraceStats,
}

#[cfg(not(target_arch = "wasm32"))]
static_assertions::assert_impl_all!(BamlTracer: Send, Sync);

/// Trait for types that can be visualized in terminal logs
pub trait Visualize {
    fn visualize(&self, max_chunk_size: usize) -> String;
}

fn log_str() -> ColoredString {
    "...[log trimmed]...".yellow().dimmed()
}

pub fn truncate_string(s: &str, max_size: usize) -> String {
    if max_size > 0 && s.len() > max_size {
        let half_size = max_size / 2;
        // We use UTF-8 aware char_indices to get the correct byte index (can't just do s[..half_size])
        let start = s
            .char_indices()
            .take(half_size)
            .map(|(i, _)| i)
            .last()
            .unwrap_or(0);
        let end = s
            .char_indices()
            .rev()
            .take(half_size)
            .map(|(i, _)| i)
            .last()
            .unwrap_or(s.len());
        format!("{}{}{}", &s[..start], log_str(), &s[end..])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("1234567890", 10), "1234567890".to_string());
        assert_eq!(
            truncate_string("12345678901", 10),
            format!("1234{}78901", log_str())
        );
        assert_eq!(truncate_string("12345678901", 0), "12345678901".to_string());
    }

    #[test]
    fn test_unicode_truncate_string() {
        assert_eq!(
            truncate_string(r#"👍👍👍👍👍👍👍"#, 4),
            format!(r#"👍{}👍👍"#, log_str())
        );
    }
}

impl Visualize for FunctionResult {
    fn visualize(&self, max_chunk_size: usize) -> String {
        let mut s = vec![];
        if self.event_chain().len() > 1 {
            s.push(format!(
                "{}",
                format!("({} other previous tries)", self.event_chain().len() - 1).yellow()
            ));
        }
        s.push(self.llm_response().visualize(max_chunk_size));
        match self.result_with_constraints() {
            Some(Ok(val)) => {
                s.push(format!(
                    "{}",
                    format!("---Parsed Response ({})---", val.r#type()).blue()
                ));
                let json_str = serde_json::to_string_pretty(&val).unwrap();
                s.push(truncate_string(&json_str, max_chunk_size).to_string());
            }
            Some(Err(e)) => {
                s.push(format!(
                    "{}",
                    format!("---Parsed Response ({})---", "Error".red()).blue()
                ));
                s.push(format!(
                    "{}",
                    truncate_string(&e.to_string(), max_chunk_size).red()
                ));
            }
            None => {}
        };
        s.join("\n")
    }
}

// A best effort way of serializing the baml_event log into a structured format.
// Users will see this as JSON in their logs (primarily in baml server)
// We may break this at any time.
// It differs from the LogEvent that is sent to the on_log_event callback in that it doesn't include
// actual tracing details like span_id, event_chain, (for now).
#[derive(Valuable)]
struct BamlEventJson {
    // Metadata
    start_time: String,
    num_tries: usize,
    total_tries: usize,

    // LLM Info
    client: String,
    model: String,
    latency_ms: u128,
    stop_reason: Option<String>,

    // Content
    prompt: Option<String>,
    llm_reply: Option<String>,
    // JSON string
    request_options_json: Option<String>,

    // Token Usage
    tokens: Option<TokenUsage>,

    // Response/Error Info
    parsed_response_type: Option<String>,
    parsed_response: Option<String>,
    error: Option<String>,
}

#[derive(Valuable)]
struct TokenUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
}

impl BamlTracer {
    pub fn new<T: AsRef<str>>(
        options: Option<APIWrapper>,
        env_vars: impl Iterator<Item = (T, T)>,
    ) -> Result<Self> {
        let options = match options {
            Some(wrapper) => wrapper,
            None => APIWrapper::from_env_vars(env_vars)?,
        };

        let trace_stats = TraceStats::default();

        let tracer = BamlTracer {
            tracer: if options.enabled() {
                Some(TracerImpl::new(&options, 20, trace_stats.clone()))
            } else {
                None
            },
            options,
            trace_stats,
        };
        Ok(tracer)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_log_event_callback(&self, log_event_callback: Option<LogEventCallbackSync>) {
        if let Some(tracer) = &self.tracer {
            tracer.set_log_event_callback(log_event_callback);
        }
    }

    pub(crate) fn flush(&self) -> Result<()> {
        if let Some(ref tracer) = self.tracer {
            tracer.flush().context("Failed to flush BAML traces")?;
        }

        Ok(())
    }

    pub(crate) fn drain_stats(&self) -> InnerTraceStats {
        self.trace_stats.drain()
    }

    pub(crate) fn start_span(
        &self,
        function_name: &str,
        ctx: &RuntimeContextManager,
        params: &BamlMap<String, BamlValue>,
    ) -> Option<TracingSpan> {
        self.trace_stats.guard().start();
        let span_id = ctx.enter(function_name);
        log::trace!("Entering span {:#?} in {:?}", span_id, function_name);
        let span = TracingSpan {
            span_id,
            params: params.clone(),
            start_time: web_time::SystemTime::now(),
        };

        Some(span)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn finish_span(
        &self,
        span: TracingSpan,
        ctx: &RuntimeContextManager,
        response: Option<BamlValue>,
    ) -> Result<Option<uuid::Uuid>> {
        let guard = self.trace_stats.guard();

        let Some((span_id, event_chain, tags)) = ctx.exit() else {
            anyhow::bail!(
                "Attempting to finish a span {:#?} without first starting one. Current context {:#?}",
                span,
                ctx
            );
        };

        if span.span_id != span_id {
            anyhow::bail!("Span ID mismatch: {} != {}", span.span_id, span_id);
        }

        if let Some(tracer) = &self.tracer {
            tracer
                .submit(response.to_log_schema(&self.options, event_chain, tags, span))
                .await?;
            guard.done();
            Ok(Some(span_id))
        } else {
            guard.done();
            Ok(None)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn finish_span(
        &self,
        span: TracingSpan,
        ctx: &RuntimeContextManager,
        response: Option<BamlValue>,
    ) -> Result<Option<uuid::Uuid>> {
        let guard = self.trace_stats.guard();
        let Some((span_id, event_chain, tags)) = ctx.exit() else {
            anyhow::bail!(
                "Attempting to finish a span {:#?} without first starting one. Current context {:#?}",
                span,
                ctx
            );
        };
        log::trace!(
            "Finishing span: {:#?} {}\nevent chain {:?}",
            span,
            span_id,
            event_chain
        );

        if span.span_id != span_id {
            anyhow::bail!("Span ID mismatch: {} != {}", span.span_id, span_id);
        }

        if let Some(tracer) = &self.tracer {
            tracer.submit(response.to_log_schema(&self.options, event_chain, tags, span))?;
            guard.finalize();
            Ok(Some(span_id))
        } else {
            guard.done();
            Ok(None)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn finish_baml_span(
        &self,
        span: TracingSpan,
        ctx: &RuntimeContextManager,
        response: &Result<FunctionResult>,
    ) -> Result<Option<uuid::Uuid>> {
        let guard = self.trace_stats.guard();
        let Some((span_id, event_chain, tags)) = ctx.exit() else {
            anyhow::bail!("Attempting to finish a span without first starting one");
        };

        if span.span_id != span_id {
            anyhow::bail!("Span ID mismatch: {} != {}", span.span_id, span_id);
        }

        if let Ok(response) = &response {
            let name = event_chain.last().map(|s| s.name.as_str());
            let is_ok = response
                .result_with_constraints()
                .as_ref()
                .is_some_and(|r| r.is_ok());
            log::log!(
                target: "baml_events",
                if is_ok { log::Level::Info } else { log::Level::Warn },
                "{}{}",
                name.map(|s| format!("Function {}:\n", s)).unwrap_or_default().purple(),
                response.visualize(self.options.config.max_log_chunk_chars())
            );
        }

        if let Some(tracer) = &self.tracer {
            tracer
                .submit(response.to_log_schema(&self.options, event_chain, tags, span))
                .await?;
            guard.done();
            Ok(Some(span_id))
        } else {
            guard.done();
            Ok(None)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn finish_baml_span(
        &self,
        span: TracingSpan,
        ctx: &RuntimeContextManager,
        response: &Result<FunctionResult>,
    ) -> Result<Option<uuid::Uuid>> {
        let guard = self.trace_stats.guard();
        let Some((span_id, event_chain, tags)) = ctx.exit() else {
            anyhow::bail!("Attempting to finish a span without first starting one");
        };

        log::trace!(
            "Finishing baml span: {:#?} {}\nevent chain {:?}",
            span,
            span_id,
            event_chain
        );

        if span.span_id != span_id {
            anyhow::bail!("Span ID mismatch: {} != {}", span.span_id, span_id);
        }

        let log_json = Self::is_json_logging_enabled();

        match response {
            Ok(response) => {
                self.handle_ok_response(response, log_json, &event_chain, &tags, &span)?
            }
            Err(e) => self.handle_error_response(e, log_json, &span),
        }

        if let Some(tracer) = &self.tracer {
            tracer.submit(response.to_log_schema(&self.options, event_chain, tags, span))?;
            guard.finalize();
            Ok(Some(span_id))
        } else {
            guard.done();
            Ok(None)
        }
    }

    fn is_json_logging_enabled() -> bool {
        matches!(
            std::env::var("BAML_LOG_JSON"),
            Ok(val) if val.trim().eq_ignore_ascii_case("true") || val.trim() == "1"
        )
    }

    fn handle_ok_response(
        &self,
        response: &FunctionResult,
        log_json: bool,
        event_chain: &[SpanCtx],
        tags: &HashMap<String, BamlValue>,
        span: &TracingSpan,
    ) -> Result<()> {
        let name = event_chain.last().map(|s| s.name.as_str());
        let is_ok = response
            .result_with_constraints()
            .as_ref()
            .is_some_and(|r| r.is_ok());

        let log_schema = response.to_log_schema(
            &self.options,
            event_chain.to_vec(),
            tags.clone(),
            span.clone(),
        );

        if log_json {
            let log_event = self.build_baml_event_json(response, span);
            log_json_event(is_ok, log_event)?;
        } else {
            log_simple_event(is_ok, name, response, &self.options);
        }

        Ok(())
    }

    fn handle_error_response(&self, error: &anyhow::Error, log_json: bool, span: &TracingSpan) {
        if log_json {
            let baml_event_json = BamlEventJson {
                start_time: to_iso_string(&span.start_time),
                num_tries: 0,
                total_tries: 0,
                client: "unknown".to_string(),
                model: "unknown".to_string(),
                latency_ms: 0,
                stop_reason: None,
                prompt: None,
                llm_reply: None,
                request_options_json: None,
                tokens: None,
                parsed_response_type: None,
                parsed_response: None,
                error: Some(error.to_string()),
            };
            rust_tracing::event!(
                target: "baml_events",
                rust_tracing::Level::ERROR,
                baml_event = baml_event_json.as_value()
            );
        } else {
            log::error!("{}", error);
        }
    }

    fn build_baml_event_json(
        &self,
        response: &FunctionResult,
        span: &TracingSpan,
    ) -> BamlEventJson {
        let last_ctx = response.llm_response();
        let start_time = to_iso_string(&span.start_time);
        let num_tries = response.event_chain().len();
        let total_tries = response.event_chain().len();
        let error = error_from_result(response).map(|e| e.message.clone());

        match last_ctx {
            LLMResponse::Success(resp) => BamlEventJson {
                start_time,
                num_tries,
                total_tries,
                client: resp.client.clone(),
                model: resp.model.clone(),
                latency_ms: resp.latency.as_millis(),
                stop_reason: resp.metadata.finish_reason.clone(),
                prompt: Some(resp.prompt.to_string()),
                llm_reply: Some(resp.content.clone()),
                request_options_json: Some(
                    serde_json::to_string(&resp.request_options).unwrap_or_default(),
                ),
                tokens: Some(TokenUsage {
                    prompt_tokens: resp.metadata.prompt_tokens,
                    completion_tokens: resp.metadata.output_tokens,
                    total_tokens: resp.metadata.total_tokens,
                }),
                parsed_response_type: response
                    .result_with_constraints()
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .map(|v| v.r#type().to_string()),
                parsed_response: response
                    .result_with_constraints()
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                error,
            },
            LLMResponse::LLMFailure(err) => BamlEventJson {
                start_time,
                num_tries,
                total_tries,
                client: err.client.clone(),
                model: err.model.clone().unwrap_or_default(),
                latency_ms: err.latency.as_millis(),
                stop_reason: None,
                prompt: Some(err.prompt.to_string()),
                llm_reply: None,
                request_options_json: Some(
                    serde_json::to_string(&err.request_options).unwrap_or_default(),
                ),
                tokens: None,
                parsed_response_type: None,
                parsed_response: None,
                error,
            },
            LLMResponse::UserFailure(msg) | LLMResponse::InternalFailure(msg) => BamlEventJson {
                start_time,
                num_tries,
                total_tries,
                client: "unknown".to_string(),
                model: "unknown".to_string(),
                latency_ms: 0,
                stop_reason: None,
                prompt: None,
                llm_reply: None,
                request_options_json: None,
                tokens: None,
                parsed_response_type: None,
                parsed_response: None,
                error: Some(msg.clone()),
            },
        }
    }
}

fn log_json_event(is_ok: bool, log_event: BamlEventJson) -> Result<()> {
    if is_ok {
        rust_tracing::event!(
            target: "baml_events",
            rust_tracing::Level::INFO,
            baml_event = log_event.as_value()
        );
    } else {
        rust_tracing::event!(
            target: "baml_events",
            rust_tracing::Level::WARN,
            baml_event = log_event.as_value()
        );
    }
    Ok(())
}

fn log_simple_event(
    is_ok: bool,
    name: Option<&str>,
    response: &FunctionResult,
    options: &APIWrapper,
) {
    log::log!(
        target: "baml_events",
        if is_ok { log::Level::Info } else { log::Level::Warn },
        "{}{}",
        name.map(|s| format!("Function {}:\n", s)).unwrap_or_default().purple(),
        response.visualize(options.config.max_log_chunk_chars())
    );
}

// Function to convert web_time::SystemTime to ISO 8601 string
fn to_iso_string(web_time: &web_time::SystemTime) -> String {
    let time = web_time.duration_since(web_time::UNIX_EPOCH).unwrap();
    // Convert to ISO 8601 string
    chrono::DateTime::from_timestamp_millis(time.as_millis() as i64)
        .unwrap()
        .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
}

impl
    From<(
        &APIWrapper,
        Vec<SpanCtx>,
        HashMap<String, BamlValue>,
        &TracingSpan,
    )> for LogSchemaContext
{
    fn from(
        (api, event_chain, tags, span): (
            &APIWrapper,
            Vec<SpanCtx>,
            HashMap<String, BamlValue>,
            &TracingSpan,
        ),
    ) -> Self {
        let parent_chain = event_chain
            .iter()
            .map(|ctx| EventChain {
                function_name: ctx.name.clone(),
                variant_name: None,
            })
            .collect::<Vec<_>>();
        LogSchemaContext {
            hostname: api.host_name().to_string(),
            stage: Some(api.stage().to_string()),
            latency_ms: span
                .start_time
                .elapsed()
                .map(|d| d.as_millis() as i128)
                .unwrap_or(0),
            process_id: api.session_id().to_string(),
            tags: tags
                .into_iter()
                .map(|(k, v)| match v.as_str() {
                    Some(v) => (k, v.to_string()),
                    None => (
                        k,
                        serde_json::to_string(&v).unwrap_or_else(|_| "<unknown>".to_string()),
                    ),
                })
                .chain(std::iter::once((
                    "baml.runtime".to_string(),
                    env!("CARGO_PKG_VERSION").to_string(),
                )))
                .collect(),
            event_chain: parent_chain,
            start_time: to_iso_string(&span.start_time),
        }
    }
}

impl From<&BamlMap<String, BamlValue>> for IOValue {
    fn from(items: &BamlMap<String, BamlValue>) -> Self {
        log::trace!("Converting IOValue from BamlMap: {:#?}", items);
        IOValue {
            r#type: TypeSchema {
                name: api_wrapper::core_types::TypeSchemaName::Multi,
                fields: items.iter().map(|(k, v)| (k.clone(), v.r#type())).collect(),
            },
            value: api_wrapper::core_types::ValueType::List(
                items
                    .iter()
                    .map(|(_, v)| {
                        serde_json::to_string(v).unwrap_or_else(|_| "<unknown>".to_string())
                    })
                    .collect(),
            ),
            r#override: None,
        }
    }
}

impl From<&BamlValue> for IOValue {
    fn from(value: &BamlValue) -> Self {
        match value {
            BamlValue::Map(obj) => obj.into(),
            _ => IOValue {
                r#type: TypeSchema {
                    name: api_wrapper::core_types::TypeSchemaName::Single,
                    fields: [("value".into(), value.r#type())].into(),
                },
                value: api_wrapper::core_types::ValueType::String(
                    serde_json::to_string(value).unwrap_or_else(|_| "<unknown>".to_string()),
                ),
                r#override: None,
            },
        }
    }
}

fn error_from_result(result: &FunctionResult) -> Option<api_wrapper::core_types::Error> {
    match result.result_with_constraints() {
        Some(Ok(_)) => None,
        Some(Err(e)) => Some(api_wrapper::core_types::Error {
            code: 2,
            message: e.to_string(),
            traceback: None,
            r#override: None,
        }),
        None => match result.llm_response() {
            LLMResponse::Success(_) => None,
            LLMResponse::LLMFailure(s) => Some(api_wrapper::core_types::Error {
                code: 2,
                message: s.message.clone(),
                traceback: None,
                r#override: None,
            }),
            LLMResponse::UserFailure(s) => Some(api_wrapper::core_types::Error {
                code: 2,
                message: s.clone(),
                traceback: None,
                r#override: None,
            }),
            LLMResponse::InternalFailure(s) => Some(api_wrapper::core_types::Error {
                code: 2,
                message: s.clone(),
                traceback: None,
                r#override: None,
            }),
        },
    }
}

trait ToLogSchema {
    // Event_chain is guaranteed to have at least one element
    fn to_log_schema(
        &self,
        api: &APIWrapper,
        event_chain: Vec<SpanCtx>,
        tags: HashMap<String, BamlValue>,
        span: TracingSpan,
    ) -> LogSchema;
}

impl<T: ToLogSchema> ToLogSchema for Result<T> {
    fn to_log_schema(
        &self,
        api: &APIWrapper,
        event_chain: Vec<SpanCtx>,
        tags: HashMap<String, BamlValue>,
        span: TracingSpan,
    ) -> LogSchema {
        match self {
            Ok(r) => r.to_log_schema(api, event_chain, tags, span),
            Err(e) => LogSchema {
                project_id: api.project_id().map(|s| s.to_string()),
                event_type: api_wrapper::core_types::EventType::FuncCode,
                root_event_id: event_chain.first().map(|s| s.span_id).unwrap().to_string(),
                event_id: event_chain.last().map(|s| s.span_id).unwrap().to_string(),
                parent_event_id: None,
                context: (api, event_chain, tags, &span).into(),
                io: IO {
                    input: Some((&span.params).into()),
                    output: None,
                },
                error: Some(api_wrapper::core_types::Error {
                    code: 2,
                    message: e.to_string(),
                    traceback: None,
                    r#override: None,
                }),
                metadata: None,
            },
        }
    }
}

impl ToLogSchema for Option<BamlValue> {
    // Event_chain is guaranteed to have at least one element
    fn to_log_schema(
        &self,
        api: &APIWrapper,
        event_chain: Vec<SpanCtx>,
        tags: HashMap<String, BamlValue>,
        span: TracingSpan,
    ) -> LogSchema {
        LogSchema {
            project_id: api.project_id().map(|s| s.to_string()),
            event_type: api_wrapper::core_types::EventType::FuncCode,
            root_event_id: event_chain.first().map(|s| s.span_id).unwrap().to_string(),
            event_id: event_chain.last().map(|s| s.span_id).unwrap().to_string(),
            parent_event_id: if event_chain.len() >= 2 {
                event_chain
                    .get(event_chain.len() - 2)
                    .map(|s| s.span_id.to_string())
            } else {
                None
            },
            context: (api, event_chain, tags, &span).into(),
            io: IO {
                input: Some((&span.params).into()),
                output: self.as_ref().map(|r| r.into()),
            },
            error: None,
            metadata: None,
        }
    }
}

impl ToLogSchema for TestResponse {
    fn to_log_schema(
        &self,
        api: &APIWrapper,
        event_chain: Vec<SpanCtx>,
        tags: HashMap<String, BamlValue>,
        span: TracingSpan,
    ) -> LogSchema {
        self.function_response
            .to_log_schema(api, event_chain, tags, span)
    }
}

impl ToLogSchema for FunctionResult {
    fn to_log_schema(
        &self,
        api: &APIWrapper,
        event_chain: Vec<SpanCtx>,
        tags: HashMap<String, BamlValue>,
        span: TracingSpan,
    ) -> LogSchema {
        LogSchema {
            project_id: api.project_id().map(|s| s.to_string()),
            event_type: api_wrapper::core_types::EventType::FuncLlm,
            root_event_id: event_chain.first().map(|s| s.span_id).unwrap().to_string(),
            event_id: event_chain.last().map(|s| s.span_id).unwrap().to_string(),
            // Second to last element in the event chain
            parent_event_id: if event_chain.len() >= 2 {
                event_chain
                    .get(event_chain.len() - 2)
                    .map(|s| s.span_id.to_string())
            } else {
                None
            },
            context: (api, event_chain, tags, &span).into(),
            io: IO {
                input: Some((&span.params).into()),
                output: self
                    .result_with_constraints()
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .map(|r| {
                        let v: BamlValue = r.into();
                        IOValue::from(&v)
                    }),
            },
            error: error_from_result(self),
            metadata: Some(self.into()),
        }
    }
}

impl From<&FunctionResult> for MetadataType {
    fn from(result: &FunctionResult) -> Self {
        MetadataType::Multi(
            result
                .event_chain()
                .iter()
                .map(|(_, r, _, _)| r.into())
                .collect::<Vec<_>>(),
        )
    }
}

impl From<&LLMResponse> for LLMEventSchema {
    fn from(response: &LLMResponse) -> Self {
        match response {
            LLMResponse::UserFailure(s) => LLMEventSchema {
                model_name: "<unknown>".into(),
                provider: "<unknown>".into(),
                input: LLMEventInput {
                    prompt: LLMEventInputPrompt {
                        template: Template::Single("<unable to render prompt>".into()),
                        template_args: Default::default(),
                        r#override: None,
                    },
                    request_options: Default::default(),
                },
                output: None,
                error: Some(s.clone()),
            },
            LLMResponse::InternalFailure(s) => LLMEventSchema {
                model_name: "<unknown>".into(),
                provider: "<unknown>".into(),
                input: LLMEventInput {
                    prompt: LLMEventInputPrompt {
                        template: Template::Single("<unable to render prompt>".into()),
                        template_args: Default::default(),
                        r#override: None,
                    },
                    request_options: Default::default(),
                },
                output: None,
                error: Some(s.clone()),
            },
            LLMResponse::Success(s) => LLMEventSchema {
                model_name: s.model.clone(),
                provider: s.client.clone(),
                input: LLMEventInput {
                    prompt: LLMEventInputPrompt {
                        template: (&s.prompt).into(),
                        template_args: Default::default(),
                        r#override: None,
                    },
                    request_options: s.request_options.clone(),
                },
                output: Some(LLMOutputModel {
                    raw_text: s.content.clone(),
                    metadata: serde_json::to_value(&s.metadata)
                        .map_or_else(Err, serde_json::from_value)
                        .unwrap_or_default(),
                    r#override: None,
                }),
                error: None,
            },
            LLMResponse::LLMFailure(s) => LLMEventSchema {
                model_name: s
                    .model
                    .as_ref()
                    .map_or_else(|| "<unknown>", |f| f.as_str())
                    .into(),
                provider: s.client.clone(),
                input: LLMEventInput {
                    prompt: LLMEventInputPrompt {
                        template: (&s.prompt).into(),
                        template_args: Default::default(),
                        r#override: None,
                    },
                    request_options: s.request_options.clone(),
                },
                output: None,
                error: Some(s.message.clone()),
            },
        }
    }
}

impl From<&internal_baml_jinja::ChatMessagePart> for ContentPart {
    fn from(value: &internal_baml_jinja::ChatMessagePart) -> Self {
        match value {
            internal_baml_jinja::ChatMessagePart::Text(t) => ContentPart::Text(t.clone()),
            internal_baml_jinja::ChatMessagePart::Media(media) => {
                match (media.media_type, &media.content) {
                    (BamlMediaType::Image, baml_types::BamlMediaContent::File(data)) => {
                        ContentPart::FileImage(
                            data.span_path.to_string_lossy().into_owned(),
                            data.relpath.to_string_lossy().into_owned(),
                        )
                    }
                    (BamlMediaType::Audio, baml_types::BamlMediaContent::File(data)) => {
                        ContentPart::FileAudio(
                            data.span_path.to_string_lossy().into_owned(),
                            data.relpath.to_string_lossy().into_owned(),
                        )
                    }
                    (BamlMediaType::Image, baml_types::BamlMediaContent::Base64(data)) => {
                        ContentPart::B64Image(data.base64.clone())
                    }
                    (BamlMediaType::Audio, baml_types::BamlMediaContent::Base64(data)) => {
                        ContentPart::B64Audio(data.base64.clone())
                    }
                    (BamlMediaType::Image, baml_types::BamlMediaContent::Url(data)) => {
                        ContentPart::UrlImage(data.url.clone())
                    }
                    (BamlMediaType::Audio, baml_types::BamlMediaContent::Url(data)) => {
                        ContentPart::UrlAudio(data.url.clone())
                    }
                }
            }
            internal_baml_jinja::ChatMessagePart::WithMeta(inner, meta) => ContentPart::WithMeta(
                Box::new(inner.as_ref().into()),
                meta.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            ),
        }
    }
}

impl From<&RenderedPrompt> for Template {
    fn from(value: &RenderedPrompt) -> Self {
        match value {
            RenderedPrompt::Completion(c) => Template::Single(c.clone()),
            RenderedPrompt::Chat(c) => Template::Multiple(
                c.iter()
                    .map(|c| LLMChat {
                        role: match serde_json::from_value::<Role>(serde_json::json!(c.role)) {
                            Ok(r) => r,
                            Err(e) => {
                                log::error!("Failed to parse role: {} {:#?}", e, c.role);
                                Role::Other(c.role.clone())
                            }
                        },
                        content: c.parts.iter().map(|p| p.into()).collect::<Vec<_>>(),
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}
