use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use indexmap::indexmap;
use napi::{
    bindgen_prelude::{Function, ObjectFinalize, PromiseRaw, Result as NapiResult},
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Env, JsObject, JsUndefined, Ref,
};
use napi_derive::napi;
use tokio::sync::Mutex;

use super::function_results::FunctionResult;
use super::runtime_ctx_manager::RuntimeContextManager;
use crate::errors::from_anyhow_error;
use baml_runtime::{self, internal::llm_client::orchestrator::OrchestrationScope};

/// A struct exposed to JS with an optional multi-event callback.
#[napi(custom_finalize)]
pub struct FunctionResultStream {
    /// The underlying native stream in Arc<Mutex<...>> so it can be used async.
    inner: Arc<Mutex<baml_runtime::FunctionResultStream>>,

    /// The callback signature. We store a `Ref` to the JS `Function`, but
    /// no lifetime param in `FunctionResultStream` itself.
    callback: Option<Ref<Function<'static, FunctionResult, ()>>>,

    tb: Option<baml_runtime::type_builder::TypeBuilder>,
    cb: Option<baml_runtime::client_registry::ClientRegistry>,
}

impl FunctionResultStream {
    /// Plain Rust constructor
    pub fn new(
        inner_stream: baml_runtime::FunctionResultStream,
        callback: Option<Ref<Function<'static, FunctionResult, ()>>>,
        tb: Option<baml_runtime::type_builder::TypeBuilder>,
        cb: Option<baml_runtime::client_registry::ClientRegistry>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner_stream)),
            callback,
            tb,
            cb,
        }
    }
}

#[napi]
impl FunctionResultStream {
    /// Let JS provide a callback `(err: any, fr: FunctionResult) => void` for repeated “events.”
    ///
    /// Notice we define the function type as `Function<'static, FunctionResult, ()>`.
    /// So from the Rust side, we pass `Ok(FunctionResult)` to the TSFN.
    #[napi]
    pub fn on_event(
        &mut self,
        env: Env,
        #[napi(ts_arg_type = "(err: any, fr: FunctionResult) => void")] func: Function<
            'static,
            FunctionResult,
            (),
        >,
    ) -> NapiResult<JsUndefined> {
        // If we already had a callback stored, unref it
        if let Some(mut old_cb) = self.callback.take() {
            old_cb.unref(&env)?;
        }
        // Store the new callback in a `Ref`, so it stays alive across async
        let new_ref = Ref::new(&env, &func)?;
        self.callback = Some(new_ref);
        env.get_undefined()
    }

    /// Complete the stream, returning a `Promise<FunctionResult>` in JS.
    ///
    /// - We spawn an async future that calls `inner.run(...)`.
    /// - If `callback` is set, we create a threadsafe function and pass events to JS.
    /// - The final result resolves the JS Promise with a `FunctionResult`.
    #[napi(ts_return_type = "Promise<FunctionResult>")]
    pub fn done(
        &self,
        env: Env,
        rctx: &RuntimeContextManager,
    ) -> NapiResult<PromiseRaw<'_, FunctionResult>> {
        let inner = self.inner.clone();

        // If a callback was set, prepare to build a TSFN that sends `FunctionResult` to JS
        // let on_event = match &self.callback {
        //     Some(ref_cb) => {
        //         // Get actual JS Function from the Ref
        //         let cb = ref_cb.get_value(&env)?;
        //         // Build a TSFN that takes `FunctionResult` -> calls JS side
        //         let tsfn = cb.build_threadsafe_function().build()?;
        //         Some(move |native_event: baml_runtime::FunctionResult| {
        //             // Convert or wrap the native_event as needed
        //             let fnRes = baml_runtime::FunctionResult::new(
        //                 OrchestrationScope { scope: vec![] },
        //                 baml_runtime::internal::llm_client::LLMResponse::Success(
        //                     baml_runtime::internal::llm_client::LLMCompleteResponse {
        //                         client: "test".to_string(),
        //                         model: "test".to_string(),
        //                         prompt: baml_runtime::RenderedPrompt::Chat(vec![]),
        //                         request_options: indexmap! {},
        //                         content: "test".to_string(),
        //                         start_time: SystemTime::now(),
        //                         latency: Duration::from_secs(0),
        //                         metadata: baml_runtime::internal::llm_client::LLMCompleteResponseMetadata {
        //                             baml_is_complete: true,
        //                             finish_reason: Some("test".to_string()),
        //                             prompt_tokens: Some(0),
        //                             output_tokens: Some(0),
        //                             total_tokens: Some(0),
        //                         },
        //                     },
        //                 ),
        //                 None,
        //                 None,
        //             );

        //             let result_data = FunctionResult::from(fnRes);
        //             let status = tsfn.call(result_data, ThreadsafeFunctionCallMode::Blocking);
        //             if status != napi::Status::Ok {
        //                 log::error!("Error calling on_event callback: {:?}", status);
        //             }
        //         })
        //     }
        //     None => None,
        // };

        let ctx_mng = rctx.inner.clone();
        let tb = self.tb.clone();
        let cb = self.cb.clone();

        // Build our async future
        let fut = async move {
            let mut guard = inner.lock().await;
            // let out = guard
            //     .run(on_event, &ctx_mng, tb.as_ref(), cb.as_ref())
            //     .await;
            // out.0.map(FunctionResult::from).map_err(from_anyhow_error)
            let fnRes = baml_runtime::FunctionResult::new(
                OrchestrationScope { scope: vec![] },
                baml_runtime::internal::llm_client::LLMResponse::Success(
                    baml_runtime::internal::llm_client::LLMCompleteResponse {
                        client: "test".to_string(),
                        model: "test".to_string(),
                        prompt: baml_runtime::RenderedPrompt::Chat(vec![]),
                        request_options: indexmap! {},
                        content: "test".to_string(),
                        start_time: SystemTime::now(),
                        latency: Duration::from_secs(0),
                        metadata: baml_runtime::internal::llm_client::LLMCompleteResponseMetadata {
                            baml_is_complete: true,
                            finish_reason: Some("test".to_string()),
                            prompt_tokens: Some(0),
                            output_tokens: Some(0),
                            total_tokens: Some(0),
                        },
                    },
                ),
                None,
                None,
            );

            let result_data = FunctionResult::from(fnRes);
            Ok(result_data)
        };

        // Turn future into a `Promise<FunctionResult>` for JS

        // env.spawn_future(fut)
        env.spawn_future_with_callback(fut, move |env, res| {
            // let reference = self.callback.as_ref().unwrap();
            // let cb = reference.get_value(&env)?;
            // cb.call(())?;
            // Ok(res)
            todo!()
        })
    }
}

/// Cleanup references on JS garbage‐collection
impl ObjectFinalize for FunctionResultStream {
    fn finalize(mut self, env: Env) -> NapiResult<()> {
        if let Some(mut cb) = self.callback.take() {
            cb.unref(&env)?;
        }
        Ok(())
    }
}
