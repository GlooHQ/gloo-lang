use super::TraceEvent;
pub struct TracerThread {
    rx: tokio::sync::mpsc::UnboundedReceiver<TraceEvent>,
}

impl TracerThread {
    pub fn new(rx: tokio::sync::mpsc::UnboundedReceiver<TraceEvent>) -> Self {
        Self { rx }
    }

    pub fn run(rx: tokio::sync::mpsc::UnboundedReceiver<TraceEvent>) {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(Self { rx }.run_impl());
        });
    }

    pub async fn run_impl(&mut self) {
        while let Some(event) = self.rx.recv().await {
            match event {
                TraceEvent::SpanStart(span) => {
                    println!(
                        "--> [{} / {}]: {} {}",
                        span.span_id
                            .0
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(" > "),
                        span.meta.verbosity,
                        span.meta.callsite,
                        serde_json::to_string(&span.fields).unwrap_or("???".to_string()),
                    );
                }
                TraceEvent::SpanEnd(span) => {
                    println!(
                        "<-- [{} / {}]: {} {}",
                        span.span_id
                            .0
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(" > "),
                        span.meta.verbosity,
                        span.meta.callsite,
                        serde_json::to_string(&span.fields).unwrap_or("???".to_string()),
                    );
                }
                TraceEvent::Log(log) => {
                    println!(
                        "log [{} / {}]: {} {} {}",
                        log.span_id
                            .0
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(" > "),
                        log.meta.verbosity,
                        log.meta.callsite,
                        log.msg,
                        serde_json::to_string(&log.tags).unwrap_or("???".to_string()),
                    );
                }
            }
        }
    }
}
