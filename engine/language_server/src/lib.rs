#![allow(dead_code)]

use anyhow::Context;
pub use edit::{DocumentKey, PositionEncoding, TextDocument};
pub use session::{ClientSettings, DocumentQuery, DocumentSnapshot, Session};
use std::num::NonZeroUsize;

use crate::server::Server;

#[macro_use]
mod message;

mod edit;
mod logging;
mod server;
mod session;
mod system;
mod tests;

// additional baml modules
mod baml_db;
mod baml_diagnostics;
mod baml_project;
mod baml_source_file;
mod baml_text_size;

pub(crate) const SERVER_NAME: &str = "red-knot";
pub(crate) const DIAGNOSTIC_NAME: &str = "Red Knot";

/// A common result type used in most cases where a
/// result type is needed.
pub(crate) type Result<T> = anyhow::Result<T>;

pub(crate) fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn run_server() -> anyhow::Result<()> {
    let four = NonZeroUsize::new(4).unwrap();

    // by default, we set the number of worker threads to `num_cpus`, with a maximum of 4.
    let worker_threads = std::thread::available_parallelism()
        .unwrap_or(four)
        .max(four);

    Server::new(worker_threads)
        .context("Failed to start server")?
        .run()?;

    Ok(())
}
