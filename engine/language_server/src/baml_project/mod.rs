use serde::{Serialize, Deserialize};
use internal_baml_diagnostics::{DatamodelError, Diagnostics};
use internal_baml_codegen::GenerateOutput;
use baml_types::BamlValue;
use baml_types::{TypeValue, BamlMediaType};
use baml_runtime::{
    // internal::llm_client::LLMResponse,
    BamlRuntime, DiagnosticsError, IRHelper,
    // RenderedPrompt,
    // runtime::InternalBamlRuntime
};

use baml_schema_build::runtime_wasm::{
    SymbolLocation, WasmDiagnosticError,
    // WasmError,
    WasmFunction, WasmGeneratorConfig, WasmParam, WasmParentFunction, WasmSpan, WasmTestCase
};
use lsp_types::{GotoDefinitionParams, Url};
use lsp_types::{
    Hover, HoverContents,
    // HoverParams,
    LocationLink, Position, Range, TextDocumentItem,
};
use position_utils::get_word_at_position;
// use rustc_hash::FxHashSet;
use std::collections::HashMap;
use std::io;
use std::path::Path;
// use std::sync::Arc;
use std::time::Instant;

mod file_utils;
pub mod metadata;
mod position_utils;
pub mod watch;

// pub struct WasmGeneratorOutput {
//     pub output_dir: String,
//     pub output_dir_relative_to_baml_src: String,
//     pub files: Vec<WasmGeneratedFile>,
// }

#[derive(Clone)]
pub struct WasmGeneratedFile {
    pub path_in_output_dir: String,
    pub contents: String,
}

// impl Into<WasmGeneratorOutput> for GenerateOutput {
//     fn into(self) -> WasmGeneratorOutput {
//         WasmGeneratorOutput {
//             output_dir: self.output_dir_full.to_string_lossy().to_string(),
//             output_dir_relative_to_baml_src: self
//                 .output_dir_shorthand
//                 .to_string_lossy()
//                 .to_string(),
//             files: self
//                 .files
//                 .into_iter()
//                 .map(|(path, contents)| WasmGeneratedFile {
//                     path_in_output_dir: path.to_string_lossy().to_string(),
//                     contents,
//                 })
//                 .collect(),
//         }
//     }
// }

// pub struct Project {
//     /// The files that are open in the project.
//     ///
//     /// Setting the open files to a non-`None` value changes `check` to only check the
//     /// open files rather than all files in the project.
//     open_fileset: Option<Arc<FxHashSet<PathBuf>>>,

//     /// The first-party files of this project.
//     file_set: Option<Arc<FxHashSet<PathBuf>>>,
//     // The metadata describing the project, including the unresolved options.
//     // pub metadata: ProjectMetadata,
// }

// --- Supporting types for definition/hover handling ---

// #[derive(Debug)]
// pub struct Position {
//     pub line: usize,
//     pub character: usize,
// }

// #[derive(Debug)]
// pub struct Range {
//     pub start: Position,
//     pub end: Position,
// }

// #[derive(Debug)]
// pub struct LocationLink {
//     pub target_uri: String,
//     pub target_range: Range,
//     pub target_selection_range: Range,
// }

// #[derive(Debug)]
// pub struct HoverContent {
//     pub language: String,
//     pub value: String,
// }

// #[derive(Debug)]
// pub struct Hover {
//     pub contents: Vec<HoverContent>,
// }

// A stub type for the symbol match that the runtime returns when looking up a symbol.
// #[derive(Debug)]
// pub struct SymbolMatch {
//     pub uri: String,
//     pub start_line: usize,
//     pub start_character: usize,
//     pub end_line: usize,
//     pub end_character: usize,
// }

// --- Helper functions for working with text documents ---

/// Trims a given string by removing non-alphanumeric characters (besides underscores and periods).
pub fn trim_line(s: &str) -> String {
    s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
        .to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BamlProject {
    pub root_dir_name: String,
    // This is the version of the file on disk
    pub files: HashMap<String, String>,
    // This is the version of the file that is currently being edited
    // (unsaved changes)
    pub unsaved_files: HashMap<String, String>,
}

impl BamlProject {

    pub fn run_generators_native(
        &self,
        no_version_check: Option<bool>,
    ) -> Result<Vec<GenerateOutput>, anyhow::Error> {
        Err(anyhow::anyhow!(
            "This function is not available in the wasm target."
        ))
    }

    pub fn set_unsaved_file(&mut self, name: &str, content: Option<String>) {
        if let Some(content) = content {
            self.unsaved_files.insert(name.to_string(), content);
        } else {
            self.unsaved_files.remove(name);
        }
    }
    pub fn save_file(&mut self, name: &str, content: &str) {
        self.files.insert(name.to_string(), content.to_string());
        self.unsaved_files.remove(name);
    }

    pub fn update_file(&mut self, name: &str, content: Option<String>) {
        if let Some(content) = content {
            self.files.insert(name.to_string(), content);
        } else {
            self.files.remove(name);
        }
    }


    pub fn runtime(&self, env_vars: HashMap<String, String>) -> Result<BamlRuntime, Diagnostics> {
        let mut hm = self.files.iter().collect::<HashMap<_, _>>();
        hm.extend(self.unsaved_files.iter());

        BamlRuntime::from_file_content(&self.root_dir_name, &hm, env_vars)
            .map_err(|e| match e.downcast::<DiagnosticsError>() {
                Ok(e) => {
                    e
                }
                Err(e) => {
                    log::debug!("Error: {:#?}", e);
                    todo!()
                }
            })
    }

    pub fn files(&self) -> Vec<String> {
        let mut saved_files = self.files.clone();
        self.unsaved_files.iter().for_each(|(k, v)| {
            saved_files.insert(k.clone(), v.clone());
        });
        let formatted_files = saved_files
            .iter()
            .map(|(k, v)| format!("{}BAML_PATH_SPLTTER{}", k, v))
            .collect::<Vec<String>>();
        formatted_files
    }


    pub fn diagnostics(&self, rt: &BamlRuntime) -> Diagnostics {
        let mut hm = self.files.iter().collect::<HashMap<_, _>>();
        hm.extend(self.unsaved_files.iter());

        rt.inner.diagnostics.clone()
    }
}

trait BamlRuntimeExt {
    fn list_testcases(&self) -> Vec<WasmTestCase>;

    fn get_testcase_from_position(
        &self,
        parent_function: WasmFunction,
        cursor_idx: usize,
    ) -> Option<WasmTestCase>;

    fn get_function_of_testcase(
        &self,
        file_name: &str,
        cursor_idx: usize,
    ) -> Option<WasmParentFunction>;

    fn search_for_symbol(&self, symbol: &str) -> Option<SymbolLocation> ;
    fn list_functions(&self) -> Vec<WasmFunction>;
    fn list_generators(&self) -> Vec<WasmGeneratorConfig>;
}

impl BamlRuntimeExt for BamlRuntime {

    fn list_generators(&self) -> Vec<WasmGeneratorConfig> {
        self
            .codegen_generators()
            .map(|generator| WasmGeneratorConfig {
                output_type: generator.output_type.clone().to_string(),
                version: generator.version.clone(),
                span: WasmSpan {
                    file_path: generator.span.file.path().to_string(),
                    start: generator.span.start,
                    end: generator.span.end,
                    start_line: generator.span.line_and_column().0 .0,
                    end_line: generator.span.line_and_column().1 .0,
                },
            })
            .collect()
    }

    fn list_functions(&self) -> Vec<WasmFunction> {
        let ctx = &self
            .create_ctx_manager(BamlValue::String("wasm".to_string()), None);
        let ctx = ctx.create_ctx_with_default();
        let ctx = ctx.eval_ctx(false);

        self
            .inner
            .ir
            .walk_functions()
            .map(|f| {
                let snippet = format!(
                    r#"test TestName {{
  functions [{name}]
  args {{
{args}
  }}
}}
"#,
                    name = f.name(),
                    args = f
                        .inputs()
                        .iter()
                        .map(|(k, t)| get_dummy_field(2, k, t))
                        .filter_map(|x| x) // Add this line to filter out None values
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let wasm_span = match f.span() {
                    Some(span) => span.into(),
                    None => WasmSpan::default(),
                };

                WasmFunction {
                    name: f.name().to_string(),
                    span: wasm_span,
                    signature: {
                        let inputs = f
                            .inputs()
                            .iter()
                            .map(|(k, t)| get_dummy_field(2, k, t))
                            .filter_map(|x| x) // Add this line to filter out None values
                            .collect::<Vec<_>>()
                            .join(",");

                        format!("({}) -> {}", inputs, f.output().to_string())
                    },
                    test_snippet: snippet,
                    test_cases: f
                        .walk_tests()
                        .map(|tc| {
                            let params = match tc.test_case_params(&ctx) {
                                Ok(params) => Ok(params
                                    .iter()
                                    .map(|(k, v)| {
                                        let as_str = match v {
                                            Ok(v) => match serde_json::to_string(v) {
                                                Ok(s) => Ok(s),
                                                Err(e) => Err(e.to_string()),
                                            },
                                            Err(e) => Err(e.to_string()),
                                        };

                                        let (value, error) = match as_str {
                                            Ok(s) => (Some(s), None),
                                            Err(e) => (None, Some(e)),
                                        };

                                        WasmParam {
                                            name: k.to_string(),
                                            value,
                                            error,
                                        }
                                    })
                                    .collect()),
                                Err(e) => Err(e.to_string()),
                            };

                            let (mut params, error) = match params {
                                Ok(p) => (p, None),
                                Err(e) => (Vec::new(), Some(e)),
                            };

                            // Any missing params should be set to an error
                            f.inputs().iter().for_each(|(param_name, t)| {
                                if !params.iter().any(|p| p.name == *param_name) && !t.is_optional()
                                {
                                    params.insert(
                                        0,
                                        WasmParam {
                                            name: param_name.to_string(),
                                            value: None,
                                            error: Some("Missing parameter".to_string()),
                                        },
                                    );
                                }
                            });

                            let wasm_span = match tc.span() {
                                Some(span) => span.into(),
                                None => WasmSpan::default(),
                            };

                            WasmTestCase {
                                name: tc.test_case().name.clone(),
                                inputs: params,
                                error,
                                span: wasm_span,
                                parent_functions: tc
                                    .test_case()
                                    .functions
                                    .iter()
                                    .map(|f| {
                                        let (start, end) = f
                                            .attributes
                                            .span
                                            .as_ref()
                                            .map_or((0, 0), |f| (f.start, f.end));
                                        WasmParentFunction {
                                            start,
                                            end,
                                            name: f.elem.name().to_string(),
                                        }
                                    })
                                    .collect(),
                            }
                        })
                        .collect(),
                }
            })
            .collect()
    }
    fn search_for_symbol(&self, symbol: &str) -> Option<SymbolLocation> {
        let runtime = self.inner.ir.clone();

        if let Ok(walker) = runtime.find_enum(symbol) {
            let elem = walker.span().unwrap();

            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }
        if let Ok(walker) = runtime.find_class(symbol) {
            let elem = walker.span().unwrap();

            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }
        if let Ok(walker) = runtime.find_type_alias(symbol) {
            let elem = walker.span().unwrap();

            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }

        if let Ok(walker) = runtime.find_function(symbol) {
            let elem = walker.span().unwrap();

            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }

        if let Ok(walker) = runtime.find_client(symbol) {
            let elem = walker.span().unwrap();

            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();

            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }

        if let Ok(walker) = runtime.find_retry_policy(symbol) {
            let elem = walker.span().unwrap();

            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }

        if let Ok(walker) = runtime.find_template_string(symbol) {
            let elem = walker.span().unwrap();
            let _uri_str = elem.file.path().to_string(); // Store the String in a variable
            let ((s_line, s_character), (e_line, e_character)) = elem.line_and_column();
            return Some(SymbolLocation {
                uri: elem.file.path().to_string(), // Use the variable here
                start_line: s_line,
                start_character: s_character,
                end_line: e_line,
                end_character: e_character,
            });
        }

        None
    }
    fn list_testcases(&self) -> Vec<WasmTestCase> {
        let ctx = self
            .create_ctx_manager(BamlValue::String("wasm".to_string()), None);

        let ctx = ctx.create_ctx_with_default();
        let ctx = ctx.eval_ctx(true);

        self.inner
            .ir
            .walk_tests()
            .map(|tc| {
                let params = match tc.test_case_params(&ctx) {
                    Ok(params) => Ok(params
                        .iter()
                        .map(|(k, v)| {
                            let as_str = match v {
                                Ok(v) => match serde_json::to_string(v) {
                                    Ok(s) => Ok(s),
                                    Err(e) => Err(e.to_string()),
                                },
                                Err(e) => Err(e.to_string()),
                            };

                            let (value, error) = match as_str {
                                Ok(s) => (Some(s), None),
                                Err(e) => (None, Some(e)),
                            };

                            WasmParam {
                                name: k.to_string(),
                                value,
                                error,
                            }
                        })
                        .collect()),
                    Err(e) => Err(e.to_string()),
                };

                let (mut params, error) = match params {
                    Ok(p) => (p, None),
                    Err(e) => (Vec::new(), Some(e)),
                };
                // Any missing params should be set to an error
                // Any missing params should be set to an error
                tc.function().inputs().iter().for_each(|func_params| {
                    let (param_name, t) = func_params;
                    if !params.iter().any(|p| p.name == *param_name) && !t.is_optional() {
                        params.push(WasmParam {
                            name: param_name.to_string(),
                            value: None,
                            error: Some("Missing parameter".to_string()),
                        });
                    }
                });
                let wasm_span = match tc.span() {
                    Some(span) => span.into(),
                    None => WasmSpan::default(),
                };

                WasmTestCase {
                    name: tc.test_case().name.clone(),
                    inputs: params,
                    error,
                    span: wasm_span,
                    parent_functions: tc
                        .test_case()
                        .functions
                        .iter()
                        .map(|f| {
                            let (start, end) = f
                                .attributes
                                .span
                                .as_ref()
                                .map_or((0, 0), |f| (f.start, f.end));
                            WasmParentFunction {
                                start,
                                end,
                                name: f.elem.name().to_string(),
                            }
                        })
                        .collect(),
                }
            })
            .collect()
    }

    fn get_testcase_from_position(
        &self,
        parent_function: WasmFunction,
        cursor_idx: usize,
    ) -> Option<WasmTestCase> {
        let testcases = parent_function.test_cases;
        for testcase in testcases {
            let span = testcase.clone().span;

            if span.file_path.as_str() == (parent_function.span.file_path)
                && ((span.start + 1)..=(span.end + 1)).contains(&cursor_idx)
            {
                return Some(testcase);
            }
        }
        None
    }

    fn get_function_of_testcase(
        &self,
        file_name: &str,
        cursor_idx: usize,
    ) -> Option<WasmParentFunction> {
        let testcases = self.list_testcases();

        for tc in testcases {
            let span = tc.span;
            if span.file_path.as_str().ends_with(file_name)
                && ((span.start + 1)..=(span.end + 1)).contains(&cursor_idx)
            {
                let first_function = tc
                    .parent_functions
                    .iter()
                    .find(|f| f.start <= cursor_idx && cursor_idx <= f.end)
                    .cloned();

                return first_function;
            }
        }
        None
    }
}

/// The Project struct wraps a WASM project, its runtime, and exposes methods for file updates,
/// diagnostics, symbol lookup, and code generation.
#[derive(Clone)]
pub struct Project {
    pub baml_project: BamlProject,
    // A callback invoked when a runtime update succeeds (passing diagnostics and a file map).
    // on_success: Box<dyn Fn(WasmDiagnosticError, HashMap<String, String>)>,
    pub current_runtime: Option<BamlRuntime>,
    pub last_successful_runtime: Option<BamlRuntime>,
}

impl Project {
    /// Creates a new `Project` instance.
    pub fn new(
        baml_project: BamlProject,
        // on_success: F
    ) -> Self
    // where
    //     F: Fn(WasmDiagnosticError, HashMap<String, String>) + 'static,
    {
        Self {
            baml_project,
            // on_success: Box::new(on_success),
            current_runtime: None,
            last_successful_runtime: None,
        }
    }

    /// Checks the version of a given generator.
    /// (In this stub, we assume `WasmRuntime::check_version` is available as a static method.)
    pub fn check_version(
        &self,
        generator: &WasmGeneratorConfig,
        _is_diagnostic: bool,
    ) -> Option<String> {
        Some(generator.version.clone())
        // Call your actual WASM runtime version check here.
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
                    eprintln!("{}", message);
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
            let fake_env_vars: HashMap<String, String> = HashMap::new();
            let no_version_check = false;

            // let runtime = self.runtime();
            let runtime = self.baml_project.runtime(fake_env_vars);
            self.current_runtime = Some(runtime.unwrap());

            let files = self.baml_project.files();
            let mut file_map = HashMap::new();
            for file in files {
                // Expecting files to be in the format: "pathBAML_PATH_SPLTTERcontent"
                let parts: Vec<&str> = file.splitn(2, "BAML_PATH_SPLTTER").collect();
                if parts.len() == 2 {
                    file_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            let diagnostics = self
                .baml_project
                .diagnostics(self.current_runtime.as_ref().unwrap());
            // (self.on_success)(diagnostics, file_map);
            todo!()
        }
        Ok(())
    }

    /// Requests diagnostics for the current project.
    pub fn request_diagnostics(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.current_runtime {
            let files = self.baml_project.files();
            let mut file_map = HashMap::new();
            for file in files {
                let parts: Vec<&str> = file.splitn(2, "BAML_PATH_SPLTTER").collect();
                if parts.len() == 2 {
                    file_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
            let diagnostics = self.baml_project.diagnostics(runtime);
            // (self.on_success)(diagnostics, file_map);
            todo!()
        }
        Ok(())
    }

    /// Retrieves a reference to the current runtime or the last successful one.
    pub fn runtime(&self) -> Result<&BamlRuntime, &str> {
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
        let files = self.baml_project.files();
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
    pub fn replace_all_files(&mut self, project: BamlProject) {
        self.baml_project = project;
        self.last_successful_runtime = self.current_runtime.take();
    }

    /// Records an update to a file that has not yet been saved.
    pub fn update_unsaved_file(&mut self, file_path: &str, content: String) {
        self.baml_project.set_unsaved_file(file_path, Some(content));
        if self.current_runtime.is_some() {
            self.last_successful_runtime = self.current_runtime.take();
        }
    }

    /// Saves a file and marks the runtime as stale.
    pub fn save_file<P: AsRef<Path>, S: AsRef<str>>(&mut self, file_path: P, content: S) {
        self.baml_project
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
        self.baml_project.update_file(file_path, content);
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
        &self.baml_project.root_dir_name
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
        match self.baml_project.run_generators_native(None) {
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
                eprintln!("Failed to generate BAML client: {:?}", e);
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

fn get_dummy_value(
    indent: usize,
    allow_multiline: bool,
    t: &baml_runtime::FieldType,
) -> Option<String> {
    let indent_str = "  ".repeat(indent);
    match t {
        baml_runtime::FieldType::Primitive(t) => {
            let dummy = match t {
                TypeValue::String => {
                    if allow_multiline {
                        format!(
                            "#\"\n{indent1}hello world\n{indent_str}\"#",
                            indent1 = "  ".repeat(indent + 1)
                        )
                    } else {
                        "\"a_string\"".to_string()
                    }
                }
                TypeValue::Int => "123".to_string(),
                TypeValue::Float => "0.5".to_string(),
                TypeValue::Bool => "true".to_string(),
                TypeValue::Null => "null".to_string(),
                TypeValue::Media(BamlMediaType::Image) => {
                    "{ url \"https://imgs.xkcd.com/comics/standards.png\" }".to_string()
                }
                TypeValue::Media(BamlMediaType::Audio) => {
                    "{ url \"https://actions.google.com/sounds/v1/emergency/beeper_emergency_call.ogg\" }".to_string()
                }
            };

            Some(dummy)
        }
        baml_runtime::FieldType::Literal(_) => None,
        baml_runtime::FieldType::Enum(_) => None,
        baml_runtime::FieldType::Class(_) => None,
        baml_runtime::FieldType::RecursiveTypeAlias(_) => None,
        baml_runtime::FieldType::List(item) => {
            let dummy = get_dummy_value(indent + 1, allow_multiline, item);
            // Repeat it 2 times
            match dummy {
                Some(dummy) => {
                    if allow_multiline {
                        Some(format!(
                            "[\n{indent1}{dummy},\n{indent1}{dummy}\n{indent_str}]",
                            dummy = dummy,
                            indent1 = "  ".repeat(indent + 1)
                        ))
                    } else {
                        Some(format!("[{}, {}]", dummy, dummy))
                    }
                }
                _ => None,
            }
        }
        baml_runtime::FieldType::Map(k, v) => {
            let dummy_k = get_dummy_value(indent, false, k);
            let dummy_v = get_dummy_value(indent + 1, allow_multiline, v);
            match (dummy_k, dummy_v) {
                (Some(k), Some(v)) => {
                    if allow_multiline {
                        Some(format!(
                            r#"{{
{indent1}{k} {v}
{indent_str}}}"#,
                            indent1 = "  ".repeat(indent + 1),
                        ))
                    } else {
                        Some(format!("{{ {k} {v} }}"))
                    }
                }
                _ => None,
            }
        }
        baml_runtime::FieldType::Union(fields) => fields
            .iter()
            .filter_map(|f| get_dummy_value(indent, allow_multiline, f))
            .next(),
        baml_runtime::FieldType::Tuple(vals) => {
            let dummy = vals
                .iter()
                .filter_map(|f| get_dummy_value(0, false, f))
                .collect::<Vec<_>>()
                .join(", ");
            Some(format!("({},)", dummy))
        }
        baml_runtime::FieldType::Optional(_) => None,
        baml_runtime::FieldType::WithMetadata { base, .. } => {
            get_dummy_value(indent, allow_multiline, base)
        }
    }
}

fn get_dummy_field(indent: usize, name: &str, t: &baml_runtime::FieldType) -> Option<String> {
    let indent_str = "  ".repeat(indent);
    let dummy = get_dummy_value(indent, true, t);
    match dummy {
        Some(dummy) => Some(format!("{indent_str}{name} {dummy}")),
        _ => None,
    }
}
