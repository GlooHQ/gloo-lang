use std::sync::Arc;

use napi::{
    bindgen_prelude::{Function, ObjectFinalize, PromiseRaw},
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Env, JsUndefined, Result as NapiResult,
};
use napi_derive::napi;

use tokio::sync::Mutex;

// Replace these with your actual imports/types:
use super::function_results::FunctionResult;
use super::runtime_ctx_manager::RuntimeContextManager;
use crate::errors::from_anyhow_error;
use baml_runtime;

/// A struct exposed to JS.  
/// We store an Arc<Mutex<...>> for concurrency, and an optional Arc<ThreadsafeFunction<...>> for callbacks.
#[napi(custom_finalize)]
pub struct FunctionResultStream {
    /// The underlying native handle
    inner: Arc<Mutex<baml_runtime::FunctionResultStream>>,

    /// If set, a JS callback `(err: any, param: FunctionResult) => void`.
    /// We wrap in `Arc` to allow `.clone()`.
    callback: Option<Arc<ThreadsafeFunction<FunctionResult, JsUndefined, FunctionResult, false>>>,

    /// Additional fields you need:
    tb: Option<baml_runtime::type_builder::TypeBuilder>,
    cb: Option<baml_runtime::client_registry::ClientRegistry>,
}

impl FunctionResultStream {
    /// Plain Rust constructor
    pub fn new(
        native: baml_runtime::FunctionResultStream,
        tb: Option<baml_runtime::type_builder::TypeBuilder>,
        cb: Option<baml_runtime::client_registry::ClientRegistry>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(native)),
            callback: None,
            tb,
            cb,
        }
    }
}

#[napi]
impl FunctionResultStream {
    /// Let JS code set a callback for “events”.  
    /// In TS: `(err: any, param: FunctionResult) => void`
    #[napi]
    pub fn on_event(
        &mut self,
        env: Env,
        #[napi(ts_arg_type = "(err: any, param: FunctionResult) => void")] func: Function<
            FunctionResult,
            JsUndefined,
        >,
    ) -> NapiResult<JsUndefined> {
        // Clear any old callback
        self.callback = None;

        // Build a TSFN from the JS function.
        // By default, that yields `ThreadsafeFunction<FunctionResult, JsUndefined, FunctionResult, false>`.
        let tsfn = func.build_threadsafe_function().build()?;

        // Store it, wrapped in Arc so we can clone it
        self.callback = Some(Arc::new(tsfn));

        env.get_undefined()
    }

    /// Complete the stream. Returns a `Promise<FunctionResult>` to JS.
    /// Meanwhile, if the callback is set, we invoke it with intermediate events.
    #[napi(ts_return_type = "Promise<FunctionResult>")]
    pub fn done(
        &self,
        env: Env,
        rctx: &RuntimeContextManager,
    ) -> NapiResult<PromiseRaw<FunctionResult>> {
        // **Clone** everything we need into `'static` data (Arc clones, etc.) so
        // the future doesn't borrow local references that would outlive `env`.
        let inner = self.inner.clone();
        let tb = self.tb.clone();
        let cb = self.cb.clone();
        let ctx_mng = rctx.inner.clone();
        let callback_opt = self.callback.clone();

        // Build an async future that references only `'static` arcs, not `env`.
        let fut = async move {
            let mut guard = inner.lock().await;

            // If we have a callback, define how to handle events:
            let on_event = callback_opt.map(|tsfn_arc| {
                move |native_event: baml_runtime::FunctionResult| {
                    // Convert the native event to your local `FunctionResult`.
                    // If there's an error type, you'd pass `Err(...)` instead of `Ok(...)`.
                    let status = tsfn_arc.call(
                        FunctionResult::from(native_event),
                        ThreadsafeFunctionCallMode::NonBlocking,
                    );
                    if status != napi::Status::Ok {
                        log::error!("Failed to call JS callback: {:?}", status);
                    }
                }
            });

            // Execute the native `.run(...)`, passing your `on_event` closure
            let result = guard
                .run(on_event, &ctx_mng, tb.as_ref(), cb.as_ref())
                .await;

            // Suppose `result.0` is `Result<baml_runtime::FunctionResult, anyhow::Error>`.
            // Convert success to `FunctionResult`, or produce a JS error.
            result
                .0
                .map(FunctionResult::from)
                .map_err(from_anyhow_error)
        };

        // Spawn the future in Node's event loop, returning a `PromiseRaw<FunctionResult>`.
        // *We do NOT capture `env` in `fut`, so there's no lifetime conflict.*
        env.spawn_future(fut)
        // env.spawn_future(fut)
    }
}

/// Called when JS garbage-collects the object.  
impl napi::bindgen_prelude::ObjectFinalize for FunctionResultStream {
    fn finalize(mut self, _env: Env) -> NapiResult<()> {
        // Drop the TSFN so no more events can happen
        self.callback.take();
        Ok(())
    }
}
