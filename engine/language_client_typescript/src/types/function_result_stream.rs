use std::sync::Arc;

use napi::{
    bindgen_prelude::{Function, ObjectFinalize, PromiseRaw},
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Env, JsObject, JsUndefined, JsUnknown, Result as NapiResult,
};
use napi_derive::napi;

use tokio::sync::Mutex;

// Replace these with your actual imports/types:
use super::function_results::FunctionResult;
use super::runtime_ctx_manager::RuntimeContextManager;
use crate::errors::from_anyhow_error;
use baml_runtime;

/// A struct exposed to JS with an optional multi-event callback (`ThreadsafeFunction`).
#[napi(custom_finalize)]
pub struct FunctionResultStream {
    // Native object in Arc<Mutex<...>> for async usage
    inner: Arc<Mutex<baml_runtime::FunctionResultStream>>,

    // If multiple events are needed, store a `ThreadsafeFunction<T>` in an Arc
    // so we can call it repeatedly from different threads or during async.
    callback: Option<Arc<ThreadsafeFunction<FunctionResult, JsUndefined, FunctionResult, false>>>,

    tb: Option<baml_runtime::type_builder::TypeBuilder>,
    cb: Option<baml_runtime::client_registry::ClientRegistry>,
}

impl FunctionResultStream {
    /// Plain Rust constructor, not directly exposed to JS.
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
    /// Let JS provide a callback `(err: any, param: FunctionResult) => void` for repeated “events”.
    #[napi]
    pub fn on_event(
        &mut self,
        env: Env,
        #[napi(ts_arg_type = "(err: any, param: FunctionResult) => void")] func: Function<
            FunctionResult,
            JsUndefined,
        >,
    ) -> NapiResult<JsUndefined> {
        // Clear any existing callback
        self.callback = None;

        // Build the threadsafe function
        let tsfn = func.build_threadsafe_function().build()?;
        // Wrap in Arc so we can clone it in async code
        self.callback = Some(Arc::new(tsfn));

        env.get_undefined()
    }

    /// Complete the stream.  
    /// Returns a JavaScript Promise<FunctionResult> (so from JS: `await stream.done()`).
    ///
    /// We do **not** return `PromiseRaw<'env, FunctionResult>` directly, because
    /// that often triggers lifetime errors in procedural macros.
    /// Instead, we return a `JsObject` that is a Promise, after converting
    /// the `PromiseRaw` with `into_js_value(...)`.
    #[napi(ts_return_type = "Promise<FunctionResult>")]
    pub fn done(
        &self,
        env: &Env, // <-- Pass `&Env`
        rctx: &RuntimeContextManager,
    ) -> NapiResult<PromiseRaw<FunctionResult>> {
        // Clone everything into `'static` arcs for async
        let inner = self.inner.clone();
        let callback_opt = self.callback.clone();
        let tb = self.tb.clone();
        let cb = self.cb.clone();
        let ctx_mng = rctx.inner.clone();

        // Build an async future that references only arcs
        let fut = async move {
            // Acquire the lock
            let mut guard = inner.lock().await;

            // If we have a TSFN, define how to handle each event
            let on_event = callback_opt.map(|tsfn_arc| {
                move |native_result: baml_runtime::FunctionResult| {
                    // Convert native type to your `FunctionResult` and pass to TSFN
                    let status = tsfn_arc.call(
                        Ok(FunctionResult::from(native_result)),
                        ThreadsafeFunctionCallMode::NonBlocking,
                    );
                    if status != napi::Status::Ok {
                        log::error!("Failed to call on_event callback: {:?}", status);
                    }
                }
            });

            // Call your native `run()`, presumably returning `(Result<..., E>, maybe_other)`
            let result = guard
                .run(on_event, &ctx_mng, tb.as_ref(), cb.as_ref())
                .await;

            // Convert success to `FunctionResult`, or produce a JS error
            result
                .0
                .map(FunctionResult::from)
                .map_err(from_anyhow_error)
        };

        // Return a `PromiseRaw<FunctionResult>` that the macro will turn into a JS Promise
        env.spawn_future(fut)
    }
}
/// Clean up references on JS garbage-collection
impl ObjectFinalize for FunctionResultStream {
    fn finalize(mut self, _env: Env) -> NapiResult<()> {
        // Drop the TSFN
        self.callback.take();
        Ok(())
    }
}
