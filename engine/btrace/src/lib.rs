// use baml_types::tracing::{
//     SpanId, TraceEvent, TraceLog, TraceMetadata, TraceSpanEnd, TraceSpanStart, TraceTags,
// };

use baml_types::baml_value::BamlValue;
use baml_types::tracing::events::{
    BamlOptions, ContentId, FunctionEnd, FunctionId, FunctionStart, LogEvent, LogEventContent,
    TraceTags,
};

use std::sync::Arc;
use time::OffsetDateTime;

#[cfg(not(target_arch = "wasm32"))]
mod tracer_thread;

#[cfg(not(target_arch = "wasm32"))]
pub use tracer_thread::TracerThread;
pub use tracing_core::Level;

#[derive(Clone, Debug)]
pub enum InstrumentationScope {
    Root,
    Child { parent_span_id: FunctionId },
}

#[derive(Clone)]
pub struct TraceContext {
    /// The scope used for all spans/logs within this context.
    pub scope: InstrumentationScope,
    /// The channel used to send trace events to the trace agent.
    pub tx: tokio::sync::mpsc::UnboundedSender<Arc<LogEvent>>,
    pub tags: TraceTags,
}

impl TraceContext {
    fn child_ctx(&self) -> (Self, FunctionId) {
        let new_uuid = format!("span_{}", uuid::Uuid::now_v7());
        let span_id = match &self.scope {
            InstrumentationScope::Root => FunctionId(new_uuid),
            InstrumentationScope::Child { parent_span_id } => {
                let mut new_parent = parent_span_id.clone();
                new_parent.0.push_str(new_uuid.as_str());
                new_parent
            }
        };
        (
            Self {
                scope: InstrumentationScope::Child {
                    parent_span_id: span_id.clone(),
                },
                tx: self.tx.clone(),
                tags: self.tags.clone(),
            },
            span_id,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
tokio::task_local! {
  pub static BAML_TRACE_CTX: TraceContext;
}
#[cfg(target_arch = "wasm32")]
thread_local! {
  pub static BAML_TRACE_CTX: TraceContext = TraceContext {
    scope: InstrumentationScope::Root,
    tx: tokio::sync::mpsc::unbounded_channel().0,
    tags: serde_json::Map::new(),
  };
}
// -------------------------------------------------------------------------------------------------

// impl TraceSpanStart {
//     pub fn new(
//         verbosity: tracing_core::Level,
//         callsite: String,
//         fields: serde_json::Value,
//     ) -> Self {
//         Self {
//             span_id: SpanId(vec![format!("span_{}", uuid::Uuid::now_v7())]),
//             start_time: web_time::Instant::now(),
//             meta: TraceMetadata {
//                 callsite,
//                 verbosity,
//             },
//             fields: match fields {
//                 serde_json::Value::Object(o) => o,
//                 _ => serde_json::Map::new(),
//             },
//         }
//     }
// }

/// In the new scheme, a basic log event is created via LogEventContent::Log. (Ensure that your
/// `baml-types` crate now defines an appropriate variant; for example:
///
///   pub enum LogEventContent {
///       Log { msg: String },
///       FunctionStart(FunctionStart),
///       FunctionEnd(FunctionEnd),
///       ... // etc.
///   }
///
/// If not, adjust this implementation accordingly.
pub fn log(
    verbosity: tracing_core::Level,
    callsite: String,
    msg: String,
    fields: serde_json::Value,
) {
    // Try to grab the current trace context; if unavailable bail out.
    let Ok(ctx) = BAML_TRACE_CTX.try_with(|ctx: &TraceContext| ctx.clone()) else {
        return;
    };

    let mut tags = ctx.tags.clone();
    if let serde_json::Value::Object(o) = fields {
        tags.extend(o);
    }

    // Determine span ID based on the current instrumentation scope.
    let span_id = match ctx.scope {
        InstrumentationScope::Root => FunctionId("".to_string()),
        InstrumentationScope::Child { parent_span_id } => parent_span_id,
    };

    // Wrap the log event in an Arc so that we have only one copy.
    let log_event = Arc::new(LogEvent {
        span_id,
        // Using an empty content span id here; adjust as needed.
        content_span_id: ContentId("".to_string()),
        span_chain: Vec::new(),
        timestamp: web_time::Instant::now(),
        content: LogEventContent::LogMessage { msg },
        tags,
    });

    // Send a clone of the Arc to the channel.
    let _ = ctx.tx.send(Arc::clone(&log_event));

    // Also store the event in the global storage.
    // Because 'put' is synchronous yet locking is async, we use a runtime to block.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(async {
        let mut storage = baml_types::tracing::storage::GLOBAL_TRACE_STORAGE
            .lock()
            .await;
        storage.put(Arc::clone(&log_event));
    });
}

/// This macro instruments a synchronous function call by sending "span start" and "span end"
/// events using the new LogEvent type. Note that we now use `FunctionStart` and `FunctionEnd`
/// (which live in baml-types) instead of the old TraceSpanStart/TraceSpanEnd.
macro_rules! impl_trace_scope {
    ($new_ctx:ident, $verbosity:ident, $name:ident, $fields:ident, $wrapped_fn:expr, $unwrapped_fn:expr, $then:expr) => {{
        let curr_ctx = BAML_TRACE_CTX.try_with(|ctx| ctx.clone());
        match curr_ctx {
            Ok(ctx) => {
                let ($new_ctx, span_id) = ctx.child_ctx();

                let name = $name.into();
                let start_time = OffsetDateTime::now_utc();

                // Send a span start event.
                let tags = $new_ctx.tags.clone();

                let start_event = LogEvent {
                    span_id: span_id.clone(),
                    content_span_id: ContentId("".to_string()),
                    span_chain: Vec::new(),
                    timestamp: start_time,
                    content: LogEventContent::FunctionStart(FunctionStart {
                        name: name.clone(),
                        // No arguments are provided in this context.
                        args: Vec::new(),
                        // Default options; adjust if you want to pass extra data.
                        options: BamlOptions {
                            type_builder: None,
                            client_registry: None,
                        },
                    }),
                    tags: {
                        let mut fields_map = $new_ctx.tags.clone();
                        if let serde_json::Value::Object(o) = $fields {
                            fields_map.extend(o);
                        }
                        fields_map
                    },
                };
                let _ = ctx.tx.send(Arc::new(start_event));

                let retval = $wrapped_fn;

                // Send a span end event.
                let end_event = LogEvent {
                    span_id,
                    content_span_id: ContentId("".to_string()),
                    span_chain: Vec::new(),
                    timestamp: OffsetDateTime::now_utc(),
                    content: LogEventContent::FunctionEnd(FunctionEnd {
                        // Because we cannot (in general) convert the return value to a BamlValue,
                        // we use a placeholder. You might convert `retval` if you require this.
                        result: Ok(BamlValue::String("".to_string())),
                    }),
                    tags: {
                        let mut fields = tags;
                        match $then(&retval) {
                            serde_json::Value::Object(o) => fields.extend(o),
                            _ => (),
                        }
                        fields
                    },
                };
                let _ = ctx.tx.send(Arc::new(end_event));
                retval
            }
            Err(_) => $unwrapped_fn,
        }
    }};
}

/// Instruments a synchronous function call with tracing.
pub fn btrace<F, R, G>(
    verbosity: tracing_core::Level,
    name: impl Into<String>,
    fields: serde_json::Value,
    f: F,
    then: G,
) -> R
where
    F: FnOnce() -> R,
    G: FnOnce(&R) -> serde_json::Value,
{
    impl_trace_scope!(
        new_ctx,
        verbosity,
        name,
        fields,
        BAML_TRACE_CTX.sync_scope(new_ctx, f),
        f(),
        then
    )
}

/// A trait to add a trace–aware method to futures.
pub trait WithTraceContext: Sized + std::future::Future {
    #[allow(async_fn_in_trait)]
    async fn btrace<F>(
        self,
        verbosity: tracing_core::Level,
        name: impl Into<String>,
        fields: serde_json::Value,
        then: F,
    ) -> <Self as std::future::Future>::Output
    where
        F: FnOnce(&<Self as std::future::Future>::Output) -> serde_json::Value,
    {
        impl_trace_scope!(
            new_ctx,
            verbosity,
            name,
            fields,
            BAML_TRACE_CTX.scope(new_ctx, self).await,
            self.await,
            then
        )
    }
}

// Auto-implement the trait for all futures.
impl<F> WithTraceContext for F where F: std::future::Future {}
