use baml_runtime::InternalRuntimeInterface;
use baml_runtime::{
    internal::llm_client::LLMResponse, BamlRuntime, DiagnosticsError, IRHelper, RenderedPrompt,
};
use baml_schema_build::runtime_wasm::{
    WasmDiagnosticError, WasmError, WasmFunction, WasmGeneratorConfig, WasmProject, WasmRuntime,
    WasmTestCase,
};
use baml_types::{BamlMediaType, BamlValue, GeneratorOutputType, TypeValue};
use file_utils::gather_files;
use indexmap::IndexMap;
use internal_baml_codegen::version_check::GeneratorType;
use internal_baml_codegen::version_check::{check_version, VersionCheckMode};
use lsp_types::{GotoDefinitionParams, Url};
use lsp_types::{
    Hover, HoverContents, HoverParams, LocationLink, Position, Range, TextDocumentItem,
};
use position_utils::get_word_at_position;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

pub mod file_utils;
pub mod metadata;
mod position_utils;
pub mod watch;

/// Native version of the project. This is similar to the WasmProject,
/// but uses native types instead of JS types.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectFiles {
    pub root_dir_name: String,
    // This is the version of the file on disk.
    files: HashMap<String, String>,
    // This is the version of the file that is currently being edited (unsaved changes)
    unsaved_files: HashMap<String, String>,
}

/// A diagnostic error structure to report errors along with a list of files.
#[derive(Clone, Debug)]
pub struct DiagnosticError {
    pub errors: DiagnosticsError,
    pub all_files: Vec<String>,
}

// impl Runtime {
//     /// Run the generators/ codegen using the provided files.
//     /// It converts the files (a HashMap of filename->contents) into the
//     /// required format and then calls `run_codegen` on the internal runtime.
//     pub fn run_generators(
//         &self,
//         input_files: &HashMap<String, String>,
//         no_version_check: bool,
//     ) -> Result<Vec<generator::GeneratorOutput>, Error> {
//         // Convert input_files from HashMap<String, String> to HashMap<PathBuf, String>
//         let files: HashMap<PathBuf, String> = input_files
//             .iter()
//             .map(|(k, v)| (PathBuf::from(k), v.clone()))
//             .collect();

//         // Call the codegen function on the BamlRuntime.
//         self.runtime
//             .run_codegen(&files, no_version_check)
//             .map(|outputs| outputs.into_iter().map(|g| g.into()).collect())
//     }
// }

impl ProjectFiles {
    /// Construct a new project from a root directory name and a mapping of files.
    pub fn new(root_dir_name: &str, files: HashMap<String, String>) -> Self {
        Self {
            root_dir_name: root_dir_name.to_string(),
            files,
            unsaved_files: HashMap::new(),
        }
    }

    /// Alternative constructor that takes owned parameters.
    pub fn create(root_dir_name: String, files: HashMap<String, String>) -> Self {
        Self {
            root_dir_name,
            files,
            unsaved_files: HashMap::new(),
        }
    }

    /// Returns a Vec of formatted strings each representing a file and its content.
    /// (Files are merged from saved and unsaved.)
    pub fn files(&self) -> Vec<String> {
        let mut merged_files = self.files.clone();
        merged_files.extend(self.unsaved_files.clone());
        merged_files
            .iter()
            .map(|(k, v)| format!("{}BAML_PATH_SPLTTER{}", k, v))
            .collect()
    }

    /// Update the saved content of a file. If `content` is `None`, the file is removed.
    pub fn update_file(&mut self, name: &str, content: Option<String>) {
        if let Some(content) = content {
            self.files.insert(name.to_string(), content);
        } else {
            self.files.remove(name);
        }
    }

    /// Saves a file’s contents (updating the saved files and removing any unsaved changes).
    pub fn save_file(&mut self, name: &str, content: &str) {
        self.files.insert(name.to_string(), content.to_string());
        self.unsaved_files.remove(name);
    }

    /// Sets the unsaved content for a file or removes it if `content` is `None`.
    pub fn set_unsaved_file(&mut self, name: &str, content: Option<String>) {
        if let Some(content) = content {
            self.unsaved_files.insert(name.to_string(), content);
        } else {
            self.unsaved_files.remove(name);
        }
    }

    /// Run the diagnostic process using the given runtime.
    /// The function combines saved and unsaved files as part of the diagnostic context.
    pub fn diagnostics(&self, rt: &BamlRuntime) -> DiagnosticError {
        let mut merged_files: HashMap<_, _> = self.files.iter().collect();
        merged_files.extend(self.unsaved_files.iter());

        DiagnosticError {
            errors: rt.internal().diagnostics().clone(),
            all_files: merged_files.keys().map(|s| s.to_string()).collect(),
        }
    }

    /// Create the runtime given a set of environment variables.
    /// All files (saved and unsaved) are combined into one HashMap.
    pub fn runtime(&self, env_vars: HashMap<String, String>) -> Result<BamlRuntime, anyhow::Error> {
        let mut merged_files = self.files.clone();
        merged_files.extend(self.unsaved_files.clone());

        BamlRuntime::from_file_content(&self.root_dir_name, &merged_files, env_vars)
    }

    /// Run the code generators using the files in the project.
    /// This function logs the files, creates a runtime with the provided
    /// environment variables, and then uses the runtime to run the generators.
    pub fn run_generators(
        &self,
        no_version_check: Option<bool>,
        env_vars: HashMap<String, String>,
    ) -> Result<Vec<generator::GeneratorOutput>, Error> {
        let no_version_check = no_version_check.unwrap_or(false);
        info!("Files are: {:#?}", self.files);
        let rt = self.runtime(env_vars)?;
        rt.run_generators(&self.files, no_version_check)
    }

    /// A native version to run generators.
    /// (In the wasm target this function was not available.)
    pub fn run_generators_native(
        &self,
        no_version_check: Option<bool>,
        env_vars: HashMap<String, String>,
    ) -> Result<Vec<generator::GeneratorOutput>, Error> {
        self.run_generators(no_version_check, env_vars)
    }
}

// --- Helper functions for working with text documents ---

/// Trims a given string by removing non-alphanumeric characters (besides underscores and periods).
pub fn trim_line(s: &str) -> String {
    s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
        .to_string()
}

/// The Project struct wraps a WASM project, its runtime, and exposes methods for file updates,
/// diagnostics, symbol lookup, and code generation.
#[derive(Clone)]
pub struct Project {
    wasm_project: WasmProject,
    // A callback invoked when a runtime update succeeds (passing diagnostics and a file map).
    // on_success: Box<dyn Fn(WasmDiagnosticError, HashMap<String, String>)>,
    current_runtime: Option<WasmRuntime>,
    last_successful_runtime: Option<WasmRuntime>,
}

impl Project {
    /// Creates a new `Project` instance.
    pub fn new(wasm_project: WasmProject) -> Self {
        Self {
            wasm_project,
            current_runtime: None,
            last_successful_runtime: None,
        }
    }

    pub fn reload_project_files(&mut self) {
        let root_dir = self.wasm_project.root_dir_name.clone();
        let root_path = std::path::Path::new(&root_dir);
        let files = match gather_files(root_path, false) {
            Ok(files) => files,
            Err(e) => {
                self.replace_all_files(WasmProject::create(root_dir.into(), HashMap::new()));
                tracing::error!("Error gathering files: {}", e);
                return;
            }
        };

        if files.is_empty() {
            self.replace_all_files(WasmProject::create(root_dir.clone().into(), HashMap::new()));
            tracing::warn!("Empty baml_src directory found: {}. See Output panel -> BAML Language Server for more details.", root_path.display());
        }

        let mut files_map = HashMap::new();
        for file in files {
            let file_path = file.display().to_string();
            if let Ok(text_document) = file_utils::convert_to_text_document(&file) {
                files_map.insert(file_path.clone(), text_document.text.clone());
            } else {
                tracing::error!("Failed to convert file {} to text document.", file_path);
            }
        }

        self.replace_all_files(WasmProject::create(root_dir.into(), files_map));
        self.update_runtime();
    }

    /// Checks the version of a given generator.
    /// (In this stub, we assume `WasmRuntime::check_version` is available as a static method.)
    pub fn check_version(
        &self,
        generator: &WasmGeneratorConfig,
        is_diagnostic: bool,
    ) -> Option<String> {
        // Call your actual WASM runtime version check here.
        None
    }

    /// Iterates over all generators and prints error messages if version mismatches are found.
    pub fn check_version_on_save(&self) -> Option<String> {
        let mut first_error_message = None;
        if let Ok(generators) = self.list_generators() {
            for gen in generators.iter() {
                if let Some(message) = self.check_version(gen, false) {
                    if first_error_message.is_none() {
                        first_error_message = Some(message.clone());
                    }
                    tracing::error!("{}", message);
                }
            }
        }
        first_error_message
    }

    /// Returns true if any generator produces TypeScript output.
    pub fn is_typescript_generator_present(&self) -> bool {
        if let Ok(generators) = self.list_generators() {
            generators
                .iter()
                .any(|g| g.output_type.to_lowercase() == "typescript")
        } else {
            false
        }
    }

    /// Retrieves diagnostics for generators.
    /// (Here we use a `HashMap<String, Vec<String>>` as a stand-in for proper diagnostic objects.)
    pub fn get_generator_diagnostics(&self) -> Option<HashMap<String, Vec<String>>> {
        if let Some(ref runtime) = self.current_runtime {
            let generators = self.list_generators().ok()?;
            let mut diagnostics = HashMap::new();
            for gen in generators {
                if let Some(message) = self.check_version(&gen, true) {
                    let diagnostic = format!("Error in {}: {}", gen.span.file_path, message);
                    diagnostics
                        .entry(gen.span.file_path.clone())
                        .or_insert_with(Vec::new)
                        .push(diagnostic);
                }
            }
            Some(diagnostics)
        } else {
            None
        }
    }

    /// Returns the VSCode generator version.
    pub fn get_vscode_generator_version(&self) -> Result<String, &'static str> {
        // Replace this stub with real configuration logic.
        Ok(String::from("1.0.0"))
    }

    /// Updates the runtime if it is not already set.
    /// Reads all files from the WASM project, builds a map from file URIs to file content,
    /// invokes diagnostics, and calls the success callback.
    pub fn update_runtime(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_runtime.is_none() {
            let fake_map: HashMap<String, String> = HashMap::new();
            let no_version_check = false;

            let js_value = serde_wasm_bindgen::to_value(&fake_map).unwrap();
            // let runtime = self.runtime();
            let runtime = self.wasm_project.runtime(js_value);
            self.current_runtime = Some(runtime.unwrap());

            let files = self.wasm_project.files();
            let mut file_map = HashMap::new();
            for file in files {
                // Expecting files to be in the format: "pathBAML_PATH_SPLTTERcontent"
                let parts: Vec<&str> = file.splitn(2, "BAML_PATH_SPLTTER").collect();
                if parts.len() == 2 {
                    file_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            let diagnostics = self
                .wasm_project
                .diagnostics(self.current_runtime.as_ref().unwrap());
            // (self.on_success)(diagnostics, file_map);
        }
        Ok(())
    }

    /// Requests diagnostics for the current project.
    pub fn request_diagnostics(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.current_runtime {
            let files = self.wasm_project.files();
            let mut file_map = HashMap::new();
            for file in files {
                let parts: Vec<&str> = file.splitn(2, "BAML_PATH_SPLTTER").collect();
                if parts.len() == 2 {
                    file_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
            let diagnostics = self.wasm_project.diagnostics(runtime);
            // (self.on_success)(diagnostics, file_map);
        }
        Ok(())
    }

    /// Retrieves a reference to the current runtime or the last successful one.
    pub fn runtime(&self) -> Result<&WasmRuntime, &str> {
        if let Some(ref rt) = self.current_runtime {
            Ok(rt)
        } else if let Some(ref rt) = self.last_successful_runtime {
            Ok(rt)
        } else {
            Err("BAML Generate failed - Project has errors.")
        }
    }

    /// Returns a map of file URIs to their content.
    pub fn files(&self) -> HashMap<String, String> {
        let files = self.wasm_project.files();
        let mut file_map = HashMap::new();
        for file in files {
            let parts: Vec<&str> = file.splitn(2, "BAML_PATH_SPLTTER").collect();
            if parts.len() == 2 {
                file_map.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        file_map
    }

    /// Replaces the current WASM project with a new one.
    pub fn replace_all_files(&mut self, project: WasmProject) {
        self.wasm_project = project;
        self.last_successful_runtime = self.current_runtime.take();
    }

    /// Records an update to a file that has not yet been saved.
    pub fn update_unsaved_file(&mut self, file_path: &str, content: String) {
        self.wasm_project.set_unsaved_file(file_path, Some(content));
        if self.current_runtime.is_some() {
            self.last_successful_runtime = self.current_runtime.take();
        }
    }

    /// Saves a file and marks the runtime as stale.
    pub fn save_file<P: AsRef<Path>, S: AsRef<str>>(&mut self, file_path: P, content: S) {
        self.wasm_project
            .save_file(file_path.as_ref().to_str().unwrap(), content.as_ref());
        if self.current_runtime.is_some() {
            self.last_successful_runtime = self.current_runtime.take();
        }
    }

    /// Reads a file and converts it into a text document.
    pub fn get_file(&self, uri: &str) -> io::Result<TextDocumentItem> {
        // Here we treat the URI as a file path.
        let path = Path::new(uri);
        file_utils::convert_to_text_document(path)
    }

    /// Updates (or inserts) the file content in the WASM project.
    pub fn upsert_file(&mut self, file_path: &str, content: Option<String>) {
        self.wasm_project.update_file(file_path, content);
        if self.current_runtime.is_some() {
            self.last_successful_runtime = self.current_runtime.take();
        }
    }

    /// Handles a definition request by attempting to look up a symbol and returning location links.
    pub fn handle_definition_request(
        &self,
        doc: &TextDocumentItem,
        definition_params: &GotoDefinitionParams,
        position: &Position,
    ) -> Vec<LocationLink> {
        let word = get_word_at_position(doc, position);
        let cleaned_word = trim_line(&word);
        if cleaned_word.is_empty() {
            return vec![];
        }
        if let Ok(runtime) = self.runtime() {
            if let Some(symbol) = runtime.search_for_symbol(&cleaned_word) {
                return vec![LocationLink {
                    target_uri: Url::from_file_path(Path::new(&symbol.uri)).unwrap(),
                    origin_selection_range: None,
                    target_range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 0,
                        },
                    },
                    target_selection_range: Range {
                        start: Position {
                            line: symbol.start_line as u32,
                            character: symbol.start_character as u32,
                        },
                        end: Position {
                            line: symbol.end_line as u32,
                            character: symbol.end_character as u32,
                        },
                    },
                }];
            }
        }
        vec![]
    }

    /// Handles a hover request.
    pub fn handle_hover_request(&self, doc: &TextDocumentItem, position: &Position) -> Hover {
        let word = get_word_at_position(doc, position);
        let cleaned_word = trim_line(&word);
        if cleaned_word.is_empty() {
            return Hover {
                contents: HoverContents::Scalar(lsp_types::MarkedString::LanguageString(
                    lsp_types::LanguageString {
                        language: "baml".to_string(),
                        value: "No definition found".to_string(),
                    },
                )),
                range: None,
            };
        }
        if let Ok(runtime) = self.runtime() {
            if let Some(symbol) = runtime.search_for_symbol(&cleaned_word) {
                let range = Range {
                    start: Position {
                        line: symbol.start_line as u32,
                        character: symbol.start_character as u32,
                    },
                    end: Position {
                        line: symbol.end_line as u32,
                        character: symbol.end_character as u32,
                    },
                };
                if let Ok(hover_doc) = self.get_file(&symbol.uri) {
                    let hover_text = hover_doc.text; //TODO: only get the text in the range.
                    return Hover {
                        contents: HoverContents::Scalar(lsp_types::MarkedString::LanguageString(
                            lsp_types::LanguageString {
                                language: "baml".to_string(),
                                value: hover_text,
                            },
                        )),
                        range: Some(range),
                    };
                }
            }
        }
        Hover {
            contents: HoverContents::Scalar(lsp_types::MarkedString::LanguageString(
                lsp_types::LanguageString {
                    language: "baml".to_string(),
                    value: "No definition found".to_string(),
                },
            )),
            range: None,
        }
    }

    /// Returns a list of functions from the WASM runtime.
    pub fn list_functions(&self) -> Result<Vec<WasmFunction>, &str> {
        if let Ok(runtime) = self.runtime() {
            Ok(runtime.list_functions())
        } else {
            Err("BAML Generate failed. Project has errors.")
        }
    }

    /// Returns a list of test cases from the WASM runtime.
    pub fn list_testcases(&self) -> Result<Vec<WasmTestCase>, &str> {
        if let Ok(runtime) = self.runtime() {
            Ok(runtime.list_testcases())
        } else {
            Err("BAML Generate failed. Project has errors.")
        }
    }

    /// Returns a list of generator configurations.
    pub fn list_generators(&self) -> Result<Vec<WasmGeneratorConfig>, &str> {
        if let Some(ref runtime) = self.current_runtime {
            Ok(runtime.list_generators())
        } else {
            Err("BAML Generate failed. Project has errors.")
        }
    }

    /// Returns the root path of this project.
    pub fn root_path(&self) -> &str {
        &self.wasm_project.root_dir_name
    }

    // Verifies whether a completion request is valid by checking for unbalanced prompt markers.
    // pub fn verify_completion_request(
    //     &self,
    //     doc: &lsp_types::TextDocumentItem,
    //     position: &lsp_types::Position,
    // ) -> bool {
    //     let text = &doc.text;
    //     let mut open_braces_count = 0;
    //     let mut close_braces_count = 0;
    //     let mut i = 0;

    //     let offset = doc.offset_at(position);
    //     let bytes = text.as_bytes();

    //     while i < offset.saturating_sub(1) {
    //         if bytes[i] == b'{' && bytes[i + 1] == b'{' {
    //             open_braces_count += 1;
    //             i += 2;
    //             continue;
    //         } else if bytes[i] == b'}' && bytes[i + 1] == b'}' {
    //             close_braces_count += 1;
    //             i += 2;
    //             continue;
    //         }
    //         i += 1;
    //     }

    //     if open_braces_count > close_braces_count {
    //         if let Ok(runtime) = self.runtime() {
    //             return runtime.check_if_in_prompt(position.line);
    //         }
    //     }
    //     false
    // }

    /// Runs generators without debouncing.
    /// (This async method simulates generator file generation and then calls one of the provided callbacks.)
    // #[cfg(feature = "async")]
    pub fn run_generators_without_debounce<F, E>(&mut self, on_success: F, on_error: E)
    where
        F: Fn(String) + Send,
        E: Fn(String) + Send,
    {
        let start = Instant::now();
        match self.wasm_project.run_generators_native(None) {
            Ok(generators) => {
                let mut generated_file_count = 0;
                for gen in generators {
                    // Process each generator and simulate file generation.
                    generated_file_count += gen.files.len();
                    // (File system operations would be performed here.)
                }
                let elapsed = start.elapsed();
                if generated_file_count > 0 {
                    on_success(format!("BAML client generated! (took {:?})", elapsed));
                }
            }
            Err(e) => {
                tracing::error!("Failed to generate BAML client: {:?}", e);
                on_error(format!("Failed to generate BAML client: {:?}", e));
            }
        }
    }

    // Runs generators with debouncing (here simply an alias).
    // #[cfg(feature = "async")]
    // pub async fn run_generators_with_debounce<F, E>(&mut self, on_success: F, on_error: E)
    // where
    //     F: Fn(String) + Send,
    //     E: Fn(String) + Send,
    // {
    //     self.run_generators_without_debounce(on_success, on_error)
    //         .await;
    // }
}
