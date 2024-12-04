use anyhow::Result;
use indexmap::IndexMap;
use internal_baml_core::ir::repr::IntermediateRepr;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use crate::GeneratorArgs;

pub(super) enum RemoveDirBehavior {
    /// Refuse to overwrite files that BAML did not generate
    ///
    /// This is the default
    Safe,

    /// Allow overwriting files that BAML did not generate
    ///
    /// Used by OpenAPI codegen, which runs openapi-generator and creates all sorts
    /// of files that BAML can't know about in advance
    Unsafe,
}

/// Controls output-type-specific behavior of codegen
pub(super) trait LanguageFeatures {
    const CONTENT_PREFIX: &'static str;

    fn content_prefix(&self) -> &'static str {
        Self::CONTENT_PREFIX.trim()
    }

    const REMOVE_DIR_BEHAVIOR: RemoveDirBehavior = RemoveDirBehavior::Safe;

    /// If set, the contents of a .gitignore file to be written to the generated baml_client
    ///
    /// It's only safe to set this for rest/openapi right now - still need to work out
    /// backwards compat implications for the other generators
    const GITIGNORE: Option<&'static str> = None;
}

pub(super) struct FileCollector<L: LanguageFeatures + Default> {
    // map of path to a an object that has the trail File
    files: IndexMap<PathBuf, String>,

    lang: L,
}

fn try_delete_tmp_dir(temp_path: &Path) -> Result<()> {
    // if the .tmp dir exists, delete it so we can get back to a working state without user intervention.
    let delete_attempts = 3; // Number of attempts to delete the directory
    let attempt_interval = Duration::from_millis(200); // Wait time between attempts

    for attempt in 1..=delete_attempts {
        if temp_path.exists() {
            match std::fs::remove_dir_all(temp_path) {
                Ok(_) => {
                    log::debug!("Temp directory successfully removed.");
                    break; // Exit loop after successful deletion
                }
                Err(e) if e.kind() == ErrorKind::Other && attempt < delete_attempts => {
                    log::warn!(
                        "Attempt {}: Failed to delete temp directory: {}",
                        attempt,
                        e
                    );
                    sleep(attempt_interval); // Wait before retrying
                }
                Err(e) => {
                    // For other errors or if it's the last attempt, fail with an error
                    return Err(anyhow::Error::new(e).context(format!(
                        "Failed to delete temp directory '{:?}' after {} attempts",
                        temp_path, attempt
                    )));
                }
            }
        } else {
            break;
        }
    }

    if temp_path.exists() {
        // If the directory still exists after the loop, return an error
        anyhow::bail!(
            "Failed to delete existing temp directory '{:?}' within the timeout",
            temp_path
        );
    }
    Ok(())
}

impl<L: LanguageFeatures + Default> FileCollector<L> {
    pub(super) fn new() -> Self {
        Self {
            files: IndexMap::new(),
            lang: L::default(),
        }
    }

    pub(super) fn add_template<
        'ir,
        V: TryFrom<(&'ir IntermediateRepr, &'ir GeneratorArgs), Error = anyhow::Error>
            + askama::Template,
    >(
        &mut self,
        name: impl Into<PathBuf> + std::fmt::Display,
        args: (&'ir IntermediateRepr, &'ir GeneratorArgs),
    ) -> Result<()> {
        let rendered = V::try_from(args)
            .map_err(|e| e.context(format!("Error while building {}", name)))?
            .render()
            .map_err(|e| {
                anyhow::Error::from(e).context(format!("Error while rendering {}", name))
            })?;
        self.files.insert(
            name.into(),
            format!("{}\n{}", self.lang.content_prefix(), rendered),
        );
        Ok(())
    }

    pub(super) fn add_file<K: AsRef<str>, V: AsRef<str>>(&mut self, name: K, contents: V) {
        self.files.insert(
            PathBuf::from(name.as_ref()),
            format!("{}\n{}", self.lang.content_prefix(), contents.as_ref()),
        );
    }

    /// Ensure that a directory contains only files we generated before nuking it.
    ///
    /// This is a safety measure to prevent accidentally deleting user files.
    ///
    /// We consider a file to be "generated by BAML" if it contains "generated by BAML"
    /// in the first 1024 bytes, and limit our search to a max of N unrecognized files.
    /// This gives us performance bounds if, for example, we find ourselves iterating
    /// through node_modules or .pycache or some other thing.
    fn remove_dir_safe(&self, output_path: &Path) -> Result<()> {
        if !output_path.exists() {
            return Ok(());
        }

        const MAX_UNKNOWN_FILES: usize = 4;
        let mut unknown_files = vec![];
        for entry in walkdir::WalkDir::new(output_path)
            .into_iter()
            .filter_entry(|e| e.path().file_name().is_some_and(|f| f != "__pycache__"))
        {
            if unknown_files.len() > MAX_UNKNOWN_FILES {
                break;
            }
            let entry = entry?;
            if entry.file_type().is_dir() {
                // Only files matter for the pre-existence check
                continue;
            }
            let path = entry.path();
            if let Ok(mut f) = std::fs::File::open(path) {
                use std::io::Read;
                let mut buf = [0; 1024];
                if f.read(&mut buf).is_ok()
                    && String::from_utf8_lossy(&buf).contains("generated by BAML")
                {
                    continue;
                }
            }
            let path = path.strip_prefix(output_path)?.to_path_buf();
            unknown_files.push(path);
        }
        unknown_files.sort();
        match L::REMOVE_DIR_BEHAVIOR {
            RemoveDirBehavior::Safe => match unknown_files.len() {
                0 => (),
                1 => anyhow::bail!(
                    "output directory contains a file that BAML did not generate\n\n\
                Please remove it and re-run codegen.\n\n\
                File: {}",
                    output_path.join(&unknown_files[0]).display()
                ),
                n => {
                    if n < MAX_UNKNOWN_FILES {
                        anyhow::bail!(
                            "output directory contains {n} files that BAML did not generate\n\n\
                    Please remove them and re-run codegen.\n\n\
                    Files:\n{}",
                            unknown_files
                                .iter()
                                .map(|p| format!("  - {}", output_path.join(p).display()))
                                .collect::<Vec<_>>()
                                .join("\n")
                        )
                    } else {
                        anyhow::bail!(
                        "output directory contains at least {n} files that BAML did not generate\n\n\
                    Please remove all files not generated by BAML and re-run codegen.\n\n\
                    Files:\n{}",
                        unknown_files
                            .iter()
                            .map(|p| format!("  - {}", output_path.join(p).display()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    }
                }
            },
            RemoveDirBehavior::Unsafe => {}
        }
        std::fs::remove_dir_all(output_path)?;
        Ok(())
    }

    /// Commit the generated files to disk.
    ///
    /// Writes to the output path, and returns a map of the paths to the contents.
    /// Ensures that we don't stomp on user files.
    ///
    /// `output_path` is the path to be written to, and the path that will be prepended
    /// to the returned file entries
    pub(super) fn commit(&mut self, output_path: &Path) -> Result<IndexMap<PathBuf, String>> {
        if let Some(gitignore) = L::GITIGNORE {
            self.files.insert(
                PathBuf::from(".gitignore"),
                format!("{}\n", gitignore.trim_start()),
            );
        }

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                log::debug!("Committing generated files in wasm is a no-op (writing is the Nodejs caller's responsibility)");
            } else {
                log::debug!("Writing files to {}", output_path.display());

                let temp_path = PathBuf::from(format!("{}.tmp", output_path.display()));

                // if the .tmp dir exists, delete it so we can get back to a working state without user intervention.
                try_delete_tmp_dir(temp_path.as_path())?;

                // Sort the files by path so that we always write to the same file
                for (relative_file_path, contents) in self.files.iter() {
                    let full_file_path = temp_path.join(relative_file_path);
                    if let Some(parent) = full_file_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&full_file_path, contents)?;
                }

                self.remove_dir_safe(output_path)?;
                std::fs::rename(&temp_path, output_path)?;

                log::debug!(
                    "Wrote {} files to {}",
                    self.files.len(),
                    output_path.display()
                );
            }
        }

        Ok(self.files.clone())
    }
}
