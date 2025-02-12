pub(crate) mod api_client;
pub(crate) mod auth;
pub(crate) mod colordiff;
pub(crate) mod commands;
pub(crate) mod deploy;
pub(crate) mod format;
pub(crate) mod propelauth;
pub(crate) mod tui;

use anyhow::Result;
use clap::Parser;

pub fn run_cli(argv: Vec<String>, caller_type: baml_runtime::RuntimeCliDefaults) -> Result<()> {
    commands::RuntimeCli::parse_from(argv).run(caller_type)
}
