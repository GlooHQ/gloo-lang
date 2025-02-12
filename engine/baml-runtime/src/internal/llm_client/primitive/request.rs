use std::collections::HashMap;

use anyhow::{Context, Result};
use baml_types::BamlMap;
use internal_baml_jinja::RenderedChatMessage;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::internal::llm_client::{traits::WithClient, ErrorCode, LLMErrorResponse, LLMResponse};

pub trait RequestBuilder {
    #[allow(async_fn_in_trait)]
    async fn build_request(
        &self,
        prompt: either::Either<&String, &[RenderedChatMessage]>,
        allow_proxy: bool,
        stream: bool,
        expose_secrets: bool,
    ) -> Result<reqwest::RequestBuilder>;

    fn request_options(&self) -> &BamlMap<String, serde_json::Value>;

    fn http_client(&self) -> &reqwest::Client;
}

fn to_prompt(
    prompt: either::Either<&String, &[RenderedChatMessage]>,
) -> internal_baml_jinja::RenderedPrompt {
    match prompt {
        either::Left(prompt) => internal_baml_jinja::RenderedPrompt::Completion(prompt.clone()),
        either::Right(prompt) => internal_baml_jinja::RenderedPrompt::Chat(prompt.to_vec()),
    }
}

pub async fn make_request(
    client: &(impl WithClient + RequestBuilder),
    prompt: either::Either<&String, &[RenderedChatMessage]>,
    stream: bool,
) -> Result<(Response, web_time::SystemTime, web_time::Instant), LLMResponse> {
    let (system_now, instant_now) = (web_time::SystemTime::now(), web_time::Instant::now());

    let req = match client
        .build_request(prompt, true, stream, true)
        .await
        .context("Failed to build request")
    {
        Ok(req) => req,
        Err(e) => {
            return Err(LLMResponse::LLMFailure(LLMErrorResponse {
                client: client.context().name.to_string(),
                model: None,
                prompt: to_prompt(prompt),
                start_time: system_now,
                request_options: client.request_options().clone(),
                latency: instant_now.elapsed(),
                message: format!("{:#?}", e),
                code: ErrorCode::Other(2),
            }));
        }
    };

    let req = match req.build() {
        Ok(req) => req,
        Err(e) => {
            return Err(LLMResponse::LLMFailure(LLMErrorResponse {
                client: client.context().name.to_string(),
                model: None,
                prompt: to_prompt(prompt),
                start_time: system_now,
                request_options: client.request_options().clone(),
                latency: instant_now.elapsed(),
                message: format!("{:#?}", e),
                code: ErrorCode::Other(2),
            }));
        }
    };

    let response = match client.http_client().execute(req).await {
        Ok(response) => response,
        Err(e) => {
            return Err(LLMResponse::LLMFailure(LLMErrorResponse {
                client: client.context().name.to_string(),
                model: None,
                prompt: to_prompt(prompt),
                start_time: system_now,
                request_options: client.request_options().clone(),
                latency: instant_now.elapsed(),
                message: format!("{:?}", e),
                code: ErrorCode::Other(2),
            }));
        }
    };

    let status = response.status();
    if !status.is_success() {
        let url = response.url().to_string();
        let text = response.text().await.map_or_else(
            |_| "<no response>".to_string(),
            |text| {
                if text.is_empty() {
                    "<empty response>".to_string()
                } else {
                    text
                }
            },
        );
        return Err(LLMResponse::LLMFailure(LLMErrorResponse {
            client: client.context().name.to_string(),
            model: None,
            prompt: to_prompt(prompt),
            start_time: system_now,
            request_options: client.request_options().clone(),
            latency: instant_now.elapsed(),
            message: format!("Request failed: {}\n{}", url, text),
            code: ErrorCode::from_status(status),
        }));
    }

    Ok((response, system_now, instant_now))
}

pub async fn make_parsed_request<T: DeserializeOwned>(
    client: &(impl WithClient + RequestBuilder),
    prompt: either::Either<&String, &[RenderedChatMessage]>,
    stream: bool,
) -> Result<(T, web_time::SystemTime, web_time::Instant), LLMResponse> {
    let (response, system_now, instant_now) = make_request(client, prompt, stream).await?;
    let j = match response.json::<serde_json::Value>().await {
        Ok(response) => response,
        Err(e) => {
            return Err(LLMResponse::LLMFailure(LLMErrorResponse {
                client: client.context().name.to_string(),
                model: None,
                prompt: to_prompt(prompt),
                start_time: system_now,
                request_options: client.request_options().clone(),
                latency: instant_now.elapsed(),
                message: e.to_string(),
                code: ErrorCode::Other(2),
            }))
        }
    };

    match T::deserialize(&j).context(format!(
        "Failed to parse into a response accepted by {}: {}",
        std::any::type_name::<T>(),
        j
    )) {
        Ok(response) => Ok((response, system_now, instant_now)),
        Err(e) => Err(LLMResponse::LLMFailure(LLMErrorResponse {
            client: client.context().name.to_string(),
            model: None,
            prompt: to_prompt(prompt),
            start_time: system_now,
            request_options: client.request_options().clone(),
            latency: instant_now.elapsed(),
            message: format!("{:?}", e),
            code: ErrorCode::Other(2),
        })),
    }
}
