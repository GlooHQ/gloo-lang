use std::sync::Arc;

use napi::{
    bindgen_prelude::{FnArgs, Function, ObjectFinalize, PromiseRaw},
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Env, JsObject, JsUndefined, JsUnknown, Ref, Result as NapiResult,
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
    // The callback signature: `(err: any, fr: FunctionResult) => void`
    callback: Option<Ref<Function<'static, (), ()>>>,

    // Additional fields from your code, if needed:
    tb: Option<baml_runtime::type_builder::TypeBuilder>,
    cb: Option<baml_runtime::client_registry::ClientRegistry>,
}

impl FunctionResultStream {
    /// Plain Rust constructor, not directly exposed to JS.
    pub fn new(
        inner: baml_runtime::FunctionResultStream,
        callback: Option<Ref<Function<'static, (), ()>>>,
        tb: Option<baml_runtime::type_builder::TypeBuilder>,
        cb: Option<baml_runtime::client_registry::ClientRegistry>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            callback: callback,
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
        #[napi(ts_arg_type = "(err: any, fr: FunctionResult) => void")] func: Function<
            'static,
            (),
            (),
        >,
    ) -> NapiResult<JsUndefined> {
        if let Some(mut old) = self.callback.take() {
            old.unref(&env)?;
        }
        // Store the new callback in a `Ref`, so it stays alive across async
        let new_ref = Ref::new(&env, &func)?;
        self.callback = Some(new_ref);

        env.get_undefined()
    }

    /// Complete the stream, returning a Promise<FunctionResult>.
    ///
    /// - We spawn an async future that calls `NativeStream::run()`.
    /// - If `callback` is set, we create a TSFN and pass events to JS.
    /// - The final result is returned to JS by resolving the Promise.
    #[napi(ts_return_type = "Promise<FunctionResult>")]
    pub fn done(
        &self,
        env: Env,
        rctx: &RuntimeContextManager,
    ) -> napi::Result<PromiseRaw<'_, FunctionResult>> {
        let inner = self.inner.clone();

        let on_event = match &self.callback {
            Some(cb) => {
                let cb = cb.get_value(&env)?; //env.get_reference_value(cb)?;
                let tsfn = cb.build_threadsafe_function().build()?;

                Some(move |event: baml_runtime::FunctionResult| {
                    let res = tsfn.call(
                        Ok(FunctionResult::from(event)),
                        ThreadsafeFunctionCallMode::Blocking,
                    );
                    if res != napi::Status::Ok {
                        log::error!("Error calling on_event callback: {:?}", res);
                    }
                })
            }
            None => None,
        };

        let ctx_mng = rctx.inner.clone();
        let tb = self.tb.clone();
        let cb = self.cb.clone();

        let fut = async move {
            let ctx_mng = ctx_mng;
            let res = inner
                .lock()
                .await
                .run(on_event, &ctx_mng, tb.as_ref(), cb.as_ref())
                .await;
            res.0.map(FunctionResult::from).map_err(from_anyhow_error)
        };

        // env.execute_tokio_future(fut, |&mut _, data| Ok(data))
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
