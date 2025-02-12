use crate::BamlValue;
use anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};

// TODO: use a prefixed UUID type for this
type SpanId = String;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct FunctionId(pub SpanId);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct ContentId(pub SpanId);

pub type TraceTags = serde_json::Map<String, serde_json::Value>;

// THESE ARE NOT CLONEABLE!!
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    /*
     * (span_id, content_span_id) is a unique identifier for a log event
     * The query (span_id, *) gets all logs for a function call
     */
    pub span_id: FunctionId,
    pub content_span_id: ContentId,

    // The chain of spans that lead to this log event
    // Includes span_id at the last position (content_span_id is not included)
    pub span_chain: Vec<FunctionId>,

    // The timestamp of the log
    // idk what this does yet #[serde(with = "timestamp_serde")]
    #[serde(with = "timestamp_serde")]
    pub timestamp: OffsetDateTime,

    /// human-readable callsite identifier, e.g. "ExtractResume" or "openai/gpt-4o/chat"
    pub callsite: String,

    /// verbosity level
    #[serde(with = "level_serde")]
    pub verbosity: TraceLevel,

    // The content of the log
    pub content: TraceData,

    pub tags: TraceTags,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TraceData {
    LogMessage {
        msg: String,
    },
    // All start events
    FunctionStart(FunctionStart),
    // All end events
    FunctionEnd(FunctionEnd),

    // The rest are intermediate events that happen between start and end

    // LLM request
    LLMRequest(LLMRequest),
    // Raw HTTP request to the LLM
    RawLLMRequest(HTTPRequest),

    // Do to streaming, its possible to have multiple responses for a single request
    // ----
    // Raw HTTP response from the LLM
    RawLLMResponse(HTTPResponse),
    // Parsed LLM response
    #[serde(deserialize_with = "deserialize_ok", serialize_with = "serialize_ok")]
    LLMResponse(Result<LLMResponse>),
    // ----

    // We don't want to store the parsed LLM response in the log event
    // as we have it in FunctionEnd
    #[serde(deserialize_with = "deserialize_ok", serialize_with = "serialize_ok")]
    Parsed(Result<()>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BamlOptions {
    pub type_builder: Option<serde_json::Value>,
    pub client_registry: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStart {
    pub name: String,
    pub args: Vec<BamlValue>,
    pub options: BamlOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionEnd {
    #[serde(deserialize_with = "deserialize_ok", serialize_with = "serialize_ok")]
    pub result: Result<BamlValue, anyhow::Error>,
    // Everything below is duplicated from the start event
    // to deal with the case where the log is dropped.
    // P2: as we can for now assume logs are not dropped,

    // pub name: String,
    // pub start_timestamp: web_time::Instant,
    // pub start_args: Vec<BamlValue>,
}

// LLM specific events

// TODO: fix this.
pub type Prompt = serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum LLMClient {
    Ref(String),
    ShortHand(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMRequest {
    pub client: LLMClient,
    pub params: serde_json::Value,
    pub prompt: Prompt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPRequest {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    pub url: String,
    pub method: String,
    pub headers: serde_json::Value,
    pub body: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPResponse {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    pub status: u16,
    pub headers: serde_json::Value,
    pub body: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    // Fully qualified model name
    pub finish_reason: String,
    pub model: String,
    pub usage: LLMUsage,
    pub string_output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

/// -------------------------------------------------------------------------
///
/// Helper deserializer for our Result types.
///
/// This assumes that the incoming JSON always represents the Ok variant.
/// (If you need to support error variants, you will have to expand this logic.)
///
use serde::Deserializer;
use time::OffsetDateTime;
fn deserialize_ok<'de, D, T>(deserializer: D) -> Result<Result<T, anyhow::Error>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Ok)
}

fn serialize_ok<S, T>(value: &Result<T, anyhow::Error>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Ok(v) => v.serialize(serializer),
        Err(err) => Err(serde::ser::Error::custom(format!("Error: {}", err))),
    }
}

/// -------------------------------------------------------------------------
///
/// Custom (de)serializer for timestamps.
///
/// This module is essentially the same as in tracing.rs but adapted
/// for the `web_time::Instant` type. (It expects that `web_time::Instant`
/// has a method [`unix_timestamp()`] and an associated
/// constructor [`from_unix_timestamp(i64)`].)
///
mod timestamp_serde {
    use serde::{Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(time: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(time.unix_timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp_millis: i64 = serde::Deserialize::deserialize(deserializer)?;
        OffsetDateTime::from_unix_timestamp(timestamp_millis).map_err(serde::de::Error::custom)
    }
}

// Add this helper module for tracing::Level serialization
mod level_serde {
    use super::TraceLevel;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(level: &TraceLevel, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*level as u32)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TraceLevel, D::Error>
    where
        D: Deserializer<'de>,
    {
        let level_num: u32 = serde::Deserialize::deserialize(deserializer)?;
        match level_num {
            100 => Ok(TraceLevel::Trace),
            200 => Ok(TraceLevel::Debug),
            300 => Ok(TraceLevel::Info),
            400 => Ok(TraceLevel::Warn),
            500 => Ok(TraceLevel::Error),
            600 => Ok(TraceLevel::Fatal),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid trace level: {}",
                level_num
            ))),
        }
    }
}

// unused yet
// use like this:
//  #[serde(with = "level_serde")]
//  pub verbosity: TraceLevel,
#[repr(usize)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TraceLevel {
    Trace = 100,
    Debug = 200,
    Info = 300,
    Warn = 400,
    Error = 500,
    Fatal = 600,
}

impl Into<TraceLevel> for tracing_core::Level {
    fn into(self) -> TraceLevel {
        match self {
            tracing_core::Level::TRACE => TraceLevel::Trace,
            tracing_core::Level::DEBUG => TraceLevel::Debug,
            tracing_core::Level::INFO => TraceLevel::Info,
            tracing_core::Level::WARN => TraceLevel::Warn,
            tracing_core::Level::ERROR => TraceLevel::Error,
        }
    }
}
