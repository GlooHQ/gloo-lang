#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms, unsafe_code)]
#![allow(clippy::derive_partial_eq_without_eq)]

pub use internal_baml_diagnostics;
pub use internal_baml_parser_database::{self};

pub use internal_baml_schema_ast::{self, ast};

use rayon::prelude::*;
use std::{path::PathBuf, sync::Mutex};

use internal_baml_diagnostics::{DatamodelError, Diagnostics, SourceFile, Span};

mod common;
pub mod configuration;
pub mod ir;
mod lockfile;
mod validate;

use self::validate::generator_loader;

pub use lockfile::LockfileVersion;

pub use crate::{
    common::{PreviewFeature, PreviewFeatures, ALL_PREVIEW_FEATURES},
    configuration::Configuration,
};

pub use lockfile::LockFileWrapper;

pub struct ValidatedSchema {
    pub db: internal_baml_parser_database::ParserDatabase,
    pub diagnostics: Diagnostics,
    pub configuration: Configuration,
}

impl std::fmt::Debug for ValidatedSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<Prisma schema>")
    }
}

/// The most general API for dealing with Prisma schemas. It accumulates what analysis and
/// validation information it can, and returns it along with any error and warning diagnostics.
pub fn validate(root_path: &PathBuf, files: Vec<SourceFile>) -> ValidatedSchema {
    let mut diagnostics = Diagnostics::new(root_path.clone());
    let mut db = internal_baml_parser_database::ParserDatabase::new();
    println!("validating!");
    {
        let diagnostics = Mutex::new(&mut diagnostics);
        let db = Mutex::new(&mut db);
        files.par_iter().for_each(|file| {
            match internal_baml_schema_ast::parse_schema(root_path, file) {
                Ok((ast, err)) => {
                    let mut diagnostics = diagnostics.lock().unwrap();
                    let mut db = db.lock().unwrap();
                    diagnostics.push(err);
                    db.add_ast(ast);
                }
                Err(err) => {
                    let mut diagnostics = diagnostics.lock().unwrap();
                    diagnostics.push(err);
                }
            }
        });
    }

    if let Err(d) = db.validate(&mut diagnostics) {
        return ValidatedSchema {
            db,
            diagnostics: d,
            configuration: Configuration::new(),
        };
    }

    let (configuration, diag) = validate_configuration(root_path, db.ast());
    diagnostics.push(diag);

    if diagnostics.has_errors() {
        return ValidatedSchema {
            db,
            diagnostics,
            configuration,
        };
    }

    // actually run the validation pipeline
    validate::validate(&db, configuration.preview_features(), &mut diagnostics);

    if diagnostics.has_errors() {
        return ValidatedSchema {
            db,
            diagnostics,
            configuration,
        };
    }

    // Some last linker stuff can only happen post validation.
    db.finalize(&mut diagnostics);

    ValidatedSchema {
        db,
        diagnostics,
        configuration,
    }
}

/// Loads all configuration blocks from a datamodel using the built-in source definitions.
pub fn parse_configuration(
    root_path: &PathBuf,
    main_schema: &SourceFile,
) -> Result<(Configuration, Diagnostics), Diagnostics> {
    let (ast, mut diagnostics) = internal_baml_schema_ast::parse_schema(root_path, main_schema)?;

    let (out, diag) = validate_configuration(root_path, &ast);
    diagnostics.push(diag);

    if out.generators.is_empty() {
        diagnostics.push_error(DatamodelError::new_validation_error(
            "No generator specified".into(),
            Span {
                file: main_schema.clone(),
                start: 0,
                end: 0,
            },
        ));
    }

    if diagnostics.has_errors() {
        return Err(diagnostics);
    }

    Ok((out, diagnostics))
}

fn validate_configuration(
    root_path: &PathBuf,
    schema_ast: &ast::SchemaAst,
    // skip_lock_file_validation: bool,
) -> (Configuration, Diagnostics) {
    let mut diagnostics = Diagnostics::new(root_path.clone());
    let generators = generator_loader::load_generators_from_ast(schema_ast, &mut diagnostics);

    let lock_files = generators
        .iter()
        .filter_map(
            |gen| match lockfile::LockFileWrapper::from_generator(&gen) {
                Ok(lock_file) => {
                    if let Ok(prev) =
                        lockfile::LockFileWrapper::from_path(gen.output_dir().join("baml.lock"))
                    {
                        lock_file.validate(&prev, &mut diagnostics);
                    }
                    Some((gen.clone(), lock_file))
                }
                Err(err) => {
                    diagnostics.push_error(DatamodelError::new_validation_error(
                        &format!("Failed to create lock file: {}", err),
                        gen.span.clone(),
                    ));
                    None
                }
            },
        )
        .collect();

    (
        Configuration {
            generators: lock_files,
        },
        diagnostics,
    )
}
