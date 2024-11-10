use anyhow::Result;
use baml_runtime::RuntimeCliDefaults;
use indicatif::MultiProgress;
use tracing_subscriber::{self, EnvFilter};

fn main() -> Result<()> {
    // Note we also check this in baml_runtime/src/internal/tracing/mod.rs
    let use_json = match std::env::var("BAML_LOG_JSON") {
        Ok(val) => val.trim().eq_ignore_ascii_case("true") || val.trim() == "1",
        Err(_) => false,
    };

    // Set up the logging subscriber based on the env var
    if use_json {
        // JSON formatting
        tracing_subscriber::fmt()
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .json()
            .with_env_filter(
                EnvFilter::try_from_env("BAML_LOG").unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .flatten_event(true)
            .with_current_span(false)
            .with_span_list(false)
            .init();
    } else {
        // Regular formatting with indicatif integration
        let logger = env_logger::Builder::from_env(
            env_logger::Env::new()
                .filter_or("BAML_LOG", "info")
                .write_style("BAML_LOG_STYLE"),
        )
        .build();
        let level = logger.filter();

        if let Err(e) =
            indicatif_log_bridge::LogWrapper::new(MultiProgress::new(), logger).try_init()
        {
            eprintln!("Failed to initialize BAML logger: {:#}", e);
        }
        log::set_max_level(level);
    }

    let argv: Vec<String> = std::env::args().collect();

    baml_cli::run_cli(
        argv,
        RuntimeCliDefaults {
            output_type: baml_types::GeneratorOutputType::OpenApi,
        },
    )?;

    Ok(())
}
