use anyhow::Result;
use crate::BamlValue;

// TODO: use a prefixed UUID type for this
type SpanId = String;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionId(pub SpanId);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContentId(pub SpanId);

// THESE ARE NOT CLONEABLE!!
pub struct LogEvent {
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
    pub timestamp: web_time::Instant,
    // The content of the log
    pub content: LogEventContent,
}


pub enum LogEventContent {
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
    LLMResponse(Result<LLMResponse>),
    // ----

    // We don't want to store the parsed LLM response in the log event
    // as we have it in FunctionEnd
    Parsed(Result<()>),
}

pub struct BamlOptions {
    pub type_builder: Option<serde_json::Value>,
    pub client_registry: Option<serde_json::Value>,
}

pub struct FunctionStart {
    pub name: String,
    pub args: Vec<BamlValue>,
    pub options: BamlOptions,
}

pub struct FunctionEnd {
    pub result: Result<BamlValue>,
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

pub enum LLMClient {
    Ref(String),
    ShortHand(String, String),
}

pub struct LLMRequest {
    pub client: LLMClient,
    pub params: serde_json::Value,
    pub prompt: Prompt,
}

pub struct HTTPRequest {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    pub url: String,
    pub method: String,
    pub headers: serde_json::Value,
    pub body: serde_json::Value,
}

pub struct HTTPResponse {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    pub status: u16,
    pub headers: serde_json::Value,
    pub body: serde_json::Value,
}


pub struct LLMResponse {
    // since LLM requests could be made in parallel, we need to match the response to the request
    pub request_id: ContentId,
    // Fully qualified model name
    pub finish_reason: String,
    pub model: String,
    pub usage: LLMUsage,
    pub string_output: String,
}

pub struct LLMUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

