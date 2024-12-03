use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Args;
use internal_baml_core::internal_baml_schema_ast::{format_schema, FormatOptions};

#[derive(Args, Debug)]
pub struct FormatArgs {
    #[arg(long, help = "path/to/baml_src", default_value = "./baml_src")]
    pub from: PathBuf,
}

impl FormatArgs {
    pub fn run(&self) -> Result<()> {
        let source = fs::read_to_string(&self.from)?;
        let formatted = format_schema(
            &source,
            FormatOptions {
                indent_width: 4,
                fail_on_unhandled_rule: false,
            },
        )?;

        let mut to = self.from.clone();
        to.set_extension("formatted.baml");
        fs::write(&to, formatted)?;

        log::info!("Formatted {} to {}", self.from.display(), to.display());

        Ok(())
    }
}
