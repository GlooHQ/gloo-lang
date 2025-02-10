use anyhow::{Context, Result};
use std::pin::pin;

use std::time::Instant;

use super::TraceEvent;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::UnboundedReceiverStream;
pub struct TracerThread {
    rx: UnboundedReceiverStream<TraceEvent>,
}

impl TracerThread {
    pub fn new(rx: tokio::sync::mpsc::UnboundedReceiver<TraceEvent>) -> Self {
        Self {
            rx: UnboundedReceiverStream::new(rx),
        }
    }

    pub fn run(rx: tokio::sync::mpsc::UnboundedReceiver<TraceEvent>) {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(
                Self {
                    rx: UnboundedReceiverStream::new(rx),
                }
                .run_impl(),
            );
        });
    }

    pub async fn run_impl(self) {
        let mut stream = pin!(self
            .rx
            .chunks_timeout(1024, std::time::Duration::from_secs(5)));

        while let Some(events) = stream.next().await {
            let batch = TraceEventBatch { events };
            let body = serde_json::to_string(&batch).unwrap();

            loop {
                // TODO: this impl is wrong, every time a batch of trace events is ready,
                // we should enqueue it for send, instead of blocking on send before processing the next batch
                let Ok(presigned_url) = get_presigned_url(
                    "https://7ddxr6jp5gmzdu44srm2jszaq40mbazu.lambda-url.us-east-1.on.aws/",
                    "sam-boundary-studio-traceproce-tracebucket53be43bb-ow7chhxvu0ek",
                    "raw-events",
                )
                .await
                else {
                    continue;
                };

                // send the payload out

                let Ok(_) = upload_file(&presigned_url, &body).await else {
                    continue;
                };
                break;
            }
        }

        log::debug!("Trace upload complete");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetSignedUrlRequest {
    bucket: String,
    key: String,
}

// TODO: responses can be errors
#[derive(Debug, Serialize, Deserialize)]
struct GetSignedUrlResponse {
    upload_url: String,
    // generation_time_ms: u64,
}

async fn get_presigned_url(
    lambda_url: &str,
    bucket: &str,
    key_prefix: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let start = Instant::now();

    let request_body = GetSignedUrlRequest {
        bucket: bucket.to_string(),
        key: format!("{}/{}.json", key_prefix, uuid::Uuid::new_v4()),
    };

    eprintln!("Making POST request to URL: {}", lambda_url);
    eprintln!(
        "Request body: {}",
        serde_json::to_string_pretty(&request_body)?
    );

    let response = client
        .post(lambda_url)
        .json(&request_body)
        .send()
        .await
        .context("Failed to call lambda function")?;

    let duration_ms = start.elapsed().as_millis();
    eprintln!("Got response in {}ms", duration_ms);
    eprintln!("Response status: {}", response.status());
    eprintln!("Response headers: {:#?}", response.headers());

    // Get response body as text first
    let response_text = response
        .text()
        .await
        .context("Failed to get response body as text")?;

    // Try to parse the JSON, if it fails show the raw response
    let response_data: GetSignedUrlResponse =
        serde_json::from_str(&response_text).with_context(|| {
            format!(
                "Failed to parse response as GetSignedUrlResponse. Raw response: {}",
                response_text
            )
        })?;

    eprintln!("\nParsed response: {:?}", response_data);

    if response_data.upload_url.is_empty() {
        return Err(anyhow::anyhow!("Lambda returned empty upload URL"));
    }

    Ok(response_data.upload_url)
}

async fn upload_file(presigned_url: &str, body: &str) -> Result<()> {
    let start = Instant::now();

    let client = reqwest::Client::new();
    let response = client
        .put(presigned_url)
        .body(body.to_string())
        .send()
        .await
        .context("Failed to upload file")?;

    let duration_ms = start.elapsed().as_millis();
    log::info!(
        "Upload completed with status {} in {}ms",
        response.status(),
        duration_ms
    );

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct TraceEventBatch {
    events: Vec<TraceEvent>,
}
