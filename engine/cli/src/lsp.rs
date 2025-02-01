use std::num::NonZeroUsize;

use anyhow::Result;
use clap::Args;
use language_server::Server;

#[derive(Args, Debug)]
pub struct LanguageServerArgs {
    #[arg(
        long,
        help = "port to expose language server on",
        default_value = "2025"
    )]
    port: u16,
}

impl LanguageServerArgs {
    pub fn run(&self) -> Result<()> {
        run_server(NonZeroUsize::new(4).unwrap(), None)
    }
}

pub(crate) fn run_server(worker_threads: NonZeroUsize, preview: Option<bool>) -> Result<()> {
    let server = Server::new(worker_threads, preview)?;

    server.run()
}
