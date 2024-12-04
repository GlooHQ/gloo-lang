#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
#![allow(clippy::derive_partial_eq_without_eq)]

//! See the docs on [ParserDatabase](./struct.ParserDatabase.html).
//!
//! ## Scope
//!
//! The ParserDatabase is tasked with gathering information about the schema. It is _connector
//! agnostic_: it gathers information and performs generic validations, leaving connector-specific
//! validations to later phases in datamodel core.
//!
//! ## Terminology
//!
//! Names:
//!
//! - _name_: the item name in the schema for datasources, generators, models, model fields,
//!   composite types, composite type fields, enums and enum variants. The `name:` argument for
//!   unique constraints, primary keys and relations.
//! - _mapped name_: the name inside an `@map()` or `@@map()` attribute of a model, field, enum or
//!   enum value. This is used to determine what the name of the Prisma schema item is in the
//!   database.
//! - _database name_: the name in the database, once both the name of the item and the mapped
//!   name have been taken into account. The logic is always the same: if a mapped name is defined,
//!   then the database name is the mapped name, otherwise it is the name of the item.
//! - _constraint name_: indexes, primary keys and defaults can have a constraint name. It can be
//!   defined with a `map:` argument or be a default, generated name if the `map:` argument is not
//!   provided. These usually require a datamodel connector to be defined.

pub mod walkers;

mod attributes;
mod coerce_expression;
mod context;
mod interner;
mod names;
mod tarjan;
mod types;

use std::collections::{HashMap, HashSet, VecDeque};

pub use coerce_expression::{coerce, coerce_array, coerce_opt};
pub use internal_baml_schema_ast::ast;
use internal_baml_schema_ast::ast::{FieldType, SchemaAst, WithName};
pub use tarjan::Tarjan;
use types::resolve_type_alias;
pub use types::{
    Attributes, ClientProperties, ContantDelayStrategy, ExponentialBackoffStrategy, PrinterType,
    PromptAst, PromptVariable, RetryPolicy, RetryPolicyStrategy, StaticType,
};
pub use walkers::TypeWalker;

use self::{context::Context, interner::StringId, types::Types};
use internal_baml_diagnostics::{DatamodelError, Diagnostics};
use names::Names;

/// ParserDatabase is a container for a Schema AST, together with information
/// gathered during schema validation. Each validation step enriches the
/// database with information that can be used to work with the schema, without
/// changing the AST. Instantiating with `ParserDatabase::new()` will perform a
/// number of validations and make sure the schema makes sense, but it cannot
/// fail. In case the schema is invalid, diagnostics will be created and the
/// resolved information will be incomplete.
///
/// Validations are carried out in the following order:
///
/// - The AST is walked a first time to resolve names: to each relevant
///   identifier, we attach an ID that can be used to reference the
///   corresponding item (model, enum, field, ...)
/// - The AST is walked a second time to resolve types. For each field and each
///   type alias, we look at the type identifier and resolve what it refers to.
/// - The AST is walked a third time to validate attributes on models and
///   fields.
/// - Global validations are then performed on the mostly validated schema.
///   Currently only index name collisions.
pub struct ParserDatabase {
    ast: ast::SchemaAst,
    interner: interner::StringInterner,
    names: Names,
    types: Types,
}

impl Default for ParserDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserDatabase {
    /// Create a new, empty ParserDatabase.
    pub fn new() -> Self {
        ParserDatabase {
            ast: ast::SchemaAst { tops: vec![] },
            interner: Default::default(),
            names: Default::default(),
            types: Default::default(),
        }
    }

    /// See the docs on [ParserDatabase](/struct.ParserDatabase.html).
    pub fn add_ast(&mut self, ast: SchemaAst) {
        self.ast.tops.extend(ast.tops);
    }

    /// See the docs on [ParserDatabase](/struct.ParserDatabase.html).
    pub fn validate(&mut self, diag: &mut Diagnostics) -> Result<(), Diagnostics> {
        let mut ctx = Context::new(
            &self.ast,
            &mut self.interner,
            &mut self.names,
            &mut self.types,
            diag,
        );

        // First pass: resolve names.
        names::resolve_names(&mut ctx);

        // Second pass: resolve top-level items and field types.
        types::resolve_types(&mut ctx);

        // Return early on type resolution errors.
        ctx.diagnostics.to_result()?;

        attributes::resolve_attributes(&mut ctx);
        ctx.diagnostics.to_result()
    }

    /// Last changes after validation.
    pub fn finalize(&mut self, diag: &mut Diagnostics) {
        self.finalize_dependencies(diag);
    }

    fn finalize_dependencies(&mut self, diag: &mut Diagnostics) {
        // Resolve type aliases.
        // Cycles are already validated so this should not stack overflow and
        // it should find the final type.
        for alias_id in self.types.type_alias_dependencies.keys() {
            let resolved = resolve_type_alias(&self.ast[*alias_id].value, &self);
            self.types.resolved_type_aliases.insert(*alias_id, resolved);
        }

        // Cycles left here after cycle validation are allowed. Basically lists
        // and maps can introduce cycles.
        self.types.structural_recursive_alias_cycles =
            Tarjan::components(&self.types.type_alias_dependencies);

        // NOTE: Class dependency cycles are already checked at
        // baml-lib/baml-core/src/validate/validation_pipeline/validations/cycle.rs
        //
        // The validation pipeline runs before this code. Check
        // baml-lib/baml-core/src/lib.rs
        //
        // Here we'll just rebuild the cycles because the validation pipeline
        // does not consider optional dependencies as part of the graph to allow
        // finite rucursive types to pass the validation. But we need the cycles
        // in order to render the LLM prompt correctly.
        //
        // TODO: Check if it's possible to build all the cycles considering
        // optional dependencies as part of the graph but detecting such
        // cycles with finite recursion during validation. That would optimize
        // away one of the calls to the Tarjan's algorithm, which is linear,
        // O(|V| + |E|), but still, if we can avoid the second call that would
        // be great. Additionally, refactor `class_dependencies` to be the same
        // type as the one expected by Tarjan::components, IDs that point to IDs
        // instead of strings (class names). That requires less conversions when
        // working with the graph. Once the work is done, IDs can be converted
        // to names where needed.
        let mut resolved_dependency_graph = HashMap::new();

        for (id, deps) in self.types.class_dependencies.iter() {
            let mut resolved_deps = HashSet::new();

            for dep in deps {
                match self.find_type_by_str(dep) {
                    Some(TypeWalker::Class(cls)) => {
                        resolved_deps.insert(cls.id);
                    }
                    Some(TypeWalker::Enum(_)) => {}
                    // Gotta resolve type aliases.
                    Some(TypeWalker::TypeAlias(alias)) => {
                        resolved_deps.extend(alias.resolved().flat_idns().iter().map(|ident| {
                            match self.find_type_by_str(ident.name()) {
                                Some(TypeWalker::Class(cls)) => cls.id,
                                Some(TypeWalker::Enum(_)) => {
                                    panic!("Enums are not allowed in type aliases")
                                }
                                Some(TypeWalker::TypeAlias(alias)) => {
                                    panic!("Alias should be resolved at this point")
                                }
                                None => panic!("Unknown class `{dep}`"),
                            }
                        }))
                    }
                    None => panic!("Unknown class `{dep}`"),
                }
            }

            resolved_dependency_graph.insert(*id, resolved_deps);
        }

        // Find the cycles and inject them into parser DB. This will then be
        // passed into the IR and then into the Jinja output format.
        self.types.finite_recursive_cycles = Tarjan::components(&resolved_dependency_graph);

        // Fully resolve function dependencies.
        let extends = self
            .types
            .function
            .iter()
            .map(|(&id, func)| {
                let (input, output) = &func.dependencies;
                let input_deps = self.collect_dependency_tree(input);
                let output_deps = self.collect_dependency_tree(output);

                (id, (input_deps, output_deps))
            })
            .collect::<Vec<_>>();

        for (id, (input, output)) in extends {
            let val = self.types.function.get_mut(&id).unwrap();
            val.dependencies.0.extend(input);
            val.dependencies.1.extend(output);
        }
    }

    /// Resolve the entire tree of dependencies for functions.
    ///
    /// Initial passes through the AST can only resolve one level of
    /// dependencies for functions. This method will go through that first level
    /// and collect all the dependencies of the dependencies.
    fn collect_dependency_tree(&self, deps: &HashSet<String>) -> HashSet<String> {
        let mut collected_deps = HashSet::new();
        let mut stack = Vec::from_iter(deps.iter().map(|dep| dep.as_str()));

        while let Some(dep) = stack.pop() {
            match self.find_type_by_str(dep) {
                // Add all the dependencies of the class.
                Some(TypeWalker::Class(walker)) => {
                    for nested_dep in walker.dependencies() {
                        if collected_deps.insert(nested_dep.to_owned()) {
                            // Recurse if not already visited.
                            stack.push(nested_dep);
                        }
                    }
                }

                // For aliases just get the resolved identifiers and
                // push them into the stack. If we find resolved classes we'll
                // add their dependencies as well. Note that this is not
                // "recursive" per se because type aliases can never "resolve"
                // to other type aliases.
                Some(TypeWalker::TypeAlias(walker)) => {
                    stack.extend(walker.resolved().flat_idns().iter().map(|ident| {
                        // Add the resolved name itself to the deps.
                        collected_deps.insert(ident.name().to_owned());
                        // Push the resolved name into the stack in case
                        // it's a class, we'll have to add its deps as
                        // well.
                        ident.name()
                    }))
                }

                // Skip enums.
                Some(TypeWalker::Enum(_)) => {}

                // This should not happen.
                _ => panic!("Unknown class `{dep}`"),
            }
        }

        collected_deps
    }

    /// The parsed AST.
    pub fn ast(&self) -> &ast::SchemaAst {
        &self.ast
    }

    /// Returns the graph of type aliases.
    ///
    /// Each vertex is a type alias and each edge is a reference to another type
    /// alias.
    pub fn type_alias_dependencies(&self) -> &HashMap<ast::TypeAliasId, HashSet<ast::TypeAliasId>> {
        &self.types.type_alias_dependencies
    }

    /// The total number of enums in the schema. This is O(1).
    pub fn enums_count(&self) -> usize {
        self.types.enum_attributes.len()
    }
}

impl std::fmt::Debug for ParserDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParserDatabase { ... }")
    }
}

impl std::ops::Index<StringId> for ParserDatabase {
    type Output = str;

    fn index(&self, index: StringId) -> &Self::Output {
        self.interner.get(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use ast::FieldArity;
    use baml_types::TypeValue;
    use internal_baml_diagnostics::{Diagnostics, SourceFile};
    use internal_baml_schema_ast::parse_schema;

    fn parse(baml: &'static str) -> Result<ParserDatabase, Diagnostics> {
        let mut db = ParserDatabase::new();
        let source = SourceFile::new_static(PathBuf::from("test.baml"), baml);
        let (ast, mut diag) = parse_schema(source.path_buf(), &source)?;

        db.add_ast(ast);
        db.validate(&mut diag)?;
        db.finalize(&mut diag);

        diag.to_result()?;

        Ok(db)
    }

    fn assert_finite_cycles(baml: &'static str, expected: &[&[&str]]) -> Result<(), Diagnostics> {
        let db = parse(baml)?;

        assert_eq!(
            db.finite_recursive_cycles()
                .iter()
                .map(|ids| Vec::from_iter(ids.iter().map(|id| db.ast()[*id].name.to_string())))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|cycle| Vec::from_iter(cycle.iter().map(ToString::to_string)))
                .collect::<Vec<_>>()
        );

        Ok(())
    }

    fn assert_structural_alias_cycles(
        baml: &'static str,
        expected: &[&[&str]],
    ) -> Result<(), Diagnostics> {
        let db = parse(baml)?;

        assert_eq!(
            db.structural_recursive_alias_cycles()
                .iter()
                .map(|ids| Vec::from_iter(ids.iter().map(|id| db.ast()[*id].name().to_string())))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|cycle| Vec::from_iter(cycle.iter().map(ToString::to_string)))
                .collect::<Vec<_>>()
        );

        Ok(())
    }

    #[test]
    fn find_simple_recursive_class() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class Node {
                    data int
                    next Node?
                }

                class LinkedList {
                    head Node?
                    len int
                }
            "#,
            &[&["Node"]],
        )
    }

    #[test]
    fn find_mutually_recursive_classes() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class Tree {
                    data int
                    children Forest
                }

                class Forest {
                    trees Tree[]
                }

                class A {
                    b B
                }

                class B {
                    a A?
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["Tree", "Forest"], &["A", "B"]],
        )
    }

    #[test]
    fn find_long_cycles() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    b B
                }

                class B {
                    c C
                }

                class C {
                    d D
                }

                class D {
                    a A?
                }

                class One {
                    two Two
                }

                class Two {
                    three Three
                }

                class Three {
                    one One?
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A", "B", "C", "D"], &["One", "Two", "Three"]],
        )
    }

    #[test]
    fn find_interconnected_long_cycles() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    b B
                }

                class B {
                    c C
                }

                class C {
                    d D
                }

                class D {
                    a A?
                    one One
                }

                class One {
                    two Two
                }

                class Two {
                    three Three
                }

                class Three {
                    one One?
                    A A
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A", "B", "C", "D", "One", "Two", "Three"]],
        )
    }

    #[test]
    fn find_simple_union_cycle() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    recursion int | string | A
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A"]],
        )
    }

    #[test]
    fn find_nested_union_cycle() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    recursion int | string | (Other | A)
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A"]],
        )
    }

    #[test]
    fn find_mutually_recursive_unions() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    recursion int | string | B
                }

                class B {
                    recursion int | string | A
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A", "B"]],
        )
    }

    #[test]
    fn find_mutually_recursive_nested_unions() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class A {
                    recursion int | string | (bool | B)
                }

                class B {
                    recursion int | string | (bool | A)
                }

                class Other {
                    dummy int
                }
            "#,
            &[&["A", "B"]],
        )
    }

    #[test]
    fn find_self_referential_map() -> Result<(), Diagnostics> {
        assert_finite_cycles(
            r#"
                class RecMap {
                    recursion map<string, RecMap>
                }
            "#,
            &[&["RecMap"]],
        )
    }

    #[test]
    fn resolve_simple_alias() -> Result<(), Diagnostics> {
        let db = parse("type Number = int")?;

        assert!(matches!(
            db.resolved_type_alias_by_name("Number").unwrap(),
            FieldType::Primitive(FieldArity::Required, TypeValue::Int, _, _)
        ));

        Ok(())
    }

    #[test]
    fn resolve_multiple_levels_of_aliases() -> Result<(), Diagnostics> {
        #[rustfmt::skip]
        let db = parse(r#"
            type One = string
            type Two = One
            type Three = Two
            type Four = Three
        "#)?;

        assert!(matches!(
            db.resolved_type_alias_by_name("Four").unwrap(),
            FieldType::Primitive(FieldArity::Required, TypeValue::String, _, _)
        ));

        Ok(())
    }

    #[test]
    fn sync_alias_arity() -> Result<(), Diagnostics> {
        #[rustfmt::skip]
        let db = parse(r#"
            type Required = float
            type Optional = Required?
        "#)?;

        assert!(matches!(
            db.resolved_type_alias_by_name("Optional").unwrap(),
            FieldType::Primitive(FieldArity::Optional, TypeValue::Float, _, _)
        ));

        Ok(())
    }

    #[test]
    fn find_basic_map_structural_cycle() -> Result<(), Diagnostics> {
        assert_structural_alias_cycles(
            "type RecursiveMap = map<string, RecursiveMap>",
            &[&["RecursiveMap"]],
        )
    }

    #[test]
    fn find_basic_list_structural_cycle() -> Result<(), Diagnostics> {
        assert_structural_alias_cycles("type A = A[]", &[&["A"]])
    }

    #[test]
    fn find_long_list_structural_cycle() -> Result<(), Diagnostics> {
        assert_structural_alias_cycles(
            r#"
                type A = B
                type B = C
                type C = A[]
            "#,
            &[&["A", "B", "C"]],
        )
    }

    #[test]
    fn find_intricate_structural_cycle() -> Result<(), Diagnostics> {
        assert_structural_alias_cycles(
            r#"
                type JsonValue = string | int | float | bool | null | JsonArray | JsonObject
                type JsonArray = JsonValue[]
                type JsonObject = map<string, JsonValue>
            "#,
            &[&["JsonValue", "JsonArray", "JsonObject"]],
        )
    }
}
