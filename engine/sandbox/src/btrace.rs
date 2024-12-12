#[derive(Clone)]
pub struct TraceContext {
    /// The scope used for all spans/logs within this context.
    pub scope: InstrumentationScope,
    /// The channel used to send trace events to the trace agent.
    pub tx: tokio::sync::mpsc::UnboundedSender<TraceEvent>,
}

tokio::task_local! {
  pub static BAML_TRACE_CTX: TraceContext;
}

struct TraceSpanBuilder {
    span_id: uuid::Uuid,
    start_time: std::time::Instant,
    meta: TraceMetadata,
    fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum InstrumentationScope {
    Root,
    Child { parent: uuid::Uuid },
}

#[derive(Debug)]
pub struct TraceMetadata {
    /// human-readable callsite identifier, e.g. "ExtractResume" or "openai/gpt-4o/chat"
    name: String,
    /// verbosity level
    verbosity: tracing_core::Level,
}

#[derive(Debug)]
pub struct TraceSpan {
    scope: InstrumentationScope,
    span_id: uuid::Uuid,
    start_time: web_time::Instant,
    duration: web_time::Duration,
    meta: TraceMetadata,
    fields: serde_json::Map<String, serde_json::Value>,
}

pub struct TraceLog {
    scope: InstrumentationScope,
    start_time: web_time::Instant,
    msg: String,
    meta: TraceMetadata,
    fields: serde_json::Map<String, serde_json::Value>,
}

pub enum TraceEvent {
    Span(TraceSpan),
    Log(TraceLog),
}

impl TraceSpanBuilder {
    pub fn start_span(name: String) -> Self {
        Self {
            span_id: uuid::Uuid::now_v7(),
            start_time: web_time::Instant::now(),
            meta: TraceMetadata {
                name,
                verbosity: tracing_core::Level::INFO,
            },
            fields: serde_json::Map::new(),
        }
    }

    pub fn end(self) -> TraceSpan {
        let duration = self.start_time.elapsed();
        let ctx = BAML_TRACE_CTX.get();
        let span = TraceSpan {
            scope: ctx.scope.clone(),
            span_id: self.span_id,
            start_time: self.start_time,
            duration,
            meta: self.meta,
            fields: self.fields,
        };
        span
    }
}

fn log(callsite: String, msg: String, fields: serde_json::Map<String, serde_json::Value>) {
    let Ok(ctx) = BAML_TRACE_CTX.try_with(|ctx| ctx.clone()) else {
        return;
    };
    let _ = ctx.tx.send(TraceEvent::Log(TraceLog {
        scope: ctx.scope.clone(),
        start_time: web_time::Instant::now(),
        msg,
        meta: TraceMetadata {
            name: callsite,
            verbosity: tracing_core::Level::INFO,
        },
        fields,
    }));
}

pub fn baml_trace_sync_scope<F, R>(name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let verbosity = tracing_core::Level::INFO;
    let curr_ctx = BAML_TRACE_CTX.try_with(|ctx| ctx.clone());

    match curr_ctx {
        Ok(ctx) => {
            let name = name.into();
            let span = TraceSpanBuilder::start_span(name.clone());
            println!(
                "entering span: {:?} -> {:?} {}",
                ctx.scope, span.span_id, name
            );
            let new_ctx = TraceContext {
                scope: InstrumentationScope::Child {
                    parent: span.span_id,
                },
                tx: ctx.tx.clone(),
            };
            let retval = BAML_TRACE_CTX.sync_scope(new_ctx, f);
            let span = span.end();
            println!(
                "exiting span: {:?} <- {:?} {}",
                ctx.scope, span.span_id, name
            );
            let _ = ctx.tx.send(TraceEvent::Span(span));
            retval
        }
        // We use TraceContext to propagate the tx channel to the trace agent, so if we have
        // no context, just run the future without tracing.
        Err(_) => f(),
    }
}

pub trait WithTraceContext: Sized + std::future::Future {
    async fn baml_traced(self, name: impl Into<String>) -> <Self as std::future::Future>::Output {
        let verbosity = tracing_core::Level::INFO;
        let curr_ctx = BAML_TRACE_CTX.try_with(|ctx| ctx.clone());

        match curr_ctx {
            Ok(ctx) => {
                let name = name.into();
                let span = TraceSpanBuilder::start_span(name.clone());
                println!(
                    "entering span: {:?} -> {:?} {}",
                    ctx.scope, span.span_id, name
                );
                let new_ctx = TraceContext {
                    scope: InstrumentationScope::Child {
                        parent: span.span_id,
                    },
                    tx: ctx.tx.clone(),
                };
                let retval = BAML_TRACE_CTX.scope(new_ctx, self).await;
                let span = span.end();
                println!(
                    "exiting span: {:?} <- {:?} {}",
                    ctx.scope, span.span_id, name
                );
                let _ = ctx.tx.send(TraceEvent::Span(span));
                retval
            }
            // We use TraceContext to propagate the tx channel to the trace agent, so if we have
            // no context, just run the future without tracing.
            Err(_) => self.await,
        }
    }
}

// Auto-implement the trait for all futures
impl<F> WithTraceContext for F where F: std::future::Future {}
